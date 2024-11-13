extern crate m3u;
use m3u::iptv::IptvEntry;
use m3u::Reader;

use crate::types::Result;

pub trait Named {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub url: String,
    pub id: Option<String>,
    pub logo_url: Option<String>,
    pub group: String,
}

impl Named for Channel {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub channels: Vec<Channel>,
}

impl Named for Group {
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn parse_m3u(m3u_filepath: &str) -> Result<Vec<Group>> {
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
            let url = match &entry.entry {
                m3u::Entry::Url(u) => u.to_string(),
                m3u::Entry::Path(p) => p.display().to_string(),
            };
            let id = props.get("tvg-id").map(|s| s.to_string());
            let logo_url = props.get("tvg-logo").map(|s| s.to_string());
            let group = props
                .get("group-title")
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Autres".to_string());

            Channel {
                name: name.clone(),
                url,
                id,
                logo_url,
                group: group,
            }
        })
        .collect();

    let mut groups: Vec<Group> = Vec::new();

    for channel in channels {
        let group_name = channel.group.clone();
        let group = groups.iter_mut().find(|g| g.name == group_name);

        match group {
            Some(g) => g.channels.push(channel),
            None => {
                groups.push(Group {
                    name: group_name,
                    channels: vec![channel],
                });
            }
        }
    }

    Ok(groups)
}
