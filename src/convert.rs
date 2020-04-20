use std::mem;
use anyhow::Result;
use crate::BlogPost;
use crate::html::SubDom;
use markup5ever_rcdom as rcdom;
use rcdom::{Node as Node, NodeData};
use crate::doc;

pub fn from_dom(post: &BlogPost, dom: &SubDom) -> Result<doc::Document> {
    let mut state = State {
        blocks: Vec::new(),
        mode: Mode::ScanForBlocks,
    };

    walk(&mut state, &dom.1);

    let meta = doc::Meta {
        origin_url: post.url.clone(),
    };
    let body = doc::Body {
        blocks: state.blocks,
    };
    let doc = doc::Document {
        meta, body
    };

    Ok(doc)
}

struct State {
    blocks: Vec<doc::Block>,
    mode: Mode,
}

enum Mode {
    ScanForBlocks,
    AccumulateInlines(Vec<doc::Inline>),
}

fn walk(state: &mut State, node: &Node) {
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
        NodeData::Text { contents } => {
            let text = String::from(contents.borrow().as_ref());
            handle_text(state, node, text);
            return;
        }
        _ => {
        }
    }

    walk_children(state, node);
}

fn handle_para(state: &mut State, node: &Node) {
    match state.mode {
        Mode::ScanForBlocks => {
            state.mode = Mode::AccumulateInlines(Vec::new());
            walk_children(state, node);
            let mode = mem::replace(&mut state.mode, Mode::ScanForBlocks);
            match mode {
                Mode::AccumulateInlines(inlines) => {
                    let new_para = doc::Paragraph {
                        inlines,
                    };
                    let new_block = doc::Block::Paragraph(new_para);
                    state.blocks.push(new_block);
                }
                _ => {
                    panic!("unexpected mode");
                }
            }
        },
        _ => {
        }
    }
}

fn handle_text(state: &mut State, node: &Node, text: String) {
    match state.mode {
        Mode::AccumulateInlines(ref mut inlines) => {
            let new = doc::Inline::Text(text);
            inlines.push(new);
        }
        _ => {
        }
    }
    walk_children(state, node);
}

fn walk_children(state: &mut State, node: &Node) {
    for child in node.children.borrow().iter() {
        walk(state, &child);
    }
}
