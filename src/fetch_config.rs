use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use custom_error::custom_error;
use regex::Regex;

/// Fetches a config from a provided url share and saves it to a file
pub fn run(arguments: &crate::arguments::FetchArgs) -> i32 {
    if std::path::Path::new(&arguments.config_path).is_dir() {
        eprintln!("[!] Invalid Path provided");
        return 1;
    }

    // Extract the Base64 string from the url
    let Some(base64url) = get_base64_from_url(&arguments.url) else {
        eprintln!("[!] Invalid URL provided");
        return 10;
    };

    // Decode the Base64 string back to UTF8
    let decoded_url = match decode_base64url_to_string(&base64url) {
        Ok(decoded_url) => decoded_url,
        Err(e) => {
            eprintln!("[!] Unable to decode URL: {e}");
            return 11;
        }
    };

    // Download the JSON config from the decoded url
    let mut json_config = match fetch_config_from_url(&decoded_url) {
        Ok(inner_json_config) => inner_json_config,
        Err(e) => {
            eprintln!("[!] Unable to fetch JSON from URL: {e}");
            return 12;
        }
    };

    // Prettify JSON config data unless ugly_json flag was enabled via args
    if !arguments.ugly_json {
        json_config = match jsonxf::pretty_print(&json_config) {
            Ok(prettified_json) => prettified_json,
            Err(e) => {
                eprintln!("[!] Unable to prettify JSON data: {e}");
                return 13;
            }
        }
    }

    // Write JSON data to the provided config path
    match std::fs::write(&arguments.config_path, json_config) {
        Ok(()) => {
            println!(
                "[i] Config file was written to disk: {}",
                arguments.config_path
            );
            0
        }
        Err(e) => {
            eprintln!("[!] Unable to write config file to disk {e}");
            2
        }
    }
}

#[test]
fn test_get_base64_from_url() {
    // Valid url
    let url = "https://www.steelseries.com/deeplink/gg/sonar/config/v1/import?url=aHR0cHM6Ly9jb21tdW5pdHktY29uZmlncy5zdGVlbHNlcmllc2Nkbi5jb20vdjEvZXhhbXBsZV9jb25maWc=";
    let result = Some(String::from(
        "aHR0cHM6Ly9jb21tdW5pdHktY29uZmlncy5zdGVlbHNlcmllc2Nkbi5jb20vdjEvZXhhbXBsZV9jb25maWc=",
    ));
    assert_eq!(get_base64_from_url(url), result);

    // Invalid url, returns None
    let url = "https://www.steelsries.com/deeplink/gg/sonar/config/v1/import?url=abcdefghijkl";
    let result: Option<String> = None;
    assert_eq!(get_base64_from_url(url), result);
}

/// Extracts the Base64 encoded string from the input url
fn get_base64_from_url(url: &str) -> Option<String> {
    #[allow(clippy::unwrap_used)]
    let re = Regex::new(
        r"^https://www.steelseries.com/deeplink/gg/sonar/config/v1/import\?url=(?<base64>.*)$",
    )
    .unwrap();
    let captures = re.captures(url)?;

    Some(String::from(&captures["base64"]))
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_decode_base64url_to_string() {
    // Valid url
    let base64url = String::from(
        "aHR0cHM6Ly9jb21tdW5pdHktY29uZmlncy5zdGVlbHNlcmllc2Nkbi5jb20vdjEvZXhhbXBsZV9jb25maWc=",
    );
    let string = String::from("https://community-configs.steelseriescdn.com/v1/example_config");
    assert_eq!(decode_base64url_to_string(&base64url).unwrap(), string);

    // Invalid url, returns Error
    let base64url = String::from("aHR0cHM6Ly9kaWZmZXJlbnQtZG9tYWluLmNvbS8=");
    let string = String::from("The decoded URL is invalid: https://different-domain.com/");
    assert_eq!(
        decode_base64url_to_string(&base64url)
            .unwrap_err()
            .to_string(),
        string
    );
}

custom_error! {URLDecodeError
    InvalidURL{url: String} = "The decoded URL is invalid: {url}",
}

/// Decodes a Base64 string back to UTF8
fn decode_base64url_to_string(base64: &String) -> Result<String, Box<dyn std::error::Error>> {
    let decoded_bytes: Vec<u8> = URL_SAFE.decode(base64)?;
    let decoded_url = String::from_utf8(decoded_bytes)?;

    // Verifying that the URL is actually pointing to SteelSeries
    if !decoded_url.starts_with("https://community-configs.steelseriescdn.com/v1/") {
        return Err(Box::new(URLDecodeError::InvalidURL { url: decoded_url }));
    }

    Ok(decoded_url)
}

/// Downloads data from a provided url to a string
fn fetch_config_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(ureq::get(url).call()?.into_string()?)
}
