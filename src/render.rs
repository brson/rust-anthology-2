use std::io::Write;
use anyhow::Result;
use crate::doc::*;
use v_htmlescape::escape;

pub fn to_string(doc: &Document) -> Result<String> {
    let mut buf = Vec::new();
    render_doc(&mut buf, doc);

    Ok(String::from_utf8(buf)?)
}

type Buf = Vec<u8>;

fn render_doc(buf: &mut Buf, doc: &Document) {
    writeln!(buf, "<!doctype html>");
    writeln!(buf, "<html lang='en'>");

    render_head(buf, &doc.meta);
    render_body(buf, &doc.body);

    writeln!(buf, "</html>");
}    

fn render_head(buf: &mut Buf, meta: &Meta) {
    writeln!(buf);
    writeln!(buf, "<head>");
    writeln!(buf, "  <meta charset='utf-8'>");
    writeln!(buf, "</head>");
    writeln!(buf);
}

fn render_body(buf: &mut Buf, body: &Body) {
    writeln!(buf);
    writeln!(buf, "<body>");
    writeln!(buf, "<main>");
    writeln!(buf, "<article>");
    for block in &body.blocks {
        render_block(buf, block);
    }
    writeln!(buf, "</article>");
    writeln!(buf, "</main>");
    writeln!(buf, "</body>");
    writeln!(buf);
}

fn render_block(buf: &mut Buf, block: &Block) {
    writeln!(buf);
    match block {
        Block::Heading(heading) => {
            render_heading(buf, heading);
        }
        Block::Paragraph(para) => {
            render_paragraph(buf, para);
        }
    }
    writeln!(buf);
}

fn render_heading(buf: &mut Buf, heading: &Heading) {
    panic!()
}

fn render_paragraph(buf: &mut Buf, para: &Paragraph) {
    writeln!(buf, "<p>");
    for inline in &para.inlines {
        render_inline(buf, inline);
    }
    writeln!(buf);
    writeln!(buf, "</p>");
}

fn render_inline(buf: &mut Buf, inline: &Inline) {
    match inline {
        Inline::Text(text) => {
            write!(buf, "{}", escape(text));
        }
        _ => {
            panic!()
        }
    }
}
