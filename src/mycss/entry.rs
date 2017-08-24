use modest_sys::mycss as ffi;

use super::Mycss;

pub struct Entry<'a> {
    raw: *mut ffi::mycss_entry_t,
    mycss: &'a mut Mycss,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
    Init,
}

impl<'a> Entry<'a> {
    pub fn new(mycss: &'a mut Mycss) -> Result<Entry<'a>, Error> {
        let raw = unsafe { ffi::mycss_entry_create() };
        if raw.is_null() {
            Err(Error::NoMemory)
        } else {
            let obj = Entry {
                raw: raw,
                mycss: mycss,
            };

            if unsafe { ffi::mycss_entry_init(obj.mycss.raw, obj.raw) } != 0 {
                Err(Error::Init)
            } else {
                Ok(obj)
            }
        }
    }
}

impl<'a> Drop for Entry<'a> {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::mycss_entry_destroy(self.raw, true) };
        assert!(free_result.is_null());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mycss_entry_make_destroy() {
        let mut mycss = Mycss::new().unwrap();
        let _entry = Entry::new(&mut mycss).unwrap();
    }
}
