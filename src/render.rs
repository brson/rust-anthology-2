use std::io::Write;
use anyhow::Result;
use crate::doc::*;
use v_htmlescape::escape;
use crate::assets::AssetDirs;
use crate::assets::{RESET_CSS_FILE, MAIN_CSS_FILE, BLOG_CSS_FILE};

pub fn to_string(assets: &AssetDirs, doc: &Document) -> Result<String> {
    let mut buf = Vec::new();
    render_doc(&mut buf, assets, doc);

    Ok(String::from_utf8(buf)?)
}

type Buf = Vec<u8>;

fn render_doc(buf: &mut Buf, assets: &AssetDirs, doc: &Document) {
    writeln!(buf, "<!doctype html>");
    writeln!(buf, "<html lang='en'>");

    render_head(buf, assets, &doc.meta);
    render_body(buf, &doc.body);

    writeln!(buf, "</html>");
}    

fn render_head(buf: &mut Buf, assets: &AssetDirs, meta: &Meta) {
    let reset_file = assets.css_dir.join(RESET_CSS_FILE);
    let main_file = assets.css_dir.join(MAIN_CSS_FILE);
    let blog_file = assets.css_dir.join(BLOG_CSS_FILE);

    writeln!(buf);
    writeln!(buf, "<head>");
    writeln!(buf, "  <meta charset='utf-8'>");
    writeln!(buf, "  <link rel='stylesheet' href='{}'>", reset_file.display());
    writeln!(buf, "  <link rel='stylesheet' href='{}'>", main_file.display());
    writeln!(buf, "  <link rel='stylesheet' href='{}'>", blog_file.display());
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
        Block::List(list) => {
            render_list(buf, list);
        }
    }
    writeln!(buf);
}

fn render_heading(buf: &mut Buf, heading: &Heading) {
    let lvl = match heading.level {
        HeadingLevel::H1 => "h1",
        HeadingLevel::H2 => "h2",
        HeadingLevel::H3 => "h3",
        HeadingLevel::H4 => "h4",
        HeadingLevel::H5 => "h5",
        HeadingLevel::H6 => "h6",
    };
    writeln!(buf, "<{}>", lvl);
    for inline in &heading.inlines {
        render_inline(buf, inline);
    }
    writeln!(buf, "</{}>", lvl);
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

fn render_list(buf: &mut Buf, list: &List) {
    let tag = match list.type_ {
        ListType::Unordered => "ul",
        ListType::Ordered => "ol",
    };

    writeln!(buf, "<{}>", tag);
    for item in &list.items {
        render_list_item(buf, item);
    }
    writeln!(buf);
    writeln!(buf, "</{}>", tag);
}

fn render_list_item(buf: &mut Buf, item: &ListItem) {
    writeln!(buf, "<li>");
    for block in &item.blocks {
        render_block(buf, block);
    }
    writeln!(buf, "</li>");
}
