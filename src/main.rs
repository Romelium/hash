use clap::Parser;

pub mod hashes;
use hashes::sha256::Sha256;
use hashes::traits::Hash;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "sha256")]
    hash: String,
    string: String,
}

fn main() {
    let args = Cli::parse();
    let sha256 = Sha256{};
    match args.hash.as_ref(){
        "sha256"=> println!("{}", sha256.hash(args.string)),
        _=>println!("Unexpected hash name"),
    }
}