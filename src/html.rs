use std::iter;
use log::info;
use std::io::Cursor;
use anyhow::{Result, Context};
use std::default::Default;
use markup5ever_rcdom as rcdom;
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use rcdom::{RcDom, SerializableHandle, Handle, NodeData};

pub fn walk_tags(src: &str) -> Result<()> {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut cursor = Cursor::new(src);
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut cursor)
        .context("parsing html")?;

    walk_dom(&dom.document, 0);

    Ok(())
}

fn walk_dom(dom: &Handle, lvl: u32) {
    let tab: String = iter::repeat(' ').take(lvl as usize * 2).collect();
    match &dom.data {
        NodeData::Element { name, .. } => {
            let boring_tags = [
                "span", "a", "img", "meta", "link", "script", "nav",
                "form", "fieldset", "input", "sup"
            ];
            let display = !boring_tags.contains(&name.local.as_ref());
            if display {
                info!("{}<{}>", tab, name.local);
            }
            walk_children(dom, lvl);
            if display {
                info!("{}</{}>", tab, name.local);
            }
        }
        _ => {
            walk_children(dom, lvl);
        }
    }
}

fn walk_children(dom: &Handle, lvl: u32) {
    for child in dom.children.borrow().iter() {
        walk_dom(&child, lvl + 1);
    }
}
