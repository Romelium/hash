use clap::Parser;

pub mod hashes;
use hashes::sha256::Sha256;
use hashes::md5::MD5;
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
    let hashes:[Box<dyn Hash>; 2] = [
        Box::new(Sha256{}),
        Box::new(MD5{})
    ];
    let hash_option = hashes.iter().find(|&x| x.get_name() == args.hash);
    if let Some(hash) = hash_option
    {
        println!("{}", hash.hash(args.string))
    }
    else {
        println!("Unexpected hash name")
    }
}