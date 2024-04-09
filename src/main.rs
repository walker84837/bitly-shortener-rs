use crate::consts::*;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs::File, io::Read};
use tokio;

mod consts;

#[derive(Parser)]
enum Command {
    Shorten {
        #[arg(help = "Long URL to shorten")]
        long_url: String,

        #[arg(short, long, help = "Print the response as JSON")]
        verbose: bool,
    },
    Create {
        #[arg(help = "Long URL to create a Bitlink for")]
        long_url: String,

        #[arg(long = "title", help = "Title for the Bitlink (optional)")]
        title: Option<String>,

        #[arg(
            long = "tags",
            help = "Comma-separated tags for the Bitlink (optional)"
        )]
        tags: Option<String>,

        #[arg(short, long, help = "Print the response as JSON")]
        verbose: bool,
    },
    Delete {
        #[arg(help = "Bitlink ID to delete")]
        bitlink: String,

        #[arg(short, long, help = "Print the response as JSON")]
        verbose: bool,
    },
    Update {
        #[arg(help = "Bitlink to update")]
        bitlink: String,

        #[arg(long = "title", help = "New title for the Bitlink (optional)")]
        title: Option<String>,

        #[arg(
            long = "tags",
            help = "Comma-separated tags for the Bitlink (optional)"
        )]
        tags: Option<String>,

        #[arg(short, long, help = "Print the response as JSON")]
        verbose: bool,
    },
    Retrieve {
        #[arg(help = "Bitlink to retrieve")]
        bitlink: String,

        #[arg(short, long, help = "Print the response as JSON")]
        verbose: bool,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct BitlyResponse {
    references: serde_json::Value,
    link: String,
    id: String,
    long_url: String,
    archived: bool,
    created_at: String,
    custom_bitlinks: Vec<String>,
    tags: Vec<String>,
    deeplinks: Vec<Deeplink>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Deeplink {
    guid: String,
    bitlink: String,
    app_uri_path: String,
    install_url: String,
    app_guid: String,
    os: String,
    install_type: String,
    created: String,
    modified: String,
    brand_guid: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Command::parse();

    let token = read_api_token()?;
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    match args {
        Command::Shorten { long_url, verbose } => {
            let url = format!("{}/shorten", BASE_URL);
            let request_body = json!({
                "long_url": long_url,
            });

            let response = client
                .post(&url)
                .headers(headers)
                .json(&request_body)
                .send()
                .await?;
            if response.status().is_success() {
                let result: BitlyResponse = response.json().await?;
                if !verbose {
                    println!("Bitlink: {}", result.link);
                } else {
                    println!("{:#?}", result);
                }
            } else {
                return Err(anyhow!(
                    "Failed to shorten link at {long_url}: {}",
                    response.status()
                ));
            }
        }
        Command::Create {
            long_url,
            title,
            tags,
            verbose,
        } => {
            let url = format!("{}/bitlinks", BASE_URL);
            let mut request_body = json!({
                "long_url": long_url,
            });

            if let Some(ref title) = title {
                request_body["title"] = json!(title);
            }
            if let Some(ref tags) = tags {
                request_body["tags"] = json!(tags.split(',').collect::<Vec<_>>());
            }

            let response = client
                .post(&url)
                .headers(headers.clone())
                .json(&request_body)
                .send()
                .await?;

            if response.status().is_success() {
                let result: BitlyResponse = response.json().await?;
                if !verbose {
                    println!("Bitlink: {}", result.link);
                } else {
                    println!("{:#?}", result);
                }
            } else {
                return Err(anyhow!(
                    "Failed to create Bitlink from link '{:?}', with tags '{:?}', and title '{:?}': {}",
                    long_url, tags, title, &response.status()
                ));
            }
        }
        Command::Delete { bitlink, verbose } => {
            let url = format!("{}/bitlinks/{}", BASE_URL, bitlink);

            let response = client.delete(&url).headers(headers).send().await?;
            if response.status().is_success() {
                let result: BitlyResponse = response.json().await?;
                if !verbose {
                    println!("Bitlink: {}", result.link);
                } else {
                    println!("{:#?}", result);
                }
            } else {
                return Err(anyhow!(
                    "Failed to delete Bitlink ({bitlink}): {}",
                    response.status()
                ));
            }
        }
        Command::Update {
            bitlink,
            title,
            tags,
            verbose,
        } => {
            let url = format!("{}/bitlinks/{}", BASE_URL, bitlink);
            let mut request_body = json!({});

            if let Some(ref title) = title {
                request_body["title"] = json!(title);
            }
            if let Some(ref tags) = tags {
                request_body["tags"] = json!(tags.split(',').collect::<Vec<_>>());
            }

            let response = client
                .patch(&url)
                .headers(headers.clone())
                .json(&request_body)
                .send()
                .await?;

            if response.status().is_success() {
                let result: BitlyResponse = response.json().await?;
                if !verbose {
                    println!("Bitlink '{bitlink}' updated successfully with title '{:?}' and tags '{:?}'!", title, tags);
                } else {
                    println!("{:#?}", result);
                }
            } else {
                return Err(anyhow!(
                    "Failed to update Bitlink ({}): {}",
                    bitlink,
                    response.status()
                ));
            }
        }
        Command::Retrieve { bitlink, verbose } => {
            let url = format!("{}/bitlinks/{}", BASE_URL, bitlink);

            let response = client.get(&url).headers(headers).send().await?;

            if response.status().is_success() {
                let result: BitlyResponse = response.json().await?;
                if !verbose {
                    println!("Retrieved Bitlink: {:?}", result.link);
                } else {
                    println!("{:#?}", result);
                }
            } else {
                return Err(anyhow!(
                    "Failed to retrieve Bitlink ({}): {}",
                    bitlink,
                    response.status()
                ));
            }
        }
    }

    Ok(())
}

fn read_api_token() -> Result<String> {
    let mut token = String::new();
    let env_var = env::var(ENV_VARIABLE).ok();

    if let Some(variable) = env_var {
        token = variable;
    } else {
        let mut file = File::open(TOKEN_FILE)?;
        file.read_to_string(&mut token)
            .with_context(|| "Failed to get API file token.")?;
    }

    Ok(token.trim().to_string())
}
