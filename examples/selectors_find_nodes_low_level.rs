extern crate modest;

use modest::{myhtml, mycss};
use modest::modest as modest_mod;

fn main() {
    let html = "<div><p id=p1><p id=p2><p id=p3><a>link</a><p id=p4><p id=p5><p id=p6></div>";
    let selector = "div > :nth-child(2n+1):not(:has(a))";

    let () = run(html, selector).unwrap();
}

#[derive(Debug)]
enum Error {
    MyhtmlCreate(myhtml::Error),
    MyhtmlTreeCreate(myhtml::tree::Error),
    MyhtmlParse(myhtml::tree::Error),
    MycssCreate(mycss::Error),
    MycssEntryCreate(mycss::entry::Error),
    MycssParse(mycss::entry::selectors::Error),
    Finder(modest_mod::finder::Error),
    NoHtmlTag,
    SelectorsListFind(mycss::entry::selectors::Error),
}

fn run(html: &str, selector: &str) -> Result<(), Error> {
    let mut myhtml = myhtml::Myhtml::new(Default::default(), 1, 0)
        .map_err(Error::MyhtmlCreate)?;
    let html_tree = myhtml::tree::Tree::new(&mut myhtml)
        .map_err(Error::MyhtmlTreeCreate)?;
    let parsed_html_tree = html_tree.parse(html, Default::default())
        .map_err(Error::MyhtmlParse)?;
    let mut mycss = mycss::Mycss::new()
        .map_err(Error::MycssCreate)?;
    let mut entry = mycss::entry::Entry::new(&mut mycss)
        .map_err(Error::MycssEntryCreate)?;
    let mut selectors = mycss::entry::selectors::Selectors::new(&mut entry);
    let selectors_list = selectors.parse(selector, Default::default())
        .map_err(Error::MycssParse)?;
    let mut finder = modest_mod::finder::Finder::new()
        .map_err(Error::Finder)?;
    let collection = selectors_list.find(&parsed_html_tree.node_html().ok_or(Error::NoHtmlTag)?, &mut finder)
        .map_err(Error::SelectorsListFind)?;
    for node in collection.iter() {
        println!("Node found: <{}>", node.name());
    }
    Ok(())
}
