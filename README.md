<div align="center"> 
<h1>Rocketfetch</h1>
A *WIP* command line system information tool written in rust for performance with toml file configuration.
</div>

# Table of Contents
- [Inspiration](#inspiration)
- [Configuration](#configuration)
  - [Top Level](#top-level-configuration)
  - [Format Header](#format-header-configuration)
  - [User Header](#user-header-configuration)
  - [Delimiter Header](#delimiter-header-configuration)
  - [OS Header](#os-header-configuration)
- [Default Configuration](#default-configuration)
- [Todo](#todo)


# Inspiration <a name="inspiration"></a>
_Rocketfetch_ was inspired by [neofetch](https://github.com/dylanaraps/neofetch). I wanted to add some performance to neofetch while also improving the configuration. I modeled my configuration after that of [Starship](https://github.com/starship/starship).

# Configuration <a name="configuration"></a>
You can configure rocketfetch either through the default path of ~/.config/rocketfetch or pass a path in cli by specifying the -c option.
### Top Level <a name="top-level-configuration"></a>
| Value        | Default             | Description                                                                                                                                                                                                              |
|--------------|---------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| module-order | "user delimiter os" | Specify the order in which the modules will go in seperated by spaces. Possible modules are: user, delimiter, os, as well as any custom modules you have defined. You may specify some more than once. More coming soon. |
| offset       | 4                   | Specify the number of spaces between the logo and the information.                                                                                                                                                       |
| logo-cmd     | "auto"              | The command to run in order to get the logo. If set to auto or nothing, it will automatically detect the operating system and choose a logo                                                                              |

### Format Header <a name="format-header-configuration"></a>
| Value                    | Default   | Description                                                                                                                                                           |
|--------------------------|-----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| mode                     | "classic" | Set the mode of output. Could be one of classic (neofetch style), side-block (with a block around the info), or bottom-block (with the logo above the block of info). |
| top-left-corner-char     | "╭"       | Set the character to use as the top left corner of the block (Only works in side-block and bottom-block modes).                                                       |
| top-right-corner-char    | "╮"       | Set the character to use as the top right corner of the block (Only works in side-block and bottom-block modes).                                                      |
| bottom-left-corner-char  | "╰"       | Set the character to use as the bottom left corner of the block (Only works in side-block and bottom-block modes).                                                    |
| bottom-right-corner-char | "╯"       | Set the character to use as the bottom right corner of the block (Only works in side-block and bottom-block modes).                                                   |
| horizontal-char          | "─"       | Set the character to use as the top and bottom parts of the block (Only works in side-block and bottom-block modes).                                                  |
| vertical-char            | "│"       | Set the character to use as the right and left parts of the block (Only works in side-block and bottom-block modes).                                                  |
| padding-left             | 1         | Set the number of spaces to put before each module inside the block (Only works in side-block and bottom-block modes).                                                |
| padding-right            | 1         | Set the number of spaces to put after each module inside the block (Only works in side-block and bottom-block modes).                                                 |
| padding-top              | 0         | Set the number of lines to put above the modules inside the block (Only works in side-block and bottom-block modes).                                                  |
| padding-bottom           | 0         | Set the number of lines to put below the modules inside the block (Only works in side-block and bottom-block modes).                                                  |

### User Header <a name="user-header-configuration"></a>
| Value           | Default       | Description                                                                                                             |
|-----------------|---------------|-------------------------------------------------------------------------------------------------------------------------|
| pre-text-style  | "bold.yellow" | A format string with each word seperated by dots that describes the style of the text that comes before this module     |
| pre-text        | ""            | Text that comes before this module                                                                                      |
| output-style    | "white"       | A format string with each word seperated by dots that describes the style of the output text                            |
| seperator-style | "white"       | A format string with each word seperated by dots that describes the style of the sperator between username and hostname |
| seperator-char  | "@"           | A character that seperates between the username and hostname                                                            |

### Delimiter Header <a name="delimiter-header-configuration"></a>
| Value      | Default | Description                                                                                                                                                   |
|------------|---------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
| style      | "white" | A format string with each word separated by dots that describes the style of the delimiter                                                                    |
| repeat-num | 0       | The number of times to repeat the delimiter char to form the delimiter. If set to 0, it will set the repeat number to the length of the module above this one |
| char       | "-"     | The character to use as the delimiter                                                                                                                         |

### OS Header <a name="os-header-configuration"></a>
| Value          | Default       | Description                                                                                                         |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| pre-text-style | "bold.yellow" | A format string with each word seperated by dots that describes the style of the text that comes before this module |
| pre-text       | "OS: "        | Text that comes before this module                                                                                  |
| output-style   | "white"       | A format string with each word seperated by dots that describes the style of the output text                        |

### Custom Header <a name="custom-header-configuration"></a>
| Value          | Description                                                                                                         |
|----------------|---------------------------------------------------------------------------------------------------------------------|
| pre-text-style | A format string with each word seperated by dots that describes the style of the text that comes before this module |
| pre-text       | Text that comes before this module                                                                                  |
| output-style   | A format string with each word seperated by dots that describes the style of the output text                        |
| Command        | The command to run to get the output of the module                                                                  |

# Default Configuration <a name="default-configuration"></a>
A default Configuration will look like so:

```toml
module-order = "user delimiter os"
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
padding-bottom = 0

[user]
pre-text-style = "bold.yellow"
pre-text = ""
output-style = "bold.yellow"
seperator-style = "white"
seperator-char = "@"

[delimiter]
style = "white"
repeat-num = 0
char = "-"

[os]
pre-text-style = "bold.yellow"
pre-text = "OS: "
output-style = "white"
```

# Todo <a name="todo"></a>
- Add more Modules
