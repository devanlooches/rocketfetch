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
            pre_text_style: String::from("bold yellow"),
            pre_text: String::from(""),
            output_style: String::from("bold yellow"),
            seperator_style: String::from("white"),
            seperator_char: String::from("@"),
        }
    }
}

impl User {
    pub fn get_info() -> String {
        String::from("Devan@Nobody")
    }
}
