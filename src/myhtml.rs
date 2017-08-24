use modest_sys::myhtml as ffi;

#[derive(Clone, Copy, Default, Debug)]
pub struct InitOptions {
    parse_mode: ParseMode,
}

#[derive(Clone, Copy, Debug)]
pub enum ParseMode {
    Default,
    Single,
    AllInOne,
    Separately,
}

pub struct Myhtml {
    raw: *mut ffi::myhtml_t,
}

pub struct Tree<'a> {
    raw: *mut ffi::myhtml_tree_t,
    myhtml: &'a mut Myhtml,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
    InitMyhtml,
    InitMyhtmlTree
}

impl Myhtml {
    pub fn new(options: InitOptions, thread_count: usize, queue_size: usize) -> Result<Myhtml, Error> {
        let raw = unsafe { ffi::myhtml_create() };
        if raw.is_null() {
            Err(Error::NoMemory)
        } else {
            let obj = Myhtml {
                raw: raw,
            };
            let opts = match options.parse_mode {
                ParseMode::Default =>
                    ffi::myhtml_options::MyHTML_OPTIONS_DEFAULT,
                ParseMode::Single =>
                    ffi::myhtml_options::MyHTML_OPTIONS_PARSE_MODE_SINGLE,
                ParseMode::AllInOne =>
                    ffi::myhtml_options::MyHTML_OPTIONS_PARSE_MODE_ALL_IN_ONE,
                ParseMode::Separately =>
                    ffi::myhtml_options::MyHTML_OPTIONS_PARSE_MODE_SEPARATELY,
            };

            if unsafe { ffi::myhtml_init(obj.raw, opts, thread_count, queue_size) } != 0 {
                Err(Error::InitMyhtml)
            } else {
                Ok(obj)
            }
        }
    }
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
                Err(Error::InitMyhtmlTree)
            } else {
                Ok(obj)
            }
        }
    }
}

impl Drop for Myhtml {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::myhtml_destroy(self.raw) };
        assert!(free_result.is_null());
    }
}

impl<'a> Drop for Tree<'a> {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::myhtml_tree_destroy(self.raw) };
        assert!(free_result.is_null());
    }
}

impl Default for ParseMode {
    fn default() -> ParseMode {
        ParseMode::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn myhtml_make_destroy() {
        let _myhtml = Myhtml::new(Default::default(), 1, 0).unwrap();
    }

    #[test]
    fn myhtml_tree_make_destroy() {
        let mut myhtml = Myhtml::new(Default::default(), 1, 0).unwrap();
        let _tree = Tree::new(&mut myhtml).unwrap();
    }
}
