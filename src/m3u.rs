extern crate m3u;
use m3u::iptv::IptvEntry;
use m3u::Reader;

use crate::types::Result;

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub url: m3u::Entry,
    pub id: Option<String>,
    pub logo_url: Option<String>,
}

pub fn parse_m3u(m3u_filepath: &str) -> Result<Vec<Channel>> {
    let mut reader: Reader<std::io::BufReader<std::fs::File>, IptvEntry> =
        Reader::open_iptv(m3u_filepath)?;
    let chans: Vec<IptvEntry> = reader.iptv_entries().filter_map(|r| r.ok()).collect();

    let channels: Vec<Channel> = chans
        .iter()
        .map(|entry| {
            let mut clone = entry.clone();
            let extinf = clone.parsed_extinf().as_ref().unwrap();
            let props = &extinf.iptv_props;
            let name = &extinf.name;
            let url = entry.entry.clone();
            let id = props.get("tvg-id").map(|s| s.to_string());
            let logo_url = props.get("tvg-logo").map(|s| s.to_string());

            Channel {
                name: name.clone(),
                url,
                id,
                logo_url,
            }
        })
        .collect();

    Ok(channels)
}
