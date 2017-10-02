/* some parts inspired by the frontmatter crate
 * https://docs.rs/crate/frontmatter/0.3.0/source/src/lib.rs */

use serde_yaml;
use serde_json;
use chrono::Local;
use comrak::{markdown_to_html, ComrakOptions};

use template::article::Article as ArticleTemplate;


fn find_yaml_block(text: &str) -> Option<(usize, usize, usize)> {
  match text.starts_with("---\n") {
    true => {
      let slice_after_marker = &text[4..];
      let fm_end = slice_after_marker.find("\n---\n");
      if fm_end.is_none() {
        return None
      };

      let fm_end = fm_end.unwrap();
      Some((4, fm_end+4, fm_end+2*4))
    },
    false => None
  }
}

fn default_author() -> String { "no author".into() }
fn default_title() -> String { "no title".into() }
fn default_tags() -> Vec<String> { vec![] }
fn default_created_at() -> i64 { Local::now().timestamp() }
fn default_content() -> String { "no content".into() }

#[derive(Serialize, Deserialize)]
pub struct Article {
  #[serde(default = "default_title")] pub title: String,
  #[serde(default = "default_author")] pub author: String,
  #[serde(default = "default_tags")] pub tags: Vec<String>,
  #[serde(default = "default_created_at")] pub created_at: i64,
  #[serde(default = "default_content")] pub raw_content: String,
  pub category: Option<String>,
  pub publish_at: Option<String>,
}

impl Article {
  fn default() -> Article {
    Article {
      title: default_title(),
      author: default_author(),
      tags: default_tags(),
      created_at: default_created_at(),
      raw_content: default_content(),
      category: None,
      publish_at: None,
    }
  }

  pub fn from_markdown(text: &str) -> Article {
    match find_yaml_block(text) {
      Some((fm_start, fm_end, content_start)) => {
        let matter = &text[fm_start..fm_end];
        let content = &text[content_start..];
        let mut article: Article = serde_yaml::from_str(matter).unwrap();
        article.raw_content = content.into();
        article
      },
      None => {
        let mut article = Article::default();
        article.raw_content = text.into();
        article
      }
    }
  }

  pub fn json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn template<'a>(&self) -> ArticleTemplate {
    ArticleTemplate {
      title: &self.title,
      category: self.category.as_ref().map(String::as_str).unwrap_or_default(),
      tags: &self.tags,
      content: markdown_to_html(&self.raw_content, &ComrakOptions::default())
    }
  }
}
