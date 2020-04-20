use anyhow::Result;
use crate::doc::*;

pub fn sanitize(doc: Document) -> Document {
    doc
}

pub fn title_to_file_name(s: String) -> String {
    let s = s.to_lowercase();
    let s = convert_space_to_dash(s);
    let s = remove_non_ascii_alphanumeric_dash(s);
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
