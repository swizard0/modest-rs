use modest_sys::mycss::{entry, selectors};

use super::Entry;

pub struct Selectors<'e, 'css: 'e> {
    raw: *mut selectors::mycss_selectors_t,
    entry: &'e mut Entry<'css>,
}

impl<'e, 'css> Selectors<'e, 'css> {
    pub fn new(entry: &'e mut Entry<'css>) -> Selectors<'e, 'css> {
        Selectors {
            raw: unsafe { entry::mycss_entry_selectors(entry.raw) as *mut selectors::mycss_selectors },
            entry: entry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Mycss;
    use super::*;

    #[test]
    fn mycss_selectors_make_destroy() {
        let mut mycss = Mycss::new().unwrap();
        let mut entry = Entry::new(&mut mycss).unwrap();
        let _selectors = Selectors::new(&mut entry);
    }
}
