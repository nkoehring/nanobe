use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Write, Error};
use serde_json;

use model::article::Article as ArticleModel;
use template::layout::Layout;
use template::article::Article as ArticleTemplate;

fn default_lang() -> String { "en".into() }
fn default_title() -> String { "Weblog made with Nanobe".into() }
fn default_header() -> String { "A weblog made with Nanobe!".into() }
fn default_footer() -> String { "© 2017 by someone who is using Nanobe".into() }

#[derive(Serialize, Deserialize)]
pub struct Website {
  #[serde(default = "default_lang")] pub language: String,
  #[serde(default = "default_title")] pub title: String,
  #[serde(default = "default_header")] pub header: String,
  #[serde(default = "default_footer")] pub footer: String
}

impl Website {
  pub fn default() -> Website {
    Website {
      language: default_lang(),
      title: default_title(),
      header: default_header(),
      footer: default_footer(),
    }
  }

  pub fn template<'a>(&self, content: &'a ArticleTemplate) -> String {
    Layout {
      language: &self.language,
      title: content.title,
      title_suffix: &self.title,
      content: content,
      header: &self.header,
      footer: &self.footer,
    }.to_string()
  }

  pub fn html(&self) -> String {
    String::from("<h1>not implemented, yet!</h1>")
  }

  pub fn json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn render_article(&self, content: &str, file_name: &str, output_dir: &Path) -> Result<String, Error> {
    let mut path = PathBuf::from(output_dir);
    path.push(file_name);

    let article = ArticleModel::from_markdown(content);
    let html = self.template(&article.template());
    let json = article.json();

    path.set_extension("html");
    let _ = File::create(&path)?.write(&html.as_bytes());

    path.set_extension("json");
    let _ = File::create(&path)?.write(&json.as_bytes());

    Ok(String::from(path.to_string_lossy()))
  }
}
