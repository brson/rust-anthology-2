use std::iter;
use log::{info, warn, error};
use std::io::Cursor;
use anyhow::{Result, Context, bail};
use std::default::Default;
use markup5ever_rcdom as rcdom;
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use rcdom::{RcDom, SerializableHandle, Handle, NodeData};

pub fn walk_tags(src: &str) -> Result<()> {
    let dom = build_dom(src)?;
    walk_dom(&dom.document, 0);
    Ok(())
}

pub fn extract_article(src: &str) -> Result<()> {
    let dom = build_dom(src)?;
    let node = find_article(&dom.document);
    match node {
        Some(node) => {
            let s = serialize_dom(&node)
                .context("serializing dom")?;
            info!("{}", s);
        }
        None => {
            bail!("no article found");
        }
    }
    Ok(())
}

fn build_dom(src: &str) -> Result<RcDom> {
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

    Ok(dom)
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

fn find_article(dom: &Handle) -> Option<Handle> {
    let mut candidate = None;
    find_article_(dom, &mut candidate);
    candidate
}

fn find_article_(dom: &Handle, candidate: &mut Option<Handle>) {
    match &dom.data {
        NodeData::Element { name, .. } => {
            let mut is_candidate = false;
            if name.local.as_ref() == "article" {
                is_candidate = true;
            }

            if is_candidate {
                if candidate.is_none() {
                    *candidate = Some(dom.clone());
                } else {
                    warn!("multiple article candidates");
                    warn!("new candidate: {}", name.local);
                }
            }

            find_article_children(dom, candidate);
        }
        _ => {
            find_article_children(dom, candidate);
        }
    }
}

fn find_article_children(dom: &Handle, candidate: &mut Option<Handle>) {
    for child in dom.children.borrow().iter() {
        find_article_(&child, candidate);
    }
}    

fn serialize_dom(dom: &Handle) -> Result<String> {
    let dom: SerializableHandle = dom.clone().into();

    let mut buf = Vec::new();

    serialize(&mut buf, &dom, Default::default())?;

    let doc = String::from_utf8(buf).context("serialized dom not utf8")?;

    Ok(doc)
}
