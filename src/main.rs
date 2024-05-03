use rustream::params::{parse_args, Params};
use rustream::types::Result;

fn run() -> Result<()> {
    let params: Params = parse_args(std::env::args())?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
