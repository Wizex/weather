//! Command-line tool that shows weather.
//!
//! It supports multiple weather providers. You can get weather by address and date.
//! Uses the APIs of providers to get data about weather. Supports both historical and forecast data.
//! Date ranges, address, response and errors depend on a provider. See the API documentation for each supported provider.
//!
//! The project also contains HTTP clients for supported providers.
//! Current supported providers:
//! - OpenWeatherMap
//! - WeatherAPI

use anyhow::Context;
use clap::{Parser, Subcommand};
use config::Config;
use time::macros;

mod config;
mod provider;

pub type Error = anyhow::Result<()>;

/// Command-line parser.
#[derive(Parser)]
#[command(about = "Command-line tool for showing weather")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure credentials for a provider
    Configure {
        #[arg(value_enum)]
        provider: provider::Provider,
    },

    /// Show weather for an address
    Get {
        address: String,

        /// yyyy-MM-dd format
        #[arg(value_parser = parse_date)]
        date: Option<i64>,
    },

    /// Set a current provider
    Set {
        #[arg(value_enum, required = true)]
        provider: provider::Provider,
    },

    /// Return current provider
    Provider,
}

fn parse_date(arg: &str) -> anyhow::Result<i64> {
    let format = macros::format_description!("[year]-[month]-[day]");
    Ok(time::Date::parse(arg, &format)
        .with_context(|| {
            "Date in the wrong format. It has to be in yyyy-MM-dd format and has proper values"
        })?
        .midnight()
        .assume_utc()
        .unix_timestamp())
}

pub fn run() -> Error {
    let cli = Cli::parse();
    let mut config = Config::load()?;
    match cli.command {
        Commands::Get { address, date } => {
            if let Some((provider, key)) = config.get_selected_provider_api_key() {
                if let Some(key) = key {
                    let date =
                        date.unwrap_or_else(|| time::OffsetDateTime::now_utc().unix_timestamp());
                    println!(
                        "{}",
                        provider.get_weather(key, &address, &date.to_string())?
                    );
                } else {
                    println!("Credentials for the provider {provider} are not configured. Configure credentials using the 'configure' command")
                }
            } else {
                println!("No selected provider. Please select a provider using the `set` command")
            }
        }
        Commands::Configure { provider } => {
            let mut api_key = String::new();
            std::io::stdin()
                .read_line(&mut api_key)
                .with_context(|| "Failed to read API key")?;
            api_key.pop();

            config.set_api_key(provider, api_key);
            config.store()?;
            println!("Credentials have been configured successfully")
        }
        Commands::Set { provider } => {
            config.set_selected_provider(Some(provider));
            config.store()?;
            println!("Provider has been selected successfully")
        }
        Commands::Provider => {
            if let Some(provider) = config.get_selected_provider() {
                println!("{provider}");
            } else {
                println!("No selected provider. Select a provider using the `set` command");
            }
        }
    }
    Ok(())
}
