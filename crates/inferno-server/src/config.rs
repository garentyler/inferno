use clap::Arg;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{fs::File, path::Path, path::PathBuf};
use tracing::{error, trace, warn};

/// The globally-accessible static instance of Config.
/// On program startup, Config::load() should be called to initialize it.
pub static CONFIG: OnceCell<Config> = OnceCell::new();

/// The globablly-accessible static instance of Args.
/// On program startup, Args::load() should be called to initialize it.
pub static ARGS: OnceCell<Args> = OnceCell::new();
static DEFAULT_ARGS: Lazy<Args> = Lazy::new(Args::default);

/// Helper function to read a file from a `Path`
/// and return its bytes as a `Vec<u8>`.
#[tracing::instrument]
fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    trace!("{:?}", path);
    let mut data = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// The main server configuration struct.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Config {
    pub port: u16,
    #[serde(skip)]
    pub server_version: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            port: 3000,
            server_version: inferno_protocol::util::get_version("server"),
        }
    }
}
impl Config {
    pub fn instance() -> &'static Self {
        match CONFIG.get() {
            Some(a) => a,
            None => Self::load(),
        }
    }
    #[tracing::instrument]
    pub fn load() -> &'static Self {
        trace!("Config::load()");
        let args = Args::instance();
        let mut config = Config::default();
        let config_path = Path::new(&args.config_file);

        if !config_path.exists() {
            warn!(
                "Configuration file does not exist, creating {}",
                config_path.to_str().unwrap_or("")
            );
            config.write(config_path);
        }

        if let Ok(cfg) = read_file(config_path) {
            let cfg: Result<Config, _> = toml::from_str(&String::from_utf8_lossy(&cfg));
            if let Ok(cfg) = cfg {
                config = cfg;
            } else {
                error!("Could not parse configuration file, using default");
            }
        } else {
            error!("Could not read configuration file, using default");
        }

        CONFIG.set(config).expect("could not set CONFIG");
        Self::instance()
    }
    #[tracing::instrument]
    fn write(&self, path: &Path) {
        trace!("Config.write()");
        if let Ok(mut file) = File::options().write(true).create(true).open(path) {
            if file
                .write_all(toml::to_string(&self).unwrap().as_bytes())
                .is_ok()
            {
                return;
            }
        }
        error!("Could not write configuration file");
        std::process::exit(1);
    }
}

/// All of the valid command line arguments for the inferno binary.
///
/// Arguments will always override the config options specified in `inferno.toml` or `Config::default()`.
#[derive(Debug)]
pub struct Args {
    config_file: PathBuf,
    pub log_level: Option<tracing::Level>,
    pub log_dir: PathBuf,
}
impl Default for Args {
    fn default() -> Self {
        let config = Config::default();
        Args {
            config_file: PathBuf::from("inferno.toml"),
            log_level: None,
            log_dir: PathBuf::from("logs"),
        }
    }
}
impl Args {
    pub fn instance() -> &'static Self {
        match ARGS.get() {
            Some(a) => a,
            None => Self::load(),
        }
    }
    pub fn load() -> &'static Self {
        ARGS.set(Self::parse()).expect("could not set ARGS");
        Self::instance()
    }
    fn parse() -> Self {
        use std::ffi::OsStr;

        let m = clap::Command::new("inferno-server")
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .disable_version_flag(true)
            .arg(
                Arg::new("version")
                    .short('V')
                    .long("version")
                    .help("Print version")
                    .global(true)
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Set log level to debug")
                    .global(true)
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("config-file")
                    .short('c')
                    .long("config-file")
                    .help("Configuration file path")
                    .value_hint(clap::ValueHint::FilePath)
                    .default_value(OsStr::new(&DEFAULT_ARGS.config_file)),
            )
            .arg(
                Arg::new("log-level")
                    .short('l')
                    .long("log-level")
                    .help("Set the log level")
                    .conflicts_with("verbose")
                    .value_name("level")
                    .value_parser(["trace", "debug", "info", "warn", "error"]),
            )
            .arg(
                Arg::new("log-dir")
                    .long("log-dir")
                    .help("Set the log output directory")
                    .value_name("dir")
                    .value_hint(clap::ValueHint::DirPath)
                    .default_value(OsStr::new(&DEFAULT_ARGS.log_dir)),
            )
            .get_matches();

        let mut args = Self::default();
        args.config_file = m
            .get_one::<String>("config-file")
            .map_or(args.config_file, PathBuf::from);
        args.log_dir = m
            .get_one::<String>("log-dir")
            .map_or(args.log_dir, PathBuf::from);

        if m.get_flag("verbose") {
            args.log_level = Some(tracing::Level::DEBUG);
        } else {
            args.log_level = m.get_one("log-level").map_or(args.log_level, |s: &String| {
                Some(s.parse::<tracing::Level>().unwrap())
            });
        }

        if m.get_flag("version") {
            println!("{}", Config::default().server_version);
            if m.get_flag("verbose") {
                println!("release: {}", env!("CARGO_PKG_VERSION"));
                println!("commit-hash: {}", env!("GIT_HASH"));
                println!("commit-date: {}", &env!("GIT_DATE")[0..10]);
                println!("license: {}", env!("CARGO_PKG_LICENSE"));
                println!("authors: {}", env!("CARGO_PKG_AUTHORS"));
                println!("build-target: {}", env!("BUILD_TARGET"));
            }
            std::process::exit(0);
        }

        args
    }
}
