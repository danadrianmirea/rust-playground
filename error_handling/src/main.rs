use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
}

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    ParseInt(ParseIntError),
    MissingField(&'static str),
    InvalidPort { raw: i64 },
    DivisionByZero,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(_) => write!(f, "I/O error"),
            AppError::ParseInt(_) => write!(f, "failed to parse integer"),
            AppError::MissingField(field) => write!(f, "missing required field: {field}"),
            AppError::InvalidPort { raw } => write!(f, "invalid port: {raw} (expected 1..=65535)"),
            AppError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::ParseInt(e) => Some(e),
            AppError::MissingField(_) => None,
            AppError::InvalidPort { .. } => None,
            AppError::DivisionByZero => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}

impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        AppError::ParseInt(value)
    }
}

fn parse_config_str(s: &str) -> Result<Config, AppError> {
    let mut host: Option<String> = None;
    let mut port_raw: Option<i64> = None;

    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (k, v) = line
            .split_once('=')
            .map(|(k, v)| (k.trim(), v.trim()))
            .unwrap_or((line, ""));

        match k {
            "host" => host = Some(v.to_string()),
            "port" => {
                // Demonstrates ParseIntError -> AppError via `?`.
                port_raw = Some(v.parse::<i64>()?);
            }
            _ => {}
        }
    }

    let host = host.ok_or(AppError::MissingField("host"))?;
    let raw = port_raw.ok_or(AppError::MissingField("port"))?;
    let port: u16 = raw
        .try_into()
        .ok()
        .filter(|p| (1..=65535).contains(p))
        .ok_or(AppError::InvalidPort { raw })?;

    Ok(Config { host, port })
}

fn load_config_from_file(path: &str) -> Result<Config, AppError> {
    // Demonstrates std::io::Error -> AppError via `From` + `?`.
    let contents = fs::read_to_string(path)?;
    parse_config_str(&contents)
}

fn parse_port_env() -> Result<u16, AppError> {
    // Demonstrates mapping a non-std error into your app error.
    let raw = env::var("APP_PORT").map_err(|_| AppError::MissingField("APP_PORT env var"))?;
    let parsed: i64 = raw.parse::<i64>()?;
    let port: u16 = parsed
        .try_into()
        .ok()
        .filter(|p| (1..=65535).contains(p))
        .ok_or(AppError::InvalidPort { raw: parsed })?;
    Ok(port)
}

fn safe_div(a: i64, b: i64) -> Result<i64, AppError> {
    if b == 0 {
        return Err(AppError::DivisionByZero);
    }
    Ok(a / b)
}

fn print_error_chain(e: &(dyn Error)) {
    eprintln!("error: {e}");
    let mut cur = e.source();
    let mut depth = 0;
    while let Some(src) = cur {
        depth += 1;
        eprintln!("  caused by ({depth}): {src}");
        cur = src.source();
    }
}

fn run(name: &str, f: impl FnOnce() -> Result<(), AppError>) {
    println!("\n== {name} ==");
    match f() {
        Ok(()) => println!("ok"),
        Err(e) => print_error_chain(&e),
    }
}

fn main() {
    run("success: parse config string", || {
        let cfg = parse_config_str(
            r#"
            # example config
            host = 127.0.0.1
            port = 8080
            "#,
        )?;
        println!("config parsed: host={} port={}", cfg.host, cfg.port);
        Ok(())
    });

    run("error: missing field", || {
        let _ = parse_config_str("host=localhost")?;
        Ok(())
    });

    run("error: parse int", || {
        let _ = parse_config_str("host=localhost\nport=not_a_number")?;
        Ok(())
    });

    run("error: validation (out of range port)", || {
        let _ = parse_config_str("host=localhost\nport=70000")?;
        Ok(())
    });

    run("error: I/O (missing file)", || {
        let _ = load_config_from_file("definitely-not-here.conf")?;
        Ok(())
    });

    run("error: env var missing / invalid", || {
        let port = parse_port_env()?;
        println!("APP_PORT parsed: {port}");
        Ok(())
    });

    run("error: domain (division by zero)", || {
        let _ = safe_div(10, 0)?;
        Ok(())
    });
}
