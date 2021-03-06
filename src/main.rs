mod pretty;

use clap::Parser;
use std::io::Read;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// file path to read
    file: Option<String>,

    #[clap(short, long)]
    string: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut contents = Vec::new();
    if let Some(path) = args.file {
        let path = std::path::Path::new(&path);
        let mut file = std::fs::File::open(path)?;
        file.read_to_end(&mut contents)?;
    } else if let Some(string) = args.string {
        contents = string.as_bytes().to_vec();
    } else {
        std::io::stdin().read_to_end(&mut contents)?;
    }
    pretty::display(&contents)?;

    Ok(())
}
