use crate::cli::Mode;
extern crate pest;
use crate::cli::Opt;
use crate::modules::*;
use crate::utils::{handle_error_option, handle_error_result};
use console::measure_text_width;
use console::style;
use console::Style;
use libmacchina::GeneralReadout;
use libmacchina::KernelReadout;
use libmacchina::PackageReadout;
use pest::Parser;
use std::cmp::Ordering;
use std::collections::HashMap;
use structopt::StructOpt;
use user_error::{UserFacingError, UFE};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    module_order: String,
    offset: usize,
    logo_cmd: String,
    format: Format,
    user: User,
    delimiter: Delimiter,
    os: Os,
    host: Host,
    kernel: Kernel,
    uptime: Uptime,
    packages: Packages,

    #[serde(flatten)]
    custom_modules: HashMap<String, Module>,
}

#[derive(Parser)]
#[grammar = "toml.pest"]
pub struct TomlParser;

impl Config {
    pub async fn get_args() -> Opt {
        Opt::from_args()
    }
    pub async fn path() -> String {
        let matches = Config::get_args().await;
        let home_dir = handle_error_option(dirs::home_dir(), "Failed to find home directory", None);
        let path = matches.config.unwrap_or(format!(
            "{}/.config/rocketfetch/config.toml",
            home_dir.to_string_lossy()
        ));
        path
    }
    pub async fn from_config(path: String) -> Self {
        match std::fs::read_to_string(path) {
            Ok(string) => match toml::from_str::<Config>(&string) {
                Ok(v) => v,
                Err(r) => {
                    Config::pest_parse(&string).await;
                    let mut line: u64 = 0;
                    let mut column: u64 = 0;
                    let mut last = String::new();
                    for word in handle_error_option(
                        r.to_string().split("at").last(),
                        "Failed to get line and column number of configuration error.",
                        None,
                    )
                    .split_whitespace()
                    {
                        if last == "line" {
                            line = handle_error_result(
                                word.parse::<u64>(),
                                Some("Failed to get line number of configuration error."),
                                None,
                            );
                        } else if last == "column" {
                            column = handle_error_result(
                                word.parse::<u64>(),
                                Some("Failed to get column number of configuration error."),
                                None,
                            );
                        }
                        last = word.to_string();
                    }
                    UserFacingError::new("Unable to parse toml file")
                        .reason(format!(
                            "--> {line}:{col}
{line_len_sep} |
{line} | {line_contents}
{line_len_sep} |{col_len_sep}^--- {error}
{line_len_sep} |",
                            error = r,
                            line_len_sep = " ".repeat(line.to_string().len()),
                            col_len_sep = " ".repeat(column.to_string().len()),
                            line = line,
                            line_contents =
                                string.lines().collect::<Vec<&str>>()[(line - 1) as usize],
                            col = column,
                        ))
                        .print_and_exit();
                    unreachable!()
                }
            },

            Err(r) => {
                println!(
                    "{}: Could not find default configuration file: {}. Falling back to default configuration.",
                    style("WARNING").yellow(),
                    r
                );
                Config::default()
            }
        }
    }

    async fn module_order(
        &self,
        kernel_readout: &KernelReadout,
        general_readout: &GeneralReadout,
        package_readout: &PackageReadout,
    ) -> Vec<String> {
        let mut vec = Vec::new();
        for (i, module) in self.module_order.split_whitespace().enumerate() {
            match module {
                "user" => vec.push(self.user.get_info(general_readout).await),
                "delimiter" => vec.push(
                    self.delimiter
                        .get_info(measure_text_width(&vec[i - 1]))
                        .await,
                ),
                "os" => vec.push(self.os.get_info(general_readout).await),
                "host" => vec.push(self.host.get_info(general_readout).await),
                "kernel" => vec.push(self.kernel.get_info(kernel_readout).await),
                "uptime" => vec.push(self.uptime.get_info(general_readout).await),
                "packages" => vec.push(self.packages.get_info(package_readout).await),
                v if !self.custom_modules.is_empty() && self.custom_modules.contains_key(v) => {
                    // Can unwrap because checked above that the key is there
                    vec.push(self.custom_modules.get(v).unwrap().get_info().await)
                }
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

    async fn logo(&self, general_readout: &GeneralReadout) -> Vec<String> {
        let os = self.os.get_os(general_readout).await;
        match os.trim() {
            x if x.contains("macOS") => {
                let yellow = Style::from_dotted_str("yellow.bold");
                let red = Style::from_dotted_str("red.bold");
                let blue = Style::from_dotted_str("blue.bold");
                let green = Style::from_dotted_str("green.bold");
                let purple = Style::new().color256(5).bold();
                vec![
                    green.apply_to("                 ,xNMM.").to_string(),
                    green.apply_to("               .OMMMMo").to_string(),
                    green.apply_to("               OMMM0,").to_string(),
                    yellow.apply_to("     .;loddo:' loolloddol;.").to_string(),
                    yellow.apply_to("   cKMMMMMMMMMMNWMMMMMMMMMM0:").to_string(),
                    yellow.apply_to(" .KMMMMMMMMMMMMMMMMMMMMMMMWd.").to_string(),
                    yellow.apply_to(" XMMMMMMMMMMMMMMMMMMMMMMMX.").to_string(),
                    yellow.apply_to(";MMMMMMMMMMMMMMMMMMMMMMMM:").to_string(),
                    red.apply_to(":MMMMMMMMMMMMMMMMMMMMMMMM:").to_string(),
                    red.apply_to(".MMMMMMMMMMMMMMMMMMMMMMMMX.").to_string(),
                    red.apply_to(" kMMMMMMMMMMMMMMMMMMMMMMMMWd.").to_string(),
                    red.apply_to(" .XMMMMMMMMMMMMMMMMMMMMMMMMMMk").to_string(),
                    purple
                        .apply_to("  .XMMMMMMMMMMMMMMMMMMMMMMMMK.")
                        .to_string(),
                    purple.apply_to("    kMMMMMMMMMMMMMMMMMMMMMMd").to_string(),
                    blue.apply_to("     ;KMMMMMMMWXXWMMMMMMMk.").to_string(),
                    blue.apply_to("       .cooc,.    .,coo:.").to_string(),
                ]
            }
            "Arch Linux" => {
                let lightblue = Style::new().blue().bright();
                vec![
                    "                     -`                 ",
                    "                    .o+`                ",
                    "                   `ooo/                ",
                    "                  `+oooo:               ",
                    "                 `+oooooo:              ",
                    "                 -+oooooo+:             ",
                    "               `/:-:++oooo+:            ",
                    "              `/++++/+++++++:           ",
                    "             `/++++++++++++++:          ",
                    "            `/+++ooooooooooooo/`        ",
                    "           ./ooosssso++osssssso+`       ",
                    "          .oossssso-````/ossssss+`      ",
                    "         -osssssso.      :ssssssso.     ",
                    "        :osssssss/        osssso+++.    ",
                    "       /ossssssss/        +ssssooo/-    ",
                    "     `/ossssso+/:-        -:/+osssso+-  ",
                    "    `+sso+:-`                 `.-/+oso: ",
                    "   `++:.                           `-/+/",
                    "   .`                                 `/",
                ]
                .iter()
                .map(|&x| lightblue.apply_to(x).to_string())
                .collect()
            }
            v => {
                UserFacingError::new(format!("Unknown OS: {}", v))
                    .help("Please file a new issue on github to request a new OS: https://github.com/devanlooches/rocketfetch/issues/new")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    async fn get_logo(&self, general_readout: &GeneralReadout) -> Vec<String> {
        if self.logo_cmd.is_empty() || self.logo_cmd == "auto" {
            self.logo(general_readout).await
        } else {
            Config::run_cmd(&self.logo_cmd)
                .await
                .lines()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        }
    }

    async fn print_classic(
        &self,
        kernel_readout: &KernelReadout,
        general_readout: &GeneralReadout,
        package_readout: &PackageReadout,
    ) {
        let mut sidelogo = self.get_logo(general_readout).await;
        let mut order = self
            .module_order(kernel_readout, general_readout, package_readout)
            .await;

        let maxlength = self.logo_maxlength(general_readout).await;

        match sidelogo.len().cmp(&order.len()) {
            Ordering::Greater => order.resize(sidelogo.len(), String::from("")),
            Ordering::Less => sidelogo.resize(order.len(), String::from("")),
            Ordering::Equal => (),
        }
        for (i, line) in sidelogo.iter().enumerate() {
            println!(
                "{}{}{}",
                line,
                " ".repeat(maxlength - measure_text_width(line) + self.offset),
                order[i]
            );
        }
    }

    pub async fn run_cmd(cmd: &str) -> String {
        use std::process::Command;
        let output = if cfg!(target_os = "windows") {
            match Command::new("cmd").args(&["/C", cmd]).output() {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to execute command")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            }
        } else {
            match Command::new("sh").args(["-c", cmd]).output() {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to execute command")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            }
        };
        match String::from_utf8(output.stdout) {
            Ok(v) => v.trim().to_string(),
            Err(r) => {
                UserFacingError::new("Failed to read stdout from command.")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    async fn logo_maxlength(&self, general_readout: &GeneralReadout) -> usize {
        match self
            .get_logo(general_readout)
            .await
            .iter()
            .max_by_key(|&x| measure_text_width(x))
        {
            Some(v) => measure_text_width(v),
            None => {
                UserFacingError::new("Failed to find logo line with greatest length.")
                    .help("If this persists, please open a github issue: https://github.com/devanlooches/rocketfetch/issues/new")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    async fn info_maxlength(
        &self,
        kernel_readout: &KernelReadout,
        general_readout: &GeneralReadout,
        package_readout: &PackageReadout,
    ) -> usize {
        match self
            .module_order(kernel_readout, general_readout, package_readout)
            .await
            .iter()
            .max_by_key(|&x| measure_text_width(x))
        {
            Some(v) => measure_text_width(v),
            None => {
                UserFacingError::new("Failed to find info line with the greatest length")
                    .help("Make sure that you have some modules defined. If this persists, please open a github issue: https://github.com/devanlooches/rocketfetch/issues/new")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    async fn print_side_table(
        &self,
        kernel_readout: &KernelReadout,
        general_readout: &GeneralReadout,
        package_readout: &PackageReadout,
    ) {
        let mut sidelogo = self.get_logo(general_readout).await;
        let mut info = self
            .module_order(kernel_readout, general_readout, package_readout)
            .await;
        let mut counter = 0;
        info.resize(sidelogo.len() + self.format.padding_top, String::from(""));
        sidelogo.resize(info.len() + self.format.padding_top, String::from(""));

        let logo_maxlength = self.logo_maxlength(general_readout).await;
        let info_maxlength = self
            .info_maxlength(kernel_readout, general_readout, package_readout)
            .await;

        println!(
            "{}{}{}{}{}",
            handle_error_option(
                sidelogo.first(),
                "Failed to get first line of sidelogo",
                None
            ),
            " ".repeat(logo_maxlength - measure_text_width(&sidelogo[0]) + self.offset),
            self.format.top_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_left + self.format.padding_right),
            self.format.top_right_corner_char,
        );
        counter += 1;

        for i in 0..self.format.padding_top {
            println!(
                "{}{}{vertical}{}{vertical}",
                sidelogo[i + counter],
                " ".repeat(
                    logo_maxlength - measure_text_width(&sidelogo[i + counter]) + self.offset
                ),
                " ".repeat(info_maxlength + self.format.padding_right + self.format.padding_left),
                vertical = self.format.vertical_char
            );
            counter += 1;
        }

        for i in info.iter().take(info.len() - 2) {
            println!(
                "{}{}{vertical}{}{}{}{}{vertical}",
                sidelogo[counter],
                " ".repeat(logo_maxlength - measure_text_width(&sidelogo[counter]) + self.offset),
                " ".repeat(self.format.padding_left),
                i,
                " ".repeat(self.format.padding_right),
                " ".repeat(info_maxlength - measure_text_width(i)),
                vertical = self.format.vertical_char
            );
            counter += 1;
        }
        println!(
            "{}{}{}{}{}",
            sidelogo[counter],
            " ".repeat(logo_maxlength - measure_text_width(&sidelogo[counter]) + self.offset),
            self.format.bottom_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_left + self.format.padding_right),
            self.format.bottom_right_corner_char,
        );
    }

    async fn print_bottom_table(
        &self,
        kernel_readout: &KernelReadout,
        general_readout: &GeneralReadout,
        package_readout: &PackageReadout,
    ) {
        let sidelogo = self.get_logo(general_readout).await;
        let info = self
            .module_order(kernel_readout, general_readout, package_readout)
            .await;
        let info_maxlength = self
            .info_maxlength(kernel_readout, general_readout, package_readout)
            .await;

        for line in sidelogo {
            println!("{}", line);
        }
        println!(
            "{}{}{}",
            self.format.top_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_right + self.format.padding_left),
            self.format.top_right_corner_char
        );
        for _ in 0..self.format.padding_top {
            println!(
                "{vertical}{}{vertical}",
                " ".repeat(info_maxlength + self.format.padding_right + self.format.padding_left),
                vertical = self.format.vertical_char
            )
        }
        for line in info {
            println!(
                "{vertical}{}{}{}{}{vertical}",
                " ".repeat(self.format.padding_left),
                line,
                " ".repeat(info_maxlength - measure_text_width(&line)),
                " ".repeat(self.format.padding_right),
                vertical = self.format.vertical_char
            );
        }
        println!(
            "{}{}{}",
            self.format.bottom_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_left + self.format.padding_right),
            self.format.bottom_right_corner_char
        );
    }

    pub async fn print(
        &self,
        kernel_readout: &KernelReadout,
        general_readout: &GeneralReadout,
        package_readout: &PackageReadout,
    ) {
        let matches = Config::get_args().await;
        if let Some(v) = matches.mode {
            match v {
                Mode::Classic => {
                    self.print_classic(kernel_readout, general_readout, package_readout)
                        .await
                }
                Mode::BottomBlock => {
                    self.print_bottom_table(kernel_readout, general_readout, package_readout)
                        .await
                }
                Mode::SideBlock => {
                    self.print_side_table(kernel_readout, general_readout, package_readout)
                        .await
                }
            }
        } else {
            match self.format.mode {
                Mode::Classic => {
                    self.print_classic(kernel_readout, general_readout, package_readout)
                        .await
                }
                Mode::BottomBlock => {
                    self.print_bottom_table(kernel_readout, general_readout, package_readout)
                        .await
                }
                Mode::SideBlock => {
                    self.print_side_table(kernel_readout, general_readout, package_readout)
                        .await
                }
            }
        }
    }

    pub async fn pest_parse(content: &str) {
        handle_error_result(TomlParser::parse(Rule::toml, content), None, None);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            offset: 4,
            module_order: String::from("user delimiter os host kernel uptime packages"),
            logo_cmd: String::from("auto"),
            format: Format::default(),
            user: User::default(),
            delimiter: Delimiter::default(),
            os: Os::default(),
            host: Host::default(),
            kernel: Kernel::default(),
            uptime: Uptime::default(),
            custom_modules: HashMap::new(),
            packages: Packages::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn check_default_config() {
        let config = Config::from_config(String::from("config.toml")).await;
        assert_eq!(Config::default(), config);
    }
}
