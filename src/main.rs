use clap::Parser;
use std::error::Error;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Duration of the interval in seconds
    #[clap(short, long, default_value_t = 1)]
    duration: u64,

    /// HTTP URL to healthcheck
    #[clap(short, long)]
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let url = Url::parse(args.url.as_str())?;

    loop {
        let code = reqwest::blocking::get(url.as_str())?.status().as_u16();

        if code == 200 {
            println!("Checking {url}. Result: OK(200)");
        } else {
            println!("Checking {url}. Result: ERR({code})");
        }

        std::thread::sleep(std::time::Duration::from_secs(args.duration));
    }
}
