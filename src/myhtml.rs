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

#[derive(Debug)]
pub enum Error {
    NoMemory,
    Init,
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
                Err(Error::Init)
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
}
