use std::iter;
use log::{info, warn, error, debug};
use std::io::Cursor;
use anyhow::{Result, Context, bail};
use std::default::Default;
use markup5ever_rcdom as rcdom;
use html5ever::Attribute;
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use rcdom::{RcDom, SerializableHandle, Handle, NodeData};
use std::cell::RefCell;

pub fn walk_tags(src: &str) -> Result<()> {
    let dom = build_dom(src)?;
    walk_dom(&dom.document, 0);
    Ok(())
}

pub fn extract_article_string(src: &str) -> Result<String> {
    let (dom, node) = extract_article(src)?;
    let s = serialize_dom(&node)
        .context("serializing dom")?;
    Ok(s)
}

pub type SubDom = (RcDom, Handle);

pub fn extract_article(src: &str) -> Result<SubDom> {
    let dom = build_dom(src)?;
    let node = find_article(&dom.document);
    match node {
        Some(node) => {
            Ok((dom, node))
        }
        None => {
            bail!("no article found");
        }
    }
}

pub fn extract_dom(src: &str) -> Result<SubDom> {
    let dom = build_dom(src)?;
    let node = dom.document.clone();
    Ok((dom, node))
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
    candidate.map(|c| c.node)
}

struct Candidate {
    name: String,
    node: Handle,
}

fn find_article_(dom: &Handle, candidate: &mut Option<Candidate>) {
    match &dom.data {
        NodeData::Element { name, attrs, .. } => {
            let mut is_candidate = false;
            let name = name.local.as_ref();
            let id_attr = find_id_attr(attrs);
            if name == "article" {
                is_candidate = true;
            }
            if name == "main" {
                is_candidate = true;
            }
            // As in bcantrill's pages
            if name == "div" && id_attr == Some("content".to_string()) {
                is_candidate = true;
            }

            if is_candidate {
                match candidate {
                    None => {
                        *candidate = Some(Candidate {
                            name: name.to_string(),
                            node: dom.clone(),
                        });
                    }
                    Some(ref mut candidate) => {
                        warn!("multiple article candidates");
                        if name == "article" {
                            warn!("upgrading from '{}' to 'article'", candidate.name);
                            *candidate = Candidate {
                                name: name.to_string(),
                                node: dom.clone(),
                            };
                        } else if candidate.name == "div" {
                            warn!("upgrading from 'div' to '{}'", name);
                            *candidate = Candidate {
                                name: name.to_string(),
                                node: dom.clone(),
                            };
                        } else {
                            warn!("new candidate: {}", name);
                            warn!("using old candidate");
                        }
                    }
                }
            }

            find_article_children(dom, candidate);
        }
        _ => {
            find_article_children(dom, candidate);
        }
    }
}

fn find_article_children(dom: &Handle, candidate: &mut Option<Candidate>) {
    for child in dom.children.borrow().iter() {
        find_article_(&child, candidate);
    }
}

fn find_id_attr(attrs: &RefCell<Vec<Attribute>>) -> Option<String> {
    for attr in &*attrs.borrow() {
        if &attr.name.local == "id" {
            return Some(attr.value.to_string());
        }
    }

    None
}

fn serialize_dom(dom: &Handle) -> Result<String> {
    let dom: SerializableHandle = dom.clone().into();

    let mut buf = Vec::new();

    serialize(&mut buf, &dom, Default::default())?;

    let doc = String::from_utf8(buf).context("serialized dom not utf8")?;

    Ok(doc)
}

pub fn walk_dom_fn(dom: &Handle, f: &mut impl FnMut(&Handle)) {
    f(dom);
    walk_dom_fn_children(dom, f);
}

fn walk_dom_fn_children(dom: &Handle, f: &mut impl FnMut(&Handle)) {
    for child in dom.children.borrow().iter() {
        walk_dom_fn(&child, f);
    }
}    
