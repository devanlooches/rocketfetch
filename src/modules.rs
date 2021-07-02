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

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Os {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Os {
    fn default() -> Self {
        Os {
            pre_text_style: String::from("bold yellow"),
            pre_text: String::from("OS: "),
            output_style: String::from("white"),
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Host {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Host {
    fn default() -> Self {
        Host {
            pre_text_style: String::from("bold yellow"),
            pre_text: String::from("Host: "),
            output_style: String::from("white"),
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Kernel {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Kernel {
    fn default() -> Self {
        Kernel {
            pre_text_style: String::from("bold yellow"),
            pre_text: String::from("Kernel: "),
            output_style: String::from("white"),
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Uptime {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Uptime {
    fn default() -> Self {
        Uptime {
            pre_text_style: String::from("bold yellow"),
            pre_text: String::from("Kernel: "),
            output_style: String::from("white"),
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Packages {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Packages {
    fn default() -> Self {
        Packages {
            pre_text_style: String::from("bold yellow"),
            pre_text: String::from("Kernel: "),
            output_style: String::from("white"),
        }
    }
}
