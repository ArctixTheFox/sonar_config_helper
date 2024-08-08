use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "SteelSeries Sonar Config Helper")]
#[command(author = "ArctixTheFox")]
#[command(version = "1.0.2")]
#[command(
    about = "A command-line utility designed to simplify the local management of Sonar configurations
Capable of downloading a JSON configuration to a file given the shared URL
As well as locally host a configuration and generate a URI for importation into Sonar"
)]
#[command(
    help_template = "\n\n{name} \nAuthor: {author-with-newline} {about-section} \nVersion: {version} \n\n{usage-heading} {usage} \n{all-args} \n"
)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Fetch a Sonar configuration from a shared URL
    Fetch(FetchArgs),

    /// Serve a Sonar configuration locally and generate a URI to import it into Sonar
    Serve(ServeArgs),
}

#[derive(Debug, Args)]
pub struct FetchArgs {
    /// Path to save the config file to
    pub config_path: String,

    /// URL of the shared Sonar config
    pub url: String,

    /// Save the JSON config without formatting
    #[arg(long)]
    pub ugly_json: bool,
}

#[derive(Debug, Args)]
pub struct ServeArgs {
    /// Path of the config file to serve
    pub config_path: String,

    /// IP address on which the web server will serve the file
    #[arg(short, long, default_value_t = String::from("localhost"))]
    pub ip_address: String,

    /// Port on which the web server will serve the file
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,
}
