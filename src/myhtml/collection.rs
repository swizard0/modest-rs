use modest_sys::myhtml as ffi;

use super::tree::{ParsedTree, Node};
use super::super::FromRaw;

pub struct Collection<'n, 't: 'n> {
    raw: *mut ffi::myhtml_collection_t,
    tree: &'n ParsedTree<'t>,
}

impl<'n, 't> Collection<'n, 't> {
    pub fn iter<'c>(&'c self) -> CollectionIter<'c, 'n, 't> {
        CollectionIter {
            collection: self,
            index: 0,
        }
    }
}

pub struct CollectionIter<'c, 'n: 'c, 't: 'n> {
    collection: &'c Collection<'n, 't>,
    index: usize,
}

impl<'c, 'n, 't> Iterator for CollectionIter<'c, 'n, 't> {
    type Item = Node<'n, 't>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.index < (*self.collection.raw).length {
                let item_ptr = (*self.collection.raw).list.offset(self.index as isize);
                self.index += 1;
                Some(FromRaw::from_raw((*item_ptr, self.collection.tree)))
            } else {
                None
            }
        }
    }
}

impl<'n, 't> Drop for Collection<'n, 't> {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::myhtml_collection_destroy(self.raw) };
        assert!(free_result.is_null());
    }
}

impl<'n, 't> FromRaw<(*mut ffi::myhtml_collection_t, &'n ParsedTree<'t>)> for Collection<'n, 't> {
    fn from_raw(init: (*mut ffi::myhtml_collection_t, &'n ParsedTree<'t>)) -> Self {
        Collection {
            raw: init.0,
            tree: init.1,
        }
    }
}
