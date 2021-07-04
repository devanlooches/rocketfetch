use crate::config::Config;
use console::{style, Style};
// use nixinfo;
use rsys::Rsys;
#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct User {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    seperator_style: String,
    seperator_char: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from(""),
            output_style: String::from("bold.yellow"),
            seperator_style: String::from("white"),
            seperator_char: String::from("@"),
        }
    }
}

impl User {
    pub async fn get_info(&self) -> String {
        let hostname = Rsys::new().hostname().unwrap();
        let username = Config::run_cmd("whoami").await;
        format!(
            "{}{}{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(username),
            Style::from_dotted_str(&self.seperator_style).apply_to(&self.seperator_char),
            Style::from_dotted_str(&self.output_style).apply_to(hostname)
        )
    }
}
