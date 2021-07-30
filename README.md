<div align="center">
    <h1>Rocketfetch</h1>
    A *WIP* (Contrubutions are greatly apperciated) command line system information tool written in rust for performance
    with toml file configuration.

<a href="https://crates.io/crates/rocketfetch"><img src="https://img.shields.io/crates/v/rocketfetch?style=flat-square" alt="Crates.io version"/></a>
<a href="https://crates.io/crates/rocketfetch"><img src="https://img.shields.io/crates/d/rocketfetch?style=flat-square" alt="Crates.io version"/></a>
<a href="https://libraries.io/cargo/rocketfetch"><img alt="Libraries.io dependency status for latest release" src="https://img.shields.io/librariesio/release/cargo/rocketfetch?style=flat-square"></a>
</div>

# Table of Contents
- [Inspiration](#inspiration)
- [Installation](#installation)
- [Command Line](#cli)
- [Configuration](#configuration)
  - [Top Level](#top_level_configuration)
  - [Format Header](#format_header_configuration)
  - [User Header](#user_header_configuration)
  - [Delimiter Header](#delimiter_header_configuration)
  - [OS Header](#os_header_configuration)
  - [Host Header](#host_header_configuration)
  - [Kernel Header](#kernel_header_configuration)
  - [Custom Header](#custom_header_configuration)
- [Default Configuration](#default_configuration)
- [Todo](#todo)


# Inspiration <a name="inspiration"></a>
_Rocketfetch_ was inspired by [neofetch](https://github.com/dylanaraps/neofetch). I wanted to add some performance to
neofetch while also improving the configuration. I modeled my configuration after that of
[Starship](https://github.com/starship/starship).

# Installation <a name="installation"></a>
```bash
cargo install rocketfetch
```
If this fails to install, please make sure a have the most recent rust version installed.

# Command Line <a name="cli"></a>
```bash
rocketfetch 0.5.3
A WIP command line system information tool (neofetch) rewritten in rust for performance with toml file configuration.

USAGE:
    rocketfetch [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets custom configuration file.
        --mode <mode>      Set the printing mode. Can be one of `side-table`, `bottom-table`, or `classic`
 ```

 # Configuration <a name="configuration"></a>
 You can configure rocketfetch either through the default path of ~/.config/rocketfetch or pass a path in cli by
 specifying the _c option.
 ### Top Level <a name="top_level_configuration"></a>
| Value        | Default             | Description                                                                                                                                                                                                              |
|--------------|---------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| module_order | "user delimiter os" | Specify the order in which the modules will go in seperated by spaces. Possible modules are: user, delimiter, os, as well as any custom modules you have defined. You may specify some more than once. More coming soon. |
| offset       | 4                   | Specify the number of spaces between the logo and the information.                                                                                                                                                       |
| logo_cmd     | "auto"              | The command to run in order to get the logo. If set to auto or nothing, it will automatically detect the operating system and choose a logo                                                                              |
### Format Header <a name="format_header_configuration"></a>
| Value                    | Default     | Description                                                                                                                                                             |
| ----------------------   | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| mode                     | "classic"   | Set the mode of output. Could be one of classic (neofetch style), side_block (with a block around the info), or bottom_block (with the logo above the block of info).   |
| top_left_corner_char     | "╭"         | Set the character to use as the top left corner of the block (Only works in and bottom_block modes).                                                                    |
| top_right_corner_char    | "╮"         | Set the character to use as the top right corner of the block (Only works in side_block and bottom_block modes).                                                        |
| bottom_left_corner_char  | "╰"         | Set the character to use as the bottom left corner of the block (Only works in side_block and bottom_block modes).                                                      |
| bottom_right_corner_char | "╯"         | Set the character to use as the bottom right corner of the block (Only works in side_block and bottom_block modes).                                                     |
| horizontal_char          | "─"         | Set the character to use as the top and bottom parts of the block (Only works in side_block and bottom_block modes).                                                    |
| vertical_char            | "│"         | Set the character to use as the right and left parts of the block (Only works in side_block and bottom_block modes).                                                    |
| padding_left             | 1           | Set the number of spaces to put before each module inside the block (Only works in side_block and bottom_block modes).                                                  |
| padding_right            | 1           | Set the number of spaces to put after each module inside the block (Only works in side_block and bottom_block modes).                                                   |
| padding_top              | 0           | Set the number of lines to put above the modules inside the block (Only works in side_block and bottom_block modes).                                                    |

### User Header <a name="user_header_configuration"></a>
| Value           | Default       | Description                                                                                                         |
|-----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| pre_text_style  | "bold.yellow" | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre_text        | ""            | Text that comes before this module                                                                                  |
| output_style    | "white"       | A format string with each word separated by dots that describes the style of the text                               |
| separator_style | "white"       | A format string with each word separated by dots that describes the style of the between username and hostname      |
| separator_char  | "@"           | A character that separates between the username and hostname                                                        |

### Delimiter Header <a name="delimiter_header_configuration"></a>
| Value        | Default   | Description                                                                                                                                                 |
| ------------ | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| style        | "white"   | A format string with each word separated by dots that describes the style of the delimiter                                                                  |
| repeat_num   | 0         | The number of times to repeat the delimiter char to form the delimiter. If set to 0, it will the repeat number to the length of the module above this one   |
| char         | "_"       | The character to use as the delimiter                                                                                                                       |

### OS Header <a name="os_header_configuration"></a>
| Value            | Default         | Description                                                                                                           |
| ---------------- | --------------- | --------------------------------------------------------------------------------------------------------------------- |
| pre_text_style   | "bold.yellow"   | A format string with each word separated by dots that describes the style of the text that comes before this module   |
| pre_text         | "OS: "          | Text that comes before this module                                                                                    |
| output_style     | "white"         | A format string with each word separated by dots that describes the style of the text                                 |

### Host Header <a name="host_header_configuration"></a>
| Value          | Default       | Description                                                                                                         |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| pre_text_style | "bold.yellow" | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre_text       | "Host: "      | Text that comes before this module                                                                                  |
| output_style   | "white"       | A format string with each word separated by dots that describes the style of the output text                        |

### Kernel Header <a name="kernel_header_configuration"></a>
| Value          | Default       | Description                                                                                                         |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| pre_text_style | "bold.yellow" | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre_text       | "Kernel: "    | Text that comes before this module                                                                                  |
| output_style   | "white"       | A format string with each word separated by dots that describes the style of the output text                        |

### Uptime Header <a name="kernel_header_configuration"></a>
| Value          | Default                                      | Description                                                                                                         |
|----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------|
| pre_text_style | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre_text       | "Uptime: "                                   | Text that comes before this module                                                                                  |
| output_style   | "white"                                      | A format string with each word separated by dots that describes the style of the output text                        |
| time_format    | "$days days, $hours hours, $minutes minutes" | A String that describes the format of the time. Variables are: $years, $days, $hours, $minutes, $seconds.           |


### Custom Header <a name="custom_header_configuration"></a>
| Value          | Description                                                                                                   |
|----------------|---------------------------------------------------------------------------------------------------------------|
| pre_text_style | A format string with each word separated by dots that describes the style of the text that before this module |
| pre_text       | Text that comes before this module                                                                            |
| output_style   | A format string with each word separated by dots that describes the style of the output text                  |
| Command        | The command to run to get the output of the module                                                            |

# Default Configuration <a name="default_configuration"></a>
A default Configuration will look like so:
```toml
module_order = "user delimiter os host kernel uptime"
offset = 4
logo_cmd = "auto"

[format]
mode = "classic"
top_left_corner_char = "╭"
top_right_corner_char = "╮"
bottom_left_corner_char = "╰"
bottom_right_corner_char = "╯"
horizontal_char = "─"
vertical_char = "│"
padding_left = 1
padding_right = 1
padding_top = 0

[user]
pre_text_style = "bold.yellow"
pre_text = ""
output_style = "bold.yellow"
separator_style = "white"
separator_char = "@"

[delimiter]
style = "white"
repeat_num = 0
char = "_"

[os]
pre_text_style = "bold.yellow"
pre_text = "OS: "
output_style = "white"

[host]
pre_text_style = "bold.yellow"
pre_text = "Host: "
output_style = "white"

[kernel]
pre_text_style = "bold.yellow"
pre_text = "Kernel: "
output_style = "white"

[uptime]
pre_text_style = "bold.yellow"
pre_text = "Uptime: "
output_style = "white"
time_format = "$days days, $hours hours, $minutes minutes"
```

# Todo <a name="todo"></a>
- [ ] Add more Modules (For starters: the ones that neofetch supports)
