use std::fmt;

#[derive(BartDisplay)]
#[template = "src/template/layout.html"]
pub struct Layout<'a, T: 'a + fmt::Display> {
    pub language: &'a str,
    pub title: &'a str,
    pub title_suffix: &'a str,
    pub content: &'a T,
    pub header: &'a str,
    pub footer: &'a str,
}
