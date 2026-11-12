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
            "-p",
            "Search",
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

    let mut inside_multiline_comment = false;

    for line in config.lines() {
        let line = line.trim();

        if inside_multiline_comment {
            if line.contains("]]") {
                inside_multiline_comment = false;
            }
            continue;
        }

        if line.starts_with("--[[") {
            if !line.contains("]]") {
                inside_multiline_comment = true;
            }
            continue;
        }

        if line.starts_with("--") || line.is_empty() {
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

// ==================
// tests
// ==================

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_file_path() -> PathBuf {
        let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);

        std::env::temp_dir().join(format!("rofi-search-test-{}-{id}.lua", std::process::id()))
    }

    fn create_temp_config(content: &str) -> PathBuf {
        let path = temp_file_path();
        fs::write(&path, content).expect("Failed to create temporary config");
        path
    }

    // --------------------------------------------------
    // classify_input tests
    // --------------------------------------------------

    #[test]
    fn classifies_http_url() {
        let result = classify_input("http://example.com");

        assert!(matches!(
            result,
            InputType::Url(url) if url == "http://example.com/"
        ));
    }

    #[test]
    fn classifies_https_url() {
        let result = classify_input("https://github.com");

        assert!(matches!(
            result,
            InputType::Url(url) if url == "https://github.com/"
        ));
    }

    #[test]
    fn classifies_bare_domain_as_url() {
        let result = classify_input("github.com");

        assert!(matches!(
            result,
            InputType::Url(url) if url == "https://github.com/"
        ));
    }

    #[test]
    fn classifies_ip_address_as_url() {
        let result = classify_input("192.168.1.1");

        assert!(matches!(
            result,
            InputType::Url(url) if url == "https://192.168.1.1/"
        ));
    }

    #[test]
    fn classifies_localhost_as_url() {
        let result = classify_input("localhost");

        assert!(matches!(
            result,
            InputType::Url(url) if url == "https://localhost/"
        ));
    }

    #[test]
    fn classifies_search_query() {
        let result = classify_input("rust programming language");

        assert!(matches!(
            result,
            InputType::Search(query)
                if query == "rust programming language"
        ));
    }

    #[test]
    fn classifies_single_word_as_search() {
        let result = classify_input("rust");

        assert!(matches!(
            result,
            InputType::Search(query) if query == "rust"
        ));
    }

    #[test]
    fn classifies_invalid_explicit_url_as_search() {
        let result = classify_input("https://");

        assert!(matches!(
            result,
            InputType::Search(query) if query == "https://"
        ));
    }

    #[test]
    fn trims_input_before_classification() {
        let result = classify_input("  github.com  ");

        assert!(matches!(
            result,
            InputType::Url(url) if url == "https://github.com/"
        ));
    }

    // --------------------------------------------------
    // build_search_url tests
    // --------------------------------------------------

    #[test]
    fn builds_search_url() {
        let result = build_search_url("https://www.google.com/search?q={}", "rust programming");

        assert_eq!(
            result.unwrap(),
            "https://www.google.com/search?q=rust+programming"
        );
    }

    #[test]
    fn encodes_persian_search_query() {
        let result =
            build_search_url("https://www.google.com/search?q={}", "برنامه نویسی Rust").unwrap();

        let parsed = Url::parse(&result).unwrap();

        let query = parsed
            .query_pairs()
            .find(|(key, _)| key == "q")
            .map(|(_, value)| value.into_owned());

        assert_eq!(query, Some("برنامه نویسی Rust".to_string()));
    }

    #[test]
    fn encodes_special_characters() {
        let result = build_search_url("https://example.com/search?q={}", "rust & cargo").unwrap();

        let parsed = Url::parse(&result).unwrap();

        let query = parsed
            .query_pairs()
            .find(|(key, _)| key == "q")
            .map(|(_, value)| value.into_owned());

        assert_eq!(query, Some("rust & cargo".to_string()));
    }

    #[test]
    fn rejects_engine_without_placeholder() {
        let result = build_search_url("https://example.com/search", "rust");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_http_search_engine() {
        let result = build_search_url("file:///search?q={}", "rust");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_malformed_search_url() {
        let result = build_search_url("https://[invalid]/search?q={}", "rust");

        assert!(result.is_err());
    }

    // --------------------------------------------------
    // read_search_engine tests
    // --------------------------------------------------

    #[test]
    fn reads_double_quoted_search_engine() {
        let path = create_temp_config(r#"search_engine = "https://www.google.com/search?q={}""#);

        let result = read_search_engine(&path);

        fs::remove_file(&path).unwrap();

        assert_eq!(result.unwrap(), "https://www.google.com/search?q={}");
    }

    #[test]
    fn reads_single_quoted_search_engine() {
        let path = create_temp_config("search_engine = 'https://duckduckgo.com/?q={}'");

        let result = read_search_engine(&path);

        fs::remove_file(&path).unwrap();

        assert_eq!(result.unwrap(), "https://duckduckgo.com/?q={}");
    }

    #[test]
    fn ignores_commented_search_engine() {
        let path = create_temp_config(
            r#"
            -- search_engine = "https://example.com/?q={}"
            search_engine = "https://google.com/search?q={}"
            "#,
        );

        let result = read_search_engine(&path);

        fs::remove_file(&path).unwrap();

        assert_eq!(result.unwrap(), "https://google.com/search?q={}");
    }

    #[test]
    fn rejects_empty_search_engine() {
        let path = create_temp_config(r#"search_engine = """#);

        let result = read_search_engine(&path);

        fs::remove_file(&path).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_search_engine() {
        let path = create_temp_config(r#"some_other_setting = "value""#);

        let result = read_search_engine(&path);

        fs::remove_file(&path).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_config_file() {
        let path = temp_file_path();

        let result = read_search_engine(&path);

        assert!(result.is_err());
    }

    #[test]
    fn ignores_multiline_commented_search_engine() {
        let path = create_temp_config(
            r#"
        --[[
        search_engine = "https://example.com/?q={}"
        ]]

        search_engine = "https://google.com/search?q={}"
        "#,
        );

        let result = read_search_engine(&path);

        fs::remove_file(&path).unwrap();

        assert_eq!(result.unwrap(), "https://google.com/search?q={}");
    }
}
