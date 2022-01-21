<div align="center">
    <h1>Rocketfetch</h1>
    A *WIP* (Contributions are greatly appreciated) command line system information tool written in Rust for performance
    with TOML file configuration.

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
  - [Uptime Header](#uptime_header_configuration)
  - [Packages Header](#packages_header_configuration)
  - [Packages Header](#packages_header_configuration)
  - [Shell Header](#shell_header_configuration)
  - [Resolution Header](#resolution_header_configuration)
  - [Desktop Environment Header](#desktop_environment_header_configuration)
  - [Window Manager Header](#window_manager_header_configuration)
  - [Terminal Header](#terminal_header_configuration)
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
If this fails to install, please make sure you have the most recent Rust version installed.

# Command Line <a name="cli"></a>
```bash
rocketfetch 0.5.3
A WIP command line system information tool (neofetch) rewritten in Rust for performance with TOML file configuration.

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
 specifying the -c option.
 ### Top Level <a name="top_level_configuration"></a>
| Value        | Default             | Description                                                                                                                                                                                                              |
|--------------|---------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| module-order | "user delimiter os host kernel uptime packages" | Specify the order in which the modules will go in separated by spaces. Possible modules are: user, delimiter, os, host, kernel, uptime, packages, as well as any custom modules you have defined. You may specify some more than once. More coming soon! |
| offset       | 4                   | Specify the number of spaces between the logo and the information.                                                                                                                                                       |
| logo-cmd     | "auto"              | The command to run in order to get the logo. If set to auto or nothing, it will automatically detect the operating system and choose a logo based on the result. (Only macos and Arch Linux is supported as of right now.)                                                                             |

### Format Header <a name="format_header_configuration"></a>
| Value                    | Default     | Description                                                                                                                                                             |
| ----------------------   | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| mode                     | "classic"   | Set the mode of output. Could be one of classic (neofetch style), side-block (with a block around the info), or bottom-block (with the logo above the block of info).   |
| top-left-corner-char     | "╭"         | Set the character to use as the top left corner of the block (Only works in and bottom-block modes).                                                                    |
| top-right-corner-char    | "╮"         | Set the character to use as the top right corner of the block (Only works in side-block and bottom-block modes).                                                        |
| bottom-left-corner-char  | "╰"         | Set the character to use as the bottom left corner of the block (Only works in side-block and bottom-block modes).                                                      |
| bottom-right-corner-char | "╯"         | Set the character to use as the bottom right corner of the block (Only works in side-block and bottom-block modes).                                                     |
| horizontal-char          | "─"         | Set the character to use as the top and bottom parts of the block (Only works in side-block and bottom-block modes).                                                    |
| vertical-char            | "│"         | Set the character to use as the right and left parts of the block (Only works in side-block and bottom-block modes).                                                    |
| padding-left             | 1           | Set the number of spaces to put before each module inside the block (Only works in side-block and bottom-block modes).                                                  |
| padding-right            | 1           | Set the number of spaces to put after each module inside the block (Only works in side-block and bottom-block modes).                                                   |
| padding-top              | 0           | Set the number of lines to put above the modules inside the block (Only works in side-block and bottom-block modes).                                                    |

### User Header <a name="user_header_configuration"></a>
| Value           | Default       | Description                                                                                                         |
|-----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow" | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre-text        | ""            | Text that comes before this module                                                                                  |
| output-style    | "white"       | A format string with each word separated by dots that describes the style of the text                               |
| separator-style | "white"       | A format string with each word separated by dots that describes the style of the between username and hostname      |
| separator-char  | "@"           | A character that separates between the username and hostname                                                        |

### Delimiter Header <a name="delimiter_header_configuration"></a>
| Value        | Default   | Description                                                                                                                                                 |
| ------------ | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| style        | "white"   | A format string with each word separated by dots that describes the style of the delimiter                                                                  |
| repeat-num   | 0         | The number of times to repeat the delimiter char to form the delimiter. If set to 0, it will the repeat number to the length of the module above this one   |
| char         | "-"       | The character to use as the delimiter                                                                                                                       |

### OS Header <a name="os_header_configuration"></a>
| Value            | Default         | Description                                                                                                           |
| ---------------- | --------------- | --------------------------------------------------------------------------------------------------------------------- |
| pre-text-style   | "bold.yellow"   | A format string with each word separated by dots that describes the style of the text that comes before this module   |
| pre-text         | "OS: "          | Text that comes before this module                                                                                    |
| output-style     | "white"         | A format string with each word separated by dots that describes the style of the text                                 |

### Host Header <a name="host_header_configuration"></a>
| Value          | Default       | Description                                                                                                         |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| pre-text-style | "bold.yellow" | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre-text       | "Host: "      | Text that comes before this module                                                                                  |
| output-style   | "white"       | A format string with each word separated by dots that describes the style of the output text                        |

### Kernel Header <a name="kernel_header_configuration"></a>
| Value          | Default       | Description                                                                                                         |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| pre-text-style | "bold.yellow" | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre-text       | "Kernel: "    | Text that comes before this module                                                                                  |
| output-style   | "white"       | A format string with each word separated by dots that describes the style of the output text                        |

### Uptime Header <a name="uptime_header_configuration"></a>
| Value          | Default                                      | Description                                                                                                         |
|----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------|
| pre-text-style | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module |
| pre-text       | "Uptime: "                                   | Text that comes before this module                                                                                  |
| output-style   | "white"                                      | A format string with each word separated by dots that describes the style of the output text                        |
| time-format    | "$days days, $hours hours, $minutes minutes" | A String that describes the format of the time. Variables are: $years, $days, $hours, $minutes, $seconds.           |

### Packages Header <a name="packages_header_configuration"></a>
| Value           | Default                                      | Description                                                                                                               |
|-----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module       |
| pre-text        | "Packages: "                                 | Text that comes before this module                                                                                        |
| output-style    | "white"                                      | A format string with each word separated by dots that describes the style of the output text                              |

### Shell Header <a name="shell_header_configuration"></a>
| Value           | Default                                      | Description                                                                                                               |
|-----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module       |
| pre-text        | "Shell: "                                    | Text that comes before this module                                                                                        |
| output-style    | "white"                                      | A format string with each word separated by dots that describes the style of the output text                              |

### Resolution Header <a name="resolution_header_configuration"></a>
| Value           | Default                                      | Description                                                                                                               |
|-----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module       |
| pre-text        | "Resolution: "                               | Text that comes before this module                                                                                        |
| output-style    | "white"                                      | A format string with each word separated by dots that describes the style of the output text                              |

### Desktop Environment Header <a name="desktop_environment_header_configuration"></a>
| Value           | Default                                      | Description                                                                                                               |
|-----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module       |
| pre-text        | "Desktop Environment: "                      | Text that comes before this module                                                                                        |
| output-style    | "white"                                      | A format string with each word separated by dots that describes the style of the output text                              |

### Window Manager Header <a name="window_manager_header_configuration"></a>
| Value           | Default                                      | Description                                                                                                               |
|-----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module       |
| pre-text        | "Window Manager: "                           | Text that comes before this module                                                                                        |
| output-style    | "white"                                      | A format string with each word separated by dots that describes the style of the output text                              |

### Terminal Header <a name="terminal_header_configuration"></a>
| Value           | Default                                      | Description                                                                                                               |
|-----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow"                                | A format string with each word separated by dots that describes the style of the text that comes before this module       |
| pre-text        | "Terminal: "                                 | Text that comes before this module                                                                                        |
| output-style    | "white"                                      | A format string with each word separated by dots that describes the style of the output text                              |

### Custom Header <a name="custom_header_configuration"></a>
| Value          | Description                                                                                                   |
|----------------|---------------------------------------------------------------------------------------------------------------|
| pre-text-style | A format string with each word separated by dots that describes the style of the text that before this module |
| pre-text       | Text that comes before this module                                                                            |
| output-style   | A format string with each word separated by dots that describes the style of the output text                  |
| command        | The command to run to get the output of the module                                                            |

# Default Configuration <a name="default_configuration"></a>
A default Configuration will look like so:
```toml
module-order = "user delimiter os host kernel uptime packages shell resolution desktop-environment window-manager terminal cpu"
offset = 4
logo-cmd = "auto"

[format]
mode = "classic"
top-left-corner-char = "╭"
top-right-corner-char = "╮"
bottom-left-corner-char = "╰"
bottom-right-corner-char = "╯"
horizontal-char = "─"
vertical-char = "│"
padding-left = 1
padding-right = 1
padding-top = 0

[user]
pre-text-style = "bold.yellow"
pre-text = ""
output-style = "bold.yellow"
separator-style = "white"
separator-char = "@"

[delimiter]
style = "white"
repeat-num = 0
char = "-"

[os]
pre-text-style = "bold.yellow"
pre-text = "OS: "
output-style = "white"

[host]
pre-text-style = "bold.yellow"
pre-text = "Host: "
output-style = "white"

[kernel]
pre-text-style = "bold.yellow"
pre-text = "Kernel: "
output-style = "white"

[uptime]
pre-text-style = "bold.yellow"
pre-text = "Uptime: "
output-style = "white"
time-format = "$days days, $hours hours, $minutes minutes"

[packages]
pre-text-style = "bold.yellow"
pre-text = "Packages: "
output-style = "white"

[shell]
pre-text-style = "bold.yellow"
pre-text = "Shell: "
output-style = "white"

[resolution]
pre-text-style = "bold.yellow"
pre-text = "Resolution: "
output-style = "white"

[desktop-environment]
pre-text-style = "bold.yellow"
pre-text = "Desktop Environment: "
output-style = "white"

[window-manager]
pre-text-style = "bold.yellow"
pre-text = "Window Manager: "
output-style = "white"

[terminal]
pre-text-style = "bold.yellow"
pre-text = "Terminal: "
output-style = "white"

[cpu]
pre-text-style = "bold.yellow"
pre-text = "CPU: "
output-style = "white"
```

# Todo <a name="todo"></a>
- [ ] Add more Modules (For starters: the ones that neofetch supports)
