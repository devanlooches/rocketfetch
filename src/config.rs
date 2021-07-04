use crate::cli::Opt;
use crate::modules::*;
use console::style;
use structopt::StructOpt;
use user_error::{UserFacingError, UFE};

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    module_order: String,
    offset: usize,
    side_icon_cmd: String,
    user: User,
}

impl Config {
    pub async fn cli() -> Opt {
        Opt::from_args()
    }
    pub async fn from_config() -> Self {
        let matches = Config::cli().await;
        let path = matches
            .config
            .unwrap_or(format!(
                "{}/.config/rustfetch/config.toml",
                dirs::home_dir().unwrap().to_string_lossy()
            ))
            .to_string();
        match std::fs::read_to_string(path) {
            Ok(v) => match toml::from_str::<Config>(&v) {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to parse toml file.")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            },

            Err(r) => {
                println!(
                    "{}: {}. Falling back to default configuration.",
                    style("WARNING").yellow(),
                    r.to_string()
                );
                Config::default()
            }
        }
    }

    async fn module_order(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for module in self.module_order.split_whitespace() {
            match module {
                "user" => vec.push(User::get_info()),
                v => {
                    UserFacingError::new("Failed to parse module order string.")
                        .reason(format!("Unknown module: {}", v))
                        .print_and_exit();
                    unreachable!();
                }
            }
        }
        vec
    }

    async fn get_logo() -> Vec<String> {
        todo!()
    }

    async fn get_side_logo(&self) -> Vec<String> {
        if self.side_icon_cmd == "" || self.side_icon_cmd == "auto" {
            Config::get_logo().await
        } else {
            self.run_cmd(&self.side_icon_cmd)
                .await
                .lines()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        }
    }

    async fn print_classic(&self) {
        use console::measure_text_width;
        let mut sidelogo = self.get_side_logo().await;
        let mut order = self.module_order().await;

        let maxlength = sidelogo
            .iter()
            .max_by(|&x, &y| measure_text_width(x).cmp(&measure_text_width(y)))
            .unwrap()
            .len();
        if sidelogo.len() > order.len() {
            order.resize(sidelogo.len(), String::from(""));
        } else if order.len() > sidelogo.len() {
            sidelogo.resize(order.len(), String::from(""));
        }
        for (i, line) in sidelogo.iter().enumerate() {
            println!(
                "{}{}{}",
                line,
                "".repeat(maxlength - line.len() + self.offset),
                order[i]
            );
        }
    }

    async fn run_cmd(&self, cmd: &String) -> String {
        use std::process::Command;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", cmd])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .args(["-c", cmd])
                .output()
                .expect("failed to execute process")
        };
        String::from_utf8(output.stdout).expect("Failed to read output")
    }

    async fn print_side_table(&self) {
        todo!()
    }

    async fn print_bottom_table(&self) {
        todo!()
    }

    pub async fn print(&self) {
        use crate::cli::Mode;
        let matches = Config::cli().await;
        match matches.mode {
            Mode::Classic => self.print_classic().await,    
            Mode::BottomTable => self.print_bottom_table().await,
            Mode::SideTable => self.print_side_table().await,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            user: User::default(),
            offset: 4,
            module_order: String::from(""),
            side_icon_cmd: String::from("echo hello | cowsay"),
        }
    }
}
