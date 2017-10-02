#[derive(BartDisplay)]
#[template = "src/template/article.html"]
pub struct Article<'a> {
    pub title: &'a str,
    pub category: &'a str,
    pub tags: &'a Vec<String>,
    pub content: String
}
