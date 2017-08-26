use modest_sys::myhtml as ffi;

use super::Myhtml;
use super::super::encoding;

pub struct Tree<'a> {
    raw: *mut ffi::myhtml_tree_t,
    myhtml: &'a Myhtml,
}

pub struct ParsedTree<'a> {
    raw: *mut ffi::myhtml_tree_t,
    _myhtml: &'a Myhtml,
}

pub struct Node<'n, 't: 'n> {
    raw: *mut ffi::myhtml_tree_node_t,
    tree: &'n ParsedTree<'t>,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
    Init,
    Parse,
}

impl<'htm> Tree<'htm> {
    pub fn new(myhtml: &'htm mut Myhtml) -> Result<Tree<'htm>, Error> {
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

    pub fn parse(self, html: &str, encoding: encoding::Encoding) -> Result<ParsedTree<'htm>, Error> {
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
            let parsed_tree = ParsedTree {
                raw: self.raw,
                _myhtml: self.myhtml,
            };
            ::std::mem::forget(self);
            Ok(parsed_tree)
        }
    }
}

impl<'htm> ParsedTree<'htm> {
    pub fn document<'t>(&'t self) -> Option<Node<'t, 'htm>> {
        self.maybe_make_node(unsafe { ffi::myhtml_tree_get_document(self.raw) })
    }

    pub fn node_html<'t>(&'t self) -> Option<Node<'t, 'htm>> {
        self.maybe_make_node(unsafe { ffi::myhtml_tree_get_node_html(self.raw) })
    }

    pub fn node_head<'t>(&'t self) -> Option<Node<'t, 'htm>> {
        self.maybe_make_node(unsafe { ffi::myhtml_tree_get_node_head(self.raw) })
    }

    pub fn node_body<'t>(&'t self) -> Option<Node<'t, 'htm>> {
        self.maybe_make_node(unsafe { ffi::myhtml_tree_get_node_body(self.raw) })
    }

    fn maybe_make_node<'t>(&'t self, raw_node: *mut ffi::myhtml_tree_node_t) -> Option<Node<'t, 'htm>> {
        if raw_node.is_null() {
            None
        } else {
            Some(Node {
                raw: raw_node,
                tree: self,
            })
        }
    }
}

pub type TagId = ffi::myhtml_tag_id_t;

impl<'n, 't> Node<'n, 't> {
    pub fn tag_id(&self) -> TagId {
        unsafe { ffi::myhtml_node_tag_id(self.raw) }
    }

    pub fn name(&self) -> &str {
        let tag_id = self.tag_id();
        let mut c_len = 0;
        let c_buf = unsafe { ffi::myhtml_tag_name_by_id(self.tree.raw, tag_id, &mut c_len) };
        unsafe {
            ::std::str::from_utf8_unchecked(
                ::std::slice::from_raw_parts(c_buf as *const u8, c_len))
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

impl<'a> Drop for ParsedTree<'a> {
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
        let tree = Tree::new(&mut myhtml).unwrap();
        let parsed_tree = tree.parse(sample_html(), Default::default()).unwrap();
        assert!(parsed_tree.document().is_some());
        assert!(parsed_tree.node_html().is_some());
        assert!(parsed_tree.node_head().is_some());
        assert!(parsed_tree.node_body().is_some());
    }

    #[test]
    fn tag_names() {
        let mut myhtml = Myhtml::new(Default::default(), 1, 0).unwrap();
        let tree = Tree::new(&mut myhtml).unwrap();
        let parsed_tree = tree.parse(sample_html(), Default::default()).unwrap();
        assert_eq!(parsed_tree.node_html().unwrap().name(), "html");
        assert_eq!(parsed_tree.node_head().unwrap().name(), "head");
        assert_eq!(parsed_tree.node_body().unwrap().name(), "body");
    }

}
