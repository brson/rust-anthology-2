use log::debug;
use anyhow::{Result, Context, anyhow};
use crate::doc::*;

pub fn title(doc: &Document) -> Option<String> {
    let mut headings = Vec::new();
    walk_doc(&mut headings, doc);

    debug!("headings for {}", doc.meta.origin_url);
    for heading in &headings {
        debug!("{:?}", heading);
    }

    headings.reverse();
    headings.pop().map(|h| h.1)
}

type Hd = (HeadingLevel, String);

fn walk_doc(hs: &mut Vec<Hd>, doc: &Document) {
    walk_blocks(hs, &doc.body.blocks);
}

fn walk_blocks(hs: &mut Vec<Hd>, blocks: &Vec<Block>) {
    for block in blocks {
        match block {
            Block::Heading(Heading { inlines, level }) => {
                let mut buf = String::new();
                cat_text_inlines(&mut buf, inlines);
                hs.push((*level, buf));
            }
            _ => { }
        }
    }
}

fn cat_text_inlines(buf: &mut String, inlines: &Vec<Inline>) {
    for inline in inlines {
        match inline {
            Inline::Text(text) => {
                buf.push_str(text);
            }
            Inline::Bold(inlines) |
            Inline::Italic(inlines) |
            Inline::Code(inlines) => {
                cat_text_inlines(buf, inlines);
            }
        }
    }
}
