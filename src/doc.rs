use url::Url;

#[derive(Debug)]
pub struct Document {
    pub meta: Meta,
    pub body: Body,
}

#[derive(Debug)]
pub struct Meta {
    pub origin_url: Url,
}

#[derive(Debug)]
pub struct Body {
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub enum Block {
    Heading(Heading),
    Paragraph(Paragraph),
    List(List),
}

#[derive(Debug)]
pub struct Heading {
    pub inlines: Vec<Inline>,
    pub level: HeadingLevel,
}

#[derive(Debug)]
pub enum HeadingLevel {
    H1, H2, H3, H4, H5, H6,
}

#[derive(Debug)]
pub struct Paragraph {
    pub inlines: Vec<Inline>,
}

#[derive(Debug)]
pub enum Inline {
    Text(String),
    Bold(Box<Inline>),
    Italic(Box<Inline>),
    Code(Box<Inline>),
}

#[derive(Debug)]
pub struct List {
    pub type_: ListType,
    pub items: Vec<ListItem>,
}

#[derive(Debug)]
pub enum ListType {
    Ordered, Unordered,
}

#[derive(Debug)]
pub struct ListItem {
    pub blocks: Vec<Block>,
}
