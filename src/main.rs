use clap::Parser;
use reqwest::Method;
use std::error::Error;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "Murl", about = "A rust based Curl", version = "0.1", author = "Muchui William", long_about = "None")]
struct Args{
    #[arg(short = 'u', long = "url")]
    url: String,

    #[arg(short = 'X', long = "method")]
    method: Option<String>,

    #[arg(short = 'H', long = "headers", value_parser = parse_key_val)]
    headers: Option<HashMap<String, String>>,

    #[arg(short = 'd', long = "data")]
    data: Option<String>,
}

fn parse_key_val(s: &str)->Result<(String, String), String>{
    let pos = s.find('=').ok_or_else(||String::from("Invalid key value Pair"))?;

    let key = s[..pos].to_string();
    let value = s[pos+1..].to_string();
    Ok((key, value))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();

    let url = &args.url; 
    println!("hello : {}", url);

    let method_input = args.method.unwrap_or_else(||String::from("GET"));
    let method = match method_input.to_uppercase().as_str(){
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => reqwest::Method::GET
    };

    let data = args.data;
    let headers = args.headers.unwrap_or_default();
    let data = fetch_data(&url, method, data, headers).await?;
    println!("{}", data);
    Ok(())
}

async fn fetch_data(
    url: &str,
    method: Method,
    data: Option<String>,
    headers: HashMap<String, String>
) -> Result<String, reqwest::Error>{

    let client = reqwest::Client::new();
    let mut request = client.request(method, url);

    for (key, value) in headers{
        request = request.header(&key, value);
    }
    if let Some(data) = data{
        request = request.body(data);
    }

    let response = request.send().await?;
    let response_text = response.text().await?;
    Ok(response_text)
}