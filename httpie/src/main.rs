use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::*;
use mime::{Mime, APPLICATION_JSON};
use reqwest::{header, Client, ClientBuilder, Response};
use std::collections::HashMap;
use std::str::FromStr;

/// A simple httpie implemented with rust
#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    Get(Get),
    Post(Post),
}

/// Feed an url and we'll retrieve response for you
#[derive(Debug, Parser)]
pub struct Get {
    #[arg(value_parser=parse_url)]
    url: String,
}

/// Feed an url and optional key=value pairs, we'll send data as JSON format and retrieve response for you
#[derive(Debug, Parser)]
pub struct Post {
    #[arg(value_parser=parse_url)]
    url: String,
    #[arg(value_parser=parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Clone, Debug)]
pub struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<KvPair> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {s}"));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_url(s: &str) -> Result<String> {
    Ok((s.parse::<reqwest::Url>()?).to_string())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse::<KvPair>()?)
}

fn print_status(res: &Response) {
    let status = format!("{:?} {}", res.version(), res.status());
    println!("{status}\n");
}

fn print_headers(res: &Response) {
    for (name, val) in res.headers() {
        println!("{}: {:?}", name.to_string().green(), val);
    }
    println!("\n");
}

fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(v) if v == APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

fn get_content_type(res: &Response) -> Option<Mime> {
    res.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

async fn print_response(res: Response) -> Result<()> {
    print_status(&res);
    print_headers(&res);
    let m = get_content_type(&res);
    let body = res.text().await?;
    print_body(m, &body);
    Ok(())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    print_response(res).await?;
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut headers = HashMap::new();
    for pair in args.body.iter() {
        headers.insert(&pair.k, &pair.v);
    }
    let res = client.post(&args.url).json(&headers).send().await?;
    print_response(res).await?;
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
