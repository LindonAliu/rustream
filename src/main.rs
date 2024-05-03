use rustream::m3u::parse_m3u;
use rustream::params::{parse_args, Params};
use rustream::types::Result;

fn run() -> Result<()> {
    let params: Params = parse_args(std::env::args())?;
    let channels = parse_m3u(&params.m3u_filepath)?;

    println!("{} channels found", channels.len());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
