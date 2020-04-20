use anyhow::Result;
use crate::BlogPost;
use crate::html::SubDom;
use crate::doc::Document;
use markup5ever_rcdom as rcdom;
use rcdom::Handle;

pub fn from_dom(post: &BlogPost, dom: &SubDom) -> Result<Document> {
    let mut state = State {
    };

    walk(&mut state, &dom.1);

    panic!()
}

struct State {
}

fn walk(state: &mut State, node: &Handle) {
    walk_children(state, node);
}

fn walk_children(state: &mut State, node: &Handle) {
    for child in node.children.borrow().iter() {
        walk(state, &child);
    }
}
