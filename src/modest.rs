use modest_sys::modest as ffi;

pub struct Modest {
    raw: *mut ffi::modest_t,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
}

impl Modest {
    pub fn new() -> Result<Modest, Error> {
        let raw = unsafe { ffi::modest_create() };
        if raw.is_null() {
            Err(Error::NoMemory)
        } else {
            Ok(Modest {
                raw: raw,
            })
        }
    }
}

impl Drop for Modest {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::modest_destroy(self.raw, true) };
        assert!(free_result.is_null());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_destroy() {
        let _modest = Modest::new().unwrap();
    }
}
