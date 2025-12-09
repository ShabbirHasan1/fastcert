use clap::Parser;
use rscert::Result;

#[derive(Parser, Debug)]
#[command(name = "rscert")]
#[command(about = "A simple zero-config tool to make locally-trusted development certificates", long_about = None)]
struct Cli {
    /// Install the local CA in the system trust store
    #[arg(long)]
    install: bool,

    /// Domain names or IP addresses to generate certificates for
    #[arg(value_name = "DOMAINS")]
    domains: Vec<String>,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    println!("rscert - certificate generation tool");
    Ok(())
}
