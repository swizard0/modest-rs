use std::{str, slice};

use modest_sys::mycss::entry as entry_ffi;
use modest_sys::mycss::selectors as selectors_ffi;
use modest_sys::modest::finder as finder_ffi;
use modest_sys::myhtml as myhtml_ffi;

use super::Entry;
use super::super::super::{Encoding, ForeignRaw, FromRaw};
use super::super::super::modest::finder::Finder;
use super::super::super::myhtml::tree::Node;
use super::super::super::myhtml::collection::Collection;

pub struct Selectors<'e, 'css: 'e> {
    raw: *mut selectors_ffi::mycss_selectors_t,
    _entry: &'e mut Entry<'css>,
}

pub struct SelectorsList<'l, 'e: 'l, 'css: 'e> {
    raw: *mut selectors_ffi::mycss_selectors_list_t,
    selectors: &'l mut Selectors<'e, 'css>,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
    BadSelectors(Result<Vec<String>, SerializeError>),
    Find,
}

#[derive(Debug)]
pub enum SerializeError {
    NullSelectorBuffer,
    InvalidUtf8(str::Utf8Error),
}

impl<'e, 'css> Selectors<'e, 'css> {
    pub fn new(entry: &'e mut Entry<'css>) -> Selectors<'e, 'css> {
        Selectors {
            raw: unsafe { entry_ffi::mycss_entry_selectors(entry.raw) as *mut selectors_ffi::mycss_selectors },
            _entry: entry,
        }
    }

    pub fn parse<'l>(&'l mut self, data: &str, encoding: Encoding) -> Result<SelectorsList<'l, 'e, 'css>, Error> {
        let mut out_status = 0u32;
        let list = unsafe {
            selectors_ffi::mycss_selectors_parse(
                self.raw,
                ::std::mem::transmute(encoding.to_ffi()),
                data.as_ptr() as *const ::std::os::raw::c_char,
                data.len(),
                &mut out_status as *mut _)
        };
        if list.is_null() {
            Err(Error::NoMemory)
        } else if unsafe { (*list).flags as u32 } & selectors_ffi::mycss_selectors_flags::MyCSS_SELECTORS_FLAGS_SELECTOR_BAD as u32 != 0 {
            let mut maybe_selectors = Ok(Vec::new());
            extern "C" fn collect_bad_selectors(
                buffer: *const ::std::os::raw::c_char,
                size: usize,
                ctx: *mut ::std::os::raw::c_void)
                -> selectors_ffi::mystatus_t
            {
                if !ctx.is_null() {
                    let current_result: &mut Result<Vec<String>, SerializeError> =
                        unsafe { &mut *(ctx as *mut _) };
                    let mut error_triggered = None;
                    if let Ok(ref mut selectors) = *current_result {
                        if buffer.is_null() && size > 0 {
                            error_triggered = Some(SerializeError::NullSelectorBuffer);
                        } else if size == 0 {
                            // do nothing
                        } else {
                            let slice = unsafe { slice::from_raw_parts(buffer as *const u8, size) };
                            match str::from_utf8(slice) {
                                Ok(cow_string) =>
                                    selectors.push(cow_string.to_string()),
                                Err(e) =>
                                    error_triggered = Some(SerializeError::InvalidUtf8(e)),
                            }
                        }
                    }
                    if let Some(e) = error_triggered {
                        *current_result = Err(e);
                    }
                }
                0
            }

            unsafe {
                selectors_ffi::mycss_selectors_serialization_list(
                    self.raw,
                    list,
                    Some(collect_bad_selectors),
                    &mut maybe_selectors as *mut _ as *mut _);
            }
            Err(Error::BadSelectors(maybe_selectors))
        } else {
            Ok(SelectorsList {
                raw: list,
                selectors: self,
            })
        }
    }
}

impl<'l, 'e, 'css> SelectorsList<'l, 'e, 'css> {
    pub fn find<'t, 'htm>(&self, node: &Node<'t, 'htm>, finder: &mut Finder) -> Result<Collection<'t, 'htm>, Error> {
        let mut collection = ::std::ptr::null_mut();
        let status = unsafe {
            finder_ffi::modest_finder_by_selectors_list(
                finder.get_raw_mut(),
                node.get_raw() as *mut finder_ffi::myhtml_tree_node,
                self.raw as *mut finder_ffi::mycss_selectors_list,
                &mut collection)
        };
        if status == 0 {
            assert!(!collection.is_null());
            Ok(FromRaw::from_raw((collection as *mut myhtml_ffi::myhtml_collection_t, node.owner())))
        } else {
            Err(Error::Find)
        }
    }
}

impl<'l, 'e, 'css> Drop for SelectorsList<'l, 'e, 'css> {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { selectors_ffi::mycss_selectors_list_destroy(self.selectors.raw, self.raw, true) };
        assert!(free_result.is_null());
    }
}

#[cfg(test)]
mod tests {
    use super::super::Mycss;
    use super::*;

    fn selector() -> &'static str {
        "div > :nth-child(2n+1):not(:has(a))"
    }

    #[test]
    fn mycss_selectors_make_destroy() {
        let mut mycss = Mycss::new().unwrap();
        let mut entry = Entry::new(&mut mycss).unwrap();
        let _selectors = Selectors::new(&mut entry);
    }

    #[test]
    fn mycss_selectors_parse() {
        let mut mycss = Mycss::new().unwrap();
        let mut entry = Entry::new(&mut mycss).unwrap();
        let mut selectors = Selectors::new(&mut entry);
        let _selectors_list = selectors.parse(selector(), Default::default()).unwrap();
    }
}
