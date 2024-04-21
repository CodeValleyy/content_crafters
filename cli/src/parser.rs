pub use clap::Parser;

/// This code snippet defines a Rust struct `CliSApiArgs` with several fields. It implements the `Parser`, `Debug`, and `Clone` traits. The struct has the following fields:

/// - `port`: The port number the server listens on. It is of type `u16` and has a default value of `8080`. It can be set using the `-P` or `--port` command-line options.
/// - `verbose`: A flag to enable/disable logging. It is of type `u8` and has a default value of `0`. It can be set using the `-v` or `--verbose` command-line options. The flag can be repeated to increase the verbosity level.
/// - `debug`: A flag to enable/disable debug mode. It is of type `u8` and has a default value of `0`. It can be set using the `-d` or `--debug` command-line options. The flag can be repeated to increase the debug level.
/// - `trace`: A flag to enable/disable trace mode. It is of type `u8` and has a default value of `0`. It can be set using the `-t` or `--trace` command-line options. The flag can be repeated to increase the trace level.

/// This struct is used for parsing command-line arguments using the `clap` crate.
#[derive(Parser, Debug, Clone)]
pub struct CliApiArgs {
    /// The port number the server listens on.
    /// Default: 8787
    #[clap(short = 'P', long = "port", default_value = "8080")]
    pub port: u16,

    /// Optional: Add a flag to enable/disable logging.
    /// Default: 0
    #[clap(short = 'v', long = "verbose", env = "VERBOSE", action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Optional: Add a flag to enable/disable debug mode.
    /// Default: 0
    #[clap(short = 'd', long = "debug", env = "DEBUG", action = clap::ArgAction::Count)]
    pub debug: u8,

    /// Optional: Add a flag to enable/disable trace mode.
    /// Default: 0
    #[clap(short = 't', long = "trace", env = "TRACE", action = clap::ArgAction::Count)]
    pub trace: u8,
}