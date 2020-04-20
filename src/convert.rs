use anyhow::Result;
use crate::BlogPost;
use crate::html::SubDom;
use crate::doc::Document;
use markup5ever_rcdom as rcdom;
use rcdom::{Handle, NodeData};
use crate::doc;

pub fn from_dom(post: &BlogPost, dom: &SubDom) -> Result<Document> {
    let mut state = State {
        blocks: Vec::new(),
        mode: Mode::ScanForBlocks,
    };

    walk(&mut state, &dom.1);

    panic!()
}

struct State {
    blocks: Vec<doc::Block>,
    mode: Mode,
}

enum Mode {
    ScanForBlocks,
    AccumulateInlines(Vec<doc::Inline>),
}

fn walk(state: &mut State, node: &Handle) {

    

    match &node.data {
        NodeData::Element { name, .. } => {
            match name.local.as_ref() {
                "p" => {
                    handle_para(state, node);
                    return;
                },
                _ => {
                }
            }
        }
        _ => {
        }
    }

    walk_children(state, node);
}

fn handle_para(state: &mut State, node: &Handle) {
    match state.mode {
        Mode::ScanForBlocks => {
            state.mode = Mode::AccumulateInlines(Vec::new());
            walk_children(state, node)
        },
        _ => {
            walk_children(state, node)
        }
    }
}

fn walk_children(state: &mut State, node: &Handle) {
    for child in node.children.borrow().iter() {
        walk(state, &child);
    }
}
