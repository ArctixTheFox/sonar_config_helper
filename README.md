# SteelSeries Sonar Config Helper
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![license MIT](https://img.shields.io/badge/license-MIT-blue)](https://choosealicense.com/licenses/mit/)

The SteelSeries Sonar Config Helper is a command-line utility designed to simplify the local management of Sonar configurations. This project exists because Sonar itself does not provide an easy way of saving a config to disk, or importing a config from disk.

## Installation
Feel free to either download one of the executables from the Releases, or compile the project yourself.

### Compiling from source
Make sure to install the target toolchain using `rustup` before proceeding
```bash
git clone https://github.com/
cd sonar_config_helper
cargo build -r
```

The compiled executable should be located in the following directory:
`sonar_config_helper/target/release/`

## Usage
### Commands
- **fetch**: Fetch a Sonar configuration from a shared URL.
- **serve**: Serve a Sonar configuration locally and generate a URI to import it into Sonar.
- **help** : Displays a help message.

### Fetch Command
The `fetch` command downloads a Sonar configuration from a URL that is usually shared from inside Sonar.
The example provided below is taken from the SteelSeries Community Server.
Example URL: https://www.steelseries.com/deeplink/gg/sonar/config/v1/import?url=aHR0cHM6Ly9jb21tdW5pdHktY29uZmlncy5zdGVlbHNlcmllc2Nkbi5jb20vdjEvMjViYzk4MTYtMTZhMS00ZGVjLWIxY2ItZDMwNDdkZjFlY2Ex

Usage:
```bash
sonar_config_helper.exe fetch [OPTIONS] <CONFIG_PATH> <URL>
```

Arguments:
- `<CONFIG_PATH>` Path to save the config file to.
- `<URL>`        URL of the shared Sonar config.

Options:
- `--ugly-json`   Save the JSON config without formatting.
- `-h, --help`    Display help message for the `fetch` command.

### Serve Command
The `serve` command hosts a Sonar configuration locally and generates a URI for importation into Sonar.

Usage:
```bash
sonar_config_helper.exe serve [OPTIONS] <CONFIG_PATH>
```

Arguments:
- `<CONFIG_PATH>` Path of the config file to serve.

Options:
- `-i, --ip-address <IP_ADDRESS>` IP address on which the web server will serve the file. [Default: localhost]
- `-p, --port <PORT>` Port on which the web server will serve the file. [Default: 0]
- `-h, --help`       Display help message for the `serve` command.

## Contributing
Contributions of any kind are welcome!
If you have ideas for improvements or find any issues, feel free to open an issue or submit a pull request.

## License
This project is licensed under the [MIT License](https://choosealicense.com/licenses/mit/)
