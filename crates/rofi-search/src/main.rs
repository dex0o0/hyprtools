use std::{
    error::Error,
    fs,
    net::IpAddr,
    path::PathBuf,
    process::{Command, ExitCode},
};

use url::Url;

enum InputType {
    Url(String),
    Search(String),
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            notify("Rofi Search", &error.to_string());
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config_path =
        PathBuf::from(std::env::var("HOME")?).join(".config/hypr/UserConfigs/user-defaults.lua");

    let theme_path = PathBuf::from(std::env::var("HOME")?).join(".config/rofi/config-search.rasi");

    let search_engine = read_search_engine(&config_path)?;

    let output = Command::new("rofi")
        .args([
            "-dmenu",
            "-i",
            "-config",
            theme_path.to_str().ok_or("Invalid theme path")?,
            "-mesg",
            "Search the web",
        ])
        .output()?;

    // Rofi returns a non-zero status when the user cancels.
    if !output.status.success() {
        return Ok(());
    }

    let input = String::from_utf8(output.stdout)?;
    let input = input.trim();

    if input.is_empty() {
        return Ok(());
    }

    match classify_input(input) {
        InputType::Url(url) => open_url(&url)?,
        InputType::Search(query) => {
            let url = build_search_url(&search_engine, &query)?;
            open_url(&url)?;
        }
    }

    Ok(())
}

fn read_search_engine(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;

    for line in config.lines() {
        let line = line.trim();

        if line.starts_with("--") {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        if key.trim() != "search_engine" {
            continue;
        }

        let value = value.trim();

        let engine = if let Some(value) = value.strip_prefix('"') {
            value.split('"').next()
        } else if let Some(value) = value.strip_prefix('\'') {
            value.split('\'').next()
        } else {
            None
        };

        if let Some(engine) = engine
            && !engine.is_empty()
        {
            return Ok(engine.to_string());
        }
    }

    Err("Search engine is not configured".into())
}

fn classify_input(input: &str) -> InputType {
    let input = input.trim();

    // Inputs containing whitespace are search queries.
    if input.chars().any(char::is_whitespace) {
        return InputType::Search(input.to_string());
    }

    // Explicit HTTP/HTTPS URLs.
    let lower = input.to_ascii_lowercase();

    if lower.starts_with("http://") || lower.starts_with("https://") {
        if let Ok(url) = Url::parse(input)
            && matches!(url.scheme(), "http" | "https")
            && url.host().is_some()
        {
            return InputType::Url(url.to_string());
        }

        return InputType::Search(input.to_string());
    }

    // Try interpreting a bare address as HTTPS.
    let candidate = format!("https://{input}");

    if let Ok(url) = Url::parse(&candidate)
        && let Some(host) = url.host_str()
    {
        let is_ip = host.parse::<IpAddr>().is_ok();
        let is_localhost = host == "localhost";
        let is_domain = host.contains('.');

        if is_ip || is_localhost || is_domain {
            return InputType::Url(url.to_string());
        }
    }

    InputType::Search(input.to_string())
}

fn build_search_url(engine: &str, query: &str) -> Result<String, Box<dyn Error>> {
    if !engine.contains("{}") {
        return Err("Search engine URL must contain the {} placeholder".into());
    }

    // application/x-www-form-urlencoded encoding:
    // spaces become '+', and special characters are escaped.
    let encoded = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .finish();

    let encoded_query = encoded
        .split_once('=')
        .map(|(_, value)| value)
        .ok_or("Could not encode search query")?;

    let search_url = engine.replace("{}", encoded_query);

    let parsed = Url::parse(&search_url)?;

    if !matches!(parsed.scheme(), "http" | "https") {
        return Err("Search engine must use HTTP or HTTPS".into());
    }

    Ok(search_url)
}

fn open_url(url: &str) -> Result<(), Box<dyn Error>> {
    Command::new("xdg-open").arg(url).spawn()?;

    // if !status.success() {
    //     return Err("Could not open the URL with xdg-open".into());
    // }

    Ok(())
}

fn notify(title: &str, message: &str) {
    let _ = Command::new("notify-send").arg(title).arg(message).status();
}
