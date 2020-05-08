use log::warn;
use anyhow::Result;
use crate::doc::*;
use markup5ever_rcdom as rcdom;
use rcdom::{Node, NodeData};
use crate::html::SubDom;
use crate::doc::{Block, HeadingLevel};

pub fn sanitize(doc: Document, post: &str) -> Document {
    let doc = maybe_add_h1(doc, post);
    doc
}

pub fn title_to_slug(s: String) -> String {
    string_to_slug(s)
}

pub fn name_to_slug(s: String) -> String {
    string_to_slug(s)
}

fn string_to_slug(s: String) -> String {
    let s = s.to_lowercase();
    let s = convert_space_to_dash(s);
    let s = remove_non_ascii_alphanumeric_dash(s);
    let s = condense_dash_runs(s);
    let s = remove_leading_and_trailing_dashes(s);
    s
}

fn remove_non_ascii(s: String) -> String {
    s.chars().filter(char::is_ascii).collect()
}

fn convert_space_to_dash(s: String) -> String {
    s.chars().map(|c| {
        if c == ' ' {
            '-'
        } else {
            c
        }
    }).collect()
}

fn remove_non_ascii_alphanumeric_dash(s: String) -> String {
    s.chars().filter(|c| {
        *c == '-' || c.is_ascii_alphanumeric()
    }).collect()
}

fn condense_dash_runs(s: String) -> String {
    let mut old = s;
    loop {
        let new = old.replace("--", "-");
        if new.len() == old.len() {
            return new;
        } else {
            old = new;
        }
    }
}

fn remove_leading_and_trailing_dashes(mut s: String) -> String {
    let mut s = &mut s[..];
    if s.as_bytes()[0] == b'-' {
        s = &mut s[1..];
    }
    let len = s.len();
    if s.as_bytes()[len - 1] == b'-' {
        s = &mut s[.. len - 1 ];
    }
    s.to_string()
}

/// Some blogs don't put their h1 title inside the `article` tag (e.g.
/// burntsushi). This hack looks for cases where there's the extracted doc
/// contains no h1 before other headers, then looks for an h1 inside the dom and
/// stuff it into the doc.
fn maybe_add_h1(mut doc: Document, post: &str) -> Document {
    if missing_h1(&doc) {
        warn!("missing h1 in {:?}", doc.meta.origin_url);
        if let Some(h1) = find_h1(post) {
            doc.body.blocks.insert(0, Block::Heading(h1));
        }
    }
    doc
}

fn missing_h1(doc: &Document) -> bool {
    for body in &doc.body.blocks {
        match body {
            Block::Heading(h) => {
                if h.level != HeadingLevel::H1 {
                    return true;
                } else {
                    return false;
                }
            }
            _ => { }
        }
    }

    false
}

use crate::html;

fn find_h1(post: &str) -> Option<Heading> {
    let dom = html::extract_dom(post);
    match dom {
        Ok(dom) => {
            let mut h1 = None;
            html::walk_dom_fn(&dom.1, &mut |node| {
            });
            h1
        }
        Err(_) => None
    }
}
