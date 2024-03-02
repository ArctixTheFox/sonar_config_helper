use base64::{engine::general_purpose::URL_SAFE, Engine as _};

/// Serves a provided config file using http
pub fn run(arguments: &crate::arguments::ServeArgs) -> i32 {
    if !std::path::Path::new(&arguments.config_path).is_file() {
        eprintln!("[!] Invalid Path provided");
        return 1;
    }

    // Reading data from the input file to a string
    let json = match std::fs::read_to_string(&arguments.config_path) {
        Ok(file_content) => file_content,
        Err(e) => {
            eprintln!("[!] Unable to read config file: {e}");
            return 2;
        }
    };

    // Starting the http server on the supplied IP and Port
    // Default IP: 127.0.0.1
    // Default Port: 0 (Requests an available port from the OS)
    let server =
        match tiny_http::Server::http(format!("{}:{}", arguments.ip_address, arguments.port)) {
            Ok(server) => server,
            Err(e) => {
                eprintln!("[!] Unable to start http server: {e}");
                return 20;
            }
        };

    let url = format!("http://{}/steelseries_config", server.server_addr());

    // Encoding the url to Base64 (url safe)
    let mut encoded_url = String::new();
    URL_SAFE.encode_string(url, &mut encoded_url);

    println!("Server started, press CTRL + C to exit\n");
    println!("URI: ssgg://gg/sonar/config/v1/import?url={encoded_url}");
    println!("\x1B]8;;ssgg://gg/sonar/config/v1/import?url={encoded_url}\x1B\\CTRL + Click here to open\x1B]8;;\x1B\\\n");

    // Start handling requests for the http server
    for request in server.incoming_requests() {
        // Default response (for incorrect urls)
        let mut response =
            tiny_http::Response::from_string("418 I'm a teapot").with_status_code(418);

        // Checking if the url is correct
        let config_url_requested = request.url() == "/steelseries_config";
        if config_url_requested {
            // Change the response content to contain the config
            response = tiny_http::Response::from_string(json.clone());
        }

        // Send the response
        match request.respond(response) {
            Ok(()) => {
                if config_url_requested {
                    println!("[i] Config served");
                }
            }
            Err(e) => {
                eprintln!("[!] Unable to send response: {e}");
            }
        }
    }

    0
}
