use modest_sys::mycss as ffi;

pub mod entry;

pub struct Mycss {
    raw: *mut ffi::mycss_t,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
    Init,
}

impl Mycss {
    pub fn new() -> Result<Mycss, Error> {
        let raw = unsafe { ffi::mycss_create() };
        if raw.is_null() {
            Err(Error::NoMemory)
        } else {
            let obj = Mycss {
                raw: raw,
            };
            if unsafe { ffi::mycss_init(obj.raw) } != 0 {
                Err(Error::Init)
            } else {
                Ok(obj)
            }
        }
    }
}

impl Drop for Mycss {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::mycss_destroy(self.raw, true) };
        assert!(free_result.is_null());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mycss_make_destroy() {
        let _mycss = Mycss::new().unwrap();
    }
}
