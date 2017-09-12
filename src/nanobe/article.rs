/* some parts inspired by the frontmatter crate
 * https://docs.rs/crate/frontmatter/0.3.0/source/src/lib.rs */

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Write, Error};
use serde_yaml;
use serde_json;
use chrono::Local;
use comrak::{markdown_to_html, ComrakOptions};


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

// rolled a couple of dice for enhanced randomness
fn gen_id() -> i64 { 23 }
fn default_author() -> String { "no author".into() }
fn default_title() -> String { "no title".into() }
fn default_tags() -> Vec<String> { vec![] }
fn default_created_at() -> i64 { Local::now().timestamp() }
fn default_content() -> String { "no content".into() }

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
  #[serde(default = "gen_id")] pub id: i64,
  #[serde(default = "default_title")] pub title: String,
  #[serde(default = "default_author")] pub author: String,
  #[serde(default = "default_tags")] pub tags: Vec<String>,
  #[serde(default = "default_created_at")] pub created_at: i64,
  #[serde(default = "default_content")] pub content: String,
  pub category: Option<String>,
  pub publish_at: Option<String>,
}

impl Article {
  fn default() -> Article {
    Article {
      id: gen_id(),
      title: default_title(),
      author: default_author(),
      tags: default_tags(),
      created_at: default_created_at(),
      content: default_content(),
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
        article.content = content.into();
        article
      },
      None => {
        let mut article = Article::default();
        article.content = text.into();
        article
      }
    }
  }

  pub fn html(&self) -> String {
    markdown_to_html(&self.content, &ComrakOptions::default())
  }

  pub fn json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn dump(&self, file_name: &str, output_dir: &Path) -> Result<String, Error> {
    let mut path = PathBuf::from(output_dir);
    path.push(file_name);

    let html = self.html();
    let json = self.json();

    path.set_extension("html");
    let _ = File::create(&path)?.write(&html.as_bytes());

    path.set_extension("json");
    let _ = File::create(&path)?.write(&json.as_bytes());

    Ok(String::from(path.to_string_lossy()))
  }
}
