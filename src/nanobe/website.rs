fn default_lang() -> String { "en".into() }
fn default_title() -> String { "Weblog made with Nanobe".into() }
fn default_header() -> String { "A weblog made with Nanobe!".into() }
fn default_footer() -> String { "© 2017 by someone who is using Nanobe".into() }

#[derive(Serialize, Deserialize, Debug)]
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
}
