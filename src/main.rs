pub mod m3u;
pub mod params;
pub mod types;
pub mod ui;
pub mod views;

use m3u::parse_m3u;
use params::{parse_args, Params};
use types::Result;
use ui::run as ui_run;

fn run() -> Result<()> {
    let params: Params = parse_args(std::env::args())?;
    let channels = parse_m3u(&params.m3u_filepath)?;

    ui_run(channels)?;

    Ok(())
}

fn main() -> iced::Result {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
