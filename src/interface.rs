use clap::{ArgEnum, Parser, Subcommand, ValueHint};

/// ea - Making command line output actionable
///
/// This tool captures paths from command line tools, and let you follow them up with actions.
/// When you run a command line tool through `ea run`, its output containing file paths will be
/// prefixed with a NUMBER. Afterwards, `ea print NUMBER` will output the associated path so that
/// you can use it as part of other commands (e.g. `vim $(ea p 42)` to open path #42 in vim).
///
/// For more details, see `man ea`, or documentation at https://github.com/dduan/ea
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Interface {
    #[clap(subcommand)]
    pub subcommand: Option<Commands>,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Style {
    Grouped,
    Linear,
    Search,
    Rust,
    Py,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run EXECUTABLE through `ea`. Expecting its output to be the format of STYLE. Arguments for
    /// EXECUTABLE must come after `--`. For example, `rg Vec src` becomes:
    ///
    /// ea run grouped rg -- Vec src
    ///
    /// (rg's default output is in the "grouped" STYLE).
    Run {
        /// Format of output from EXECUTABLE. ea looks for file paths, lines, and columns within
        /// the file at the path. A file path can have one or more "locations". A location has at
        /// least a file path, and a line number, with an optional column.
        ///
        /// [grouped]: A file path followed by a list of locations, then the next file path and its
        ///            locations, etc. Example: ripgrep's default output format.
        ///
        /// [linear]:  An location on each line. Example: fd/find's default output format.
        ///
        /// [search]:  Locations at the start of line, with additional content on the same line,
        ///            followed by lots of other content, followed by another location. Example:
        ///            clang/swift's default format.
        ///
        /// For more explanation, see `man ea`, or documentation at http://github.com/dduan/ea
        #[clap(arg_enum)]
        style: Style,
        #[clap(value_hint = ValueHint::CommandName)]
        /// The command to execute.
        executable: String,
        #[clap(last = true)]
        /// Arguments for EXECUTABLE. Must be separated from EXECUTABLE with `--` (two dashes).
        arguments: Vec<String>,
        /// Write debug info at <debug_files_base_name.*>
        #[clap(long, value_name = "debug_files_base_name", value_hint = ValueHint::FilePath)]
        debug: Option<String>,
    },

    /// List locations found from the latest `ea run` output. This is the default subcommand.
    /// Running `ea` is the same as running `ea list`.
    List,

    /// Print the location info associated with NUMBER. Optionally, customize the output FORMAT.
    /// Also availble as the shorthand `ea p ...`
    #[clap(alias("p"))]
    Print {
        /// The number associated with a file location from the latest `ea run` output.
        #[clap(required = true)]
        number: usize,
        /// A string representing the format of the location to be printed. `{path}`, `{line}`, and `{column}`
        /// in this string will be replaced with corresponding values within the location. For
        /// example, 'L{line}C{column} @ {path}' might print out 'L23C11 @ path/to/file'. If line
        /// or column info isn't available, they'll be filled with '0'.
        #[clap(default_value = "{path}")]
        format: String,
    },
}
