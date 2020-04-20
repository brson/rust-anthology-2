use std::mem;
use anyhow::Result;
use crate::BlogPost;
use crate::html::SubDom;
use markup5ever_rcdom as rcdom;
use rcdom::{Node as Node, NodeData};
use crate::doc;
use log::{warn, debug};

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

#[derive(Debug)]
enum Mode {
    ScanForBlocks,
    AccumulateInlines(Vec<doc::Inline>),
    AccumulateListItems(Vec<doc::ListItem>),
    AccumulateBlocks(Vec<doc::Block>),
    Placeholder,
}

fn walk(state: &mut State, node: &Node) {
    if maybe_synthesize_para(state, node) {
        return;
    }
    walk_(state, node);
}

fn walk_(state: &mut State, node: &Node) {
    match &node.data {
        NodeData::Element { name, .. } => {
            let name = name.local.as_ref();
            match name {
                "p" => {
                    handle_para(state, node);
                    return;
                },
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    handle_heading(state, node, name);
                    return;
                }
                "ol" => {
                    handle_list(state, node, doc::ListType::Ordered);
                    return;
                }
                "ul" => {
                    handle_list(state, node, doc::ListType::Unordered);
                    return;
                }
                "li" => {
                    handle_list_item(state, node);
                    return;
                }
                "blockquote" => {

                }
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

fn walk_children(state: &mut State, node: &Node) {
    for child in node.children.borrow().iter() {
        walk(state, &child);
    }
}

/// Our model requires the root, list items, and blockquotes to contain block
/// items, where HTML allows them to contain inlines directly. This detects this
/// situation and opens paragraph blocks that don't exist in the source HTML.
fn maybe_synthesize_para(state: &mut State, node: &Node) -> bool {
    let want_blocks = match state.mode {
        Mode::ScanForBlocks => true,
        Mode::AccumulateBlocks { .. } => true,
        _ => false,
    };

    if !want_blocks {
        return false;
    }

    if !is_inline_element(node) {
        return false;
    }

    handle_para_(state, node, true);

    true
}

fn is_inline_element(node: &Node) -> bool {
    match &node.data {
        NodeData::Element { name, .. } => {
            let name = name.local.as_ref();
            match name {
                "a" => {
                    true
                }
                _ => {
                    false
                }
            }
        }
        NodeData::Text { contents } => {
            let text = String::from(contents.borrow().as_ref().trim());
            !text.is_empty()
        }
        _ => {
            false
        }
    }
}

fn handle_para(state: &mut State, node: &Node) {
    handle_para_(state, node, false)
}

fn handle_para_(state: &mut State, node: &Node, current_is_first_inline: bool) {
    let old_mode = mem::replace(&mut state.mode, Mode::Placeholder);
    match old_mode {
        Mode::ScanForBlocks => {
            state.mode = Mode::AccumulateInlines(Vec::new());
            if !current_is_first_inline {
                walk_children(state, node);
            } else {
                walk_(state, node);
            }
            let mode = mem::replace(&mut state.mode, Mode::Placeholder);
            match mode {
                Mode::AccumulateInlines(inlines) => {
                    let new_para = doc::Paragraph { inlines };
                    let new_block = doc::Block::Paragraph(new_para);
                    state.blocks.push(new_block);
                    state.mode = Mode::ScanForBlocks;
                }
                e => panic!("unexpected mode {:?}", e),
            }
        },
        Mode::AccumulateBlocks(mut blocks) => {
            state.mode = Mode::AccumulateInlines(Vec::new());
            if !current_is_first_inline {
                walk_children(state, node);
            } else {
                walk_(state, node);
            }
            let mode = mem::replace(&mut state.mode, Mode::Placeholder);
            match mode {
                Mode::AccumulateInlines(inlines) => {
                    let new_para = doc::Paragraph { inlines };
                    let new_block = doc::Block::Paragraph(new_para);
                    blocks.push(new_block);
                    state.mode = Mode::AccumulateBlocks(blocks);
                }
                e => panic!("unexpected mode {:?}", e),
            }
        }
        _ => {
            assert!(!current_is_first_inline);
            //warn!("unhandled para");
            state.mode = old_mode;
            walk_children(state, node);
        }
    }
}

fn handle_heading(state: &mut State, node: &Node, htext: &str) {
    let level = match htext {
        "h1" => doc::HeadingLevel::H1,
        "h2" => doc::HeadingLevel::H2,
        "h3" => doc::HeadingLevel::H3,
        "h4" => doc::HeadingLevel::H4,
        "h5" => doc::HeadingLevel::H5,
        "h6" => doc::HeadingLevel::H6,
        _ => panic!("unexpected heading level"),
    };
    match state.mode {
        Mode::ScanForBlocks => {
            state.mode = Mode::AccumulateInlines(Vec::new());
            walk_children(state, node);
            let mode = mem::replace(&mut state.mode, Mode::ScanForBlocks);
            match mode {
                Mode::AccumulateInlines(inlines) => {
                    let new_heading = doc::Heading {
                        inlines,
                        level,
                    };
                    let new_block = doc::Block::Heading(new_heading);
                    state.blocks.push(new_block);
                }
                e => panic!("unexpected mode {:?}", e),
            }
        }
        _ => {
            //warn!("unhandled heading");
            walk_children(state, node);
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
            //warn!("unhandled text");
        }
    }
    walk_children(state, node);
}

fn handle_list(state: &mut State, node: &Node, type_: doc::ListType) {
    let old_mode = mem::replace(&mut state.mode, Mode::Placeholder);
    match old_mode {
        Mode::ScanForBlocks => {
            state.mode = Mode::AccumulateListItems(Vec::new());
            walk_children(state, node);
            let mode = mem::replace(&mut state.mode, Mode::Placeholder);
            match mode {
                Mode::AccumulateListItems(items) => {
                    let new_list = doc::List { type_, items };
                    let new_block = doc::Block::List(new_list);
                    state.blocks.push(new_block);
                    state.mode = Mode::ScanForBlocks;
                },
                e => panic!("unexpected mode {:?}", e),
            }
        }
        Mode::AccumulateBlocks(mut blocks) => {
            state.mode = Mode::AccumulateListItems(Vec::new());
            walk_children(state, node);
            let mode = mem::replace(&mut state.mode, Mode::Placeholder);
            match mode {
                Mode::AccumulateListItems(items) => {
                    let new_list = doc::List { type_, items };
                    let new_block = doc::Block::List(new_list);
                    blocks.push(new_block);
                    state.mode = Mode::AccumulateBlocks(blocks);
                },
                e => panic!("unexpected mode {:?}", e),
            }
        }
        _ => {
            //warn!("unhandled list");
            state.mode = old_mode;
            walk_children(state, node);
        }
    }
}

fn handle_list_item(state: &mut State, node: &Node) {
    let old_mode = mem::replace(&mut state.mode, Mode::Placeholder);
    match old_mode {
        Mode::AccumulateListItems(mut items) => {
            state.mode = Mode::AccumulateBlocks(Vec::new());
            walk_children(state, node);
            let mode = mem::replace(&mut state.mode, Mode::Placeholder);
            match mode {
                Mode::AccumulateBlocks(blocks) => {
                    let new_item = doc::ListItem { blocks };
                    items.push(new_item);
                    state.mode = Mode::AccumulateListItems(items);
                },
                e => panic!("unexpected mode {:?}", e),
            }
        },
        _ => {
            //warn!("unhandled list item");
            state.mode = old_mode;
            walk_children(state, node);
        }
    }
}
