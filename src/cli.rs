use structopt::StructOpt;
use user_error::UserFacingError;

#[derive(StructOpt)]
#[structopt(
    about = "A WIP command line system information tool (neofetch) rewritten in rust for performance with toml file configuration."
)]
pub struct Opt {
    #[structopt(
        short = "c",
        long,
        name = "FILE",
        help = "Sets custom configuration file."
    )]
    pub config: Option<String>,
    #[structopt(
        long,
        help = "Set the printing mode. Can be one of `side-block`, `bottom-block`, or `classic`"
    )]
    pub mode: Option<Mode>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Classic,
    SideBlock,
    BottomBlock,
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "classic" => Ok(Self::Classic),
            "side-block" | "sideblock" => Ok(Self::SideBlock),
            "bottom-block" | "bottomblock" => Ok(Self::BottomBlock),
            v => Err(format!(
                "\n{}",
                UserFacingError::new("Unable to parse mode string")
                    .reason(format!("Unknown Mode: {}", v))
                    .help("Expected one of `side-block`, `bottom-block`, or `classic`")
            )),
        }
    }
}
