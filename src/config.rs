use crate::modules::*;

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    user: User,
}

impl Config {
    pub fn from_config() -> Self {
        use clap::{App, Arg};
        let matches = App::new("rustfetch").version("0.1").author("Devan Looches <devan.looches@gmail.com>").about("A WIP command line system information tool (neofetch) rewritten in rust for performance with toml file configuration").arg(Arg::with_name("config").short("c").long("config").value_name("FILE").help("Sets custom config file").takes_value(true)).get_matches();
        let path = matches
            .value_of("config")
            .unwrap_or(&format!(
                "{}/.config/rustfetch/config.toml",
                dirs::home_dir().unwrap().to_string_lossy()
            ))
            .to_string();
        let string = std::fs::read_to_string(path).unwrap();

        toml::from_str::<Config>(&string).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            user: User::default(),
        }
    }
}
