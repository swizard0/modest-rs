use modest_sys::myhtml as ffi;

use super::{encoding, Myhtml};

pub struct Tree<'a> {
    raw: *mut ffi::myhtml_tree_t,
    myhtml: &'a Myhtml,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
    Init,
    Parse,
}

impl<'a> Tree<'a> {
    pub fn new(myhtml: &'a mut Myhtml) -> Result<Tree<'a>, Error> {
        let raw = unsafe { ffi::myhtml_tree_create() };
        if raw.is_null() {
            Err(Error::NoMemory)
        } else {
            let obj = Tree {
                raw: raw,
                myhtml: myhtml,
            };

            if unsafe { ffi::myhtml_tree_init(obj.raw, obj.myhtml.raw) } != 0 {
                Err(Error::Init)
            } else {
                Ok(obj)
            }
        }
    }

    pub fn parse(&mut self, html: &str, encoding: encoding::Encoding) -> Result<(), Error> {
        let status = unsafe {
            ffi::myhtml_parse(
                self.raw,
                encoding.to_ffi(),
                html.as_ptr() as *const ::std::os::raw::c_char,
                html.len())
        };
        if status != 0 {
            Err(Error::Parse)
        } else {
            Ok(())
        }
    }
}

impl<'a> Drop for Tree<'a> {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::myhtml_tree_destroy(self.raw) };
        assert!(free_result.is_null());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_html() -> &'static str {
        "<div><p id=p1><p id=p2><p id=p3><a>link</a><p id=p4><p id=p5><p id=p6></div>"
    }

    #[test]
    fn myhtml_tree_make_destroy() {
        let mut myhtml = Myhtml::new(Default::default(), 1, 0).unwrap();
        let _tree = Tree::new(&mut myhtml).unwrap();
    }

    #[test]
    fn myhtml_parse() {
        let mut myhtml = Myhtml::new(Default::default(), 1, 0).unwrap();
        let mut tree = Tree::new(&mut myhtml).unwrap();
        tree.parse(sample_html(), Default::default()).unwrap();
    }
}
