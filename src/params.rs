use crate::types::Result;
use std::env::Args;

pub struct Params {
    pub m3u_filepath: String,
}

pub fn parse_args(args: Args) -> Result<Params> {
    let mut m3u_filepath: Option<String> = None;
    let mut it = args.skip(1);

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-m" | "--m3u" => {
                m3u_filepath = Some(it.next().ok_or("Missing argument for -m")?);
            }
            _ => {
                return Err(format!("Unknown argument: {}", arg))?;
            }
        }
    }

    let m3u_filepath = m3u_filepath.ok_or("Missing argument: -m")?;

    if !std::path::Path::new(&m3u_filepath).exists() {
        return Err(format!("File not found: {}", m3u_filepath))?;
    }

    Ok(Params { m3u_filepath })
}
