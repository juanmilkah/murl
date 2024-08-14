use clap::Parser;

#[derive(Parser)]
#[command(name = "Murl", about = "A rust based Curl", version = "0.1", author = "Muchui William", long_about = "None")]
struct Args{
    #[arg(short = 'u', long = "url")]
    url: String,

    #[arg(short = 'X')]
    method: Option<String>,
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let url = get_url()?;
    println!("hello : {}", url);

    let method = get_method()?;
    println!("method: {}", method);
    Ok(())
}

fn get_url()->Result<String, Box<dyn std::error::Error>>{
    let args = Args::parse();

    Ok(args.url)
}

fn get_method()-> Result<String, Box<dyn std::error::Error>>{
    let args = Args::parse();
    let method = match args.method {
        Some(v) => v,
        None => String::from("GET"),
    };
    Ok(method)
    
}