use anyhow::Result;
use crate::doc::*;

pub fn sanitize(doc: Document) -> Document {
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
