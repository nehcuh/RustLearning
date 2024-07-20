use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use clap::Parser;
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response};

/// A simple naive httpie implemented with rust
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// Feed Get an url and we'll retrieve response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser=parse_url)]
    url: String,
}

/// Feed Post an url and optional key=value pairs, we'll post body as JSON and retrieve response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[arg(value_parser=parse_url)]
    url: String,
    /// HTTP 请求的 Body
    #[arg(value_parser=parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Clone, Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse::<KvPair>()?)
}

fn parse_url(s: &str) -> Result<String> {
    Ok((s.parse::<reqwest::Url>()?).to_string())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let response = client.get(&args.url).send().await?;
    println!("{}", response.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut headers = HashMap::new();
    for pair in &args.body {
        headers.insert(&pair.k, &pair.v);
    }
    let response = client.post(&args.url).json(&headers).send().await?;
    println!("{}", response.text().await?);
    Ok(())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, val) in resp.headers() {
        println!("{} {:?}", name.to_string().green(), val)
    }
    print!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        _ => println!("{}", body),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client = Client::builder().default_headers(headers).build()?;
    let result = match cli.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
