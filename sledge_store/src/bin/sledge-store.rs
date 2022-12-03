use sledge_config::error::Error as ConfigError;
use sledge_config::get_config;
use sledge_store::error::Error as StoreError;
use sledge_store::{create_datastore, get_current_datastore};
use std::fmt::Display;
use structopt::StructOpt;
use tracing::info;
use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::filter::{EnvFilter, LevelFilter, ParseError};
use tracing_subscriber::FmtSubscriber;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Command-Line Structure
// ------------------------------------------------------------------------------------------------

const TOOL_NAME: &str = "sledge-store";

#[derive(Debug, StructOpt)]
#[structopt(name = TOOL_NAME)]
struct Cli {
    /// The level of logging to perform, from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Create something new
    New,
    /// Verify an existing thing
    Verify,
}

// ------------------------------------------------------------------------------------------------
// Command-Line Errors
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum TracingSource {
    EnvFilter(ParseError),
    SetGlobal(SetGlobalDefaultError),
}

impl Display for TracingSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EnvFilter(e) => e.to_string(),
                Self::SetGlobal(e) => e.to_string(),
            }
        )
    }
}

impl From<ParseError> for TracingSource {
    fn from(e: ParseError) -> Self {
        Self::EnvFilter(e)
    }
}

impl From<SetGlobalDefaultError> for TracingSource {
    fn from(e: SetGlobalDefaultError) -> Self {
        Self::SetGlobal(e)
    }
}

impl std::error::Error for TracingSource {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EnvFilter(source) => Some(source),
            Self::SetGlobal(source) => Some(source),
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum ToolError {
    TracingInitFailed(TracingSource),
    ConfigError(String),
    StoreError(StoreError),
}

impl Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TracingInitFailed(e) => e.to_string(),
                Self::ConfigError(e) => e.to_string(),
                Self::StoreError(e) => e.to_string(),
            }
        )
    }
}

impl From<TracingSource> for ToolError {
    fn from(e: TracingSource) -> Self {
        Self::TracingInitFailed(e)
    }
}

impl From<&'static ConfigError> for ToolError {
    fn from(e: &'static ConfigError) -> Self {
        Self::ConfigError(e.to_string())
    }
}

impl From<ConfigError> for ToolError {
    fn from(e: ConfigError) -> Self {
        Self::ConfigError(e.to_string())
    }
}

impl From<StoreError> for ToolError {
    fn from(e: StoreError) -> Self {
        Self::StoreError(e)
    }
}

impl std::error::Error for ToolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TracingInitFailed(source) => Some(source),
            Self::StoreError(source) => Some(source),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Main Function
// ------------------------------------------------------------------------------------------------

fn main() -> Result<(), ToolError> {
    let args = Cli::from_args();

    init_tracing(args.verbose)?;

    let connection_uri = get_config()?.store();

    match args.cmd {
        Command::New => do_init(connection_uri),
        Command::Verify => do_verify(connection_uri),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(verbosity: i8) -> Result<(), ToolError> {
    let log_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive(
            format!("{}={}", module_path!(), log_level)
                .parse()
                .map_err(|e| ToolError::TracingInitFailed(TracingSource::EnvFilter(e)))?,
        )
        .add_directive(
            format!("{}={}", "sledge_config", log_level)
                .parse()
                .map_err(|e| ToolError::TracingInitFailed(TracingSource::EnvFilter(e)))?,
        )
        .add_directive(
            format!("{}={}", "sledge_model", log_level)
                .parse()
                .map_err(|e| ToolError::TracingInitFailed(TracingSource::EnvFilter(e)))?,
        );
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| ToolError::TracingInitFailed(TracingSource::SetGlobal(e)))?;
    info!("Log level set to `LevelFilter::{:?}`", log_level);

    Ok(())
}

fn do_init(connection_uri: &Url) -> Result<(), ToolError> {
    info!(
        "Initializing a new store using connection: <{}>",
        connection_uri
    );

    create_datastore(connection_uri, &Default::default())?;

    Ok(())
}

fn do_verify(connection_uri: &Url) -> Result<(), ToolError> {
    info!(
        "Verifying data store using connection: <{}>",
        connection_uri
    );

    let store = get_current_datastore(connection_uri)?;

    Ok(())
}
