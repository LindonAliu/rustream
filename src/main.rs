use gstreamer as gst;
use gstreamer::prelude::*;
use iced::Application;
use rustream::m3u::parse_m3u;
use rustream::params::{parse_args, Params};
use rustream::types::Result;
use rustream::ui::M3UApp;

fn run() -> Result<()> {
    // Initialiser GStreamer
    gst::init()?;

    let params: Params = parse_args(std::env::args())?;
    let channels = parse_m3u(&params.m3u_filepath)?;

    M3UApp::run(iced::Settings {
        flags: channels,
        ..iced::Settings::default()
    })?;

    Ok(())
}

fn main() -> iced::Result {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
