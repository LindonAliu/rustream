use super::{GroupView, View, ViewMessage};
use crate::m3u::{Channel, Group};
use iced::widget::{button, scrollable, text_input, Column, Row, Container};
use iced::{Element, Length};

use std::cmp::Ordering;
use std::io::BufRead;
use std::process::Stdio;
use std::{
    env::{consts::OS, current_exe},
    path::Path,
};
use which::which;


pub struct ChannelView {
    m3u_filepath: Option<String>,
    groups: Vec<Group>,
    channels: Vec<Channel>,
    filtered_channels: Vec<Channel>,
    search_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackToGroups,
    ChannelSelected(usize),
    SearchTextChanged(String),
}

impl ChannelView {
    pub fn new(group: Group, groups: Vec<Group>, m3u_filepath: Option<String>) -> Self {
        Self {
            m3u_filepath,
            groups,
            channels: group.channels.clone(),
            filtered_channels: group.channels,
            search_text: String::new(),
        }
    }

    fn get_play_args(channel: &Channel, path: String) -> Result<Vec<String>, std::io::Error> {
        let mut args = vec![channel.url.clone()];

        if channel.url.ends_with(".mkv") || channel.url.ends_with(".mp4") {
            args.push("--save-position-on-quit".to_string());
        }
        if OS == "macos" && path != "mpv" {
            args.push("--script-opts=ytdl_hook-ytdl_path=".to_string() + find_macos_bin("yt-dlp".to_string()).as_str());
        }
        args.push(format!("--title={}", channel.name));
        args.push("--msg-level=all=error".to_string());

        Ok(args)
    }

    fn create_search_bar(&self) -> Element<ViewMessage> {
        text_input("Rechercher", &self.search_text)
            .padding(10)
            .size(20)
            .on_input(|s| ViewMessage::ChannelViewMessage(Message::SearchTextChanged(s)))
            .into()
    }

    fn create_back_button(&self) -> Element<ViewMessage> {
        button("Retour")
            .on_press(ViewMessage::ChannelViewMessage(Message::BackToGroups))
            .padding(10)
            .width(Length::Fill)
            .height(50)
            .into()
    }

    fn create_channel_buttons(&self) -> Column<ViewMessage> {
        self.filtered_channels
            .iter()
            .enumerate()
            .collect::<Vec<_>>()
            .chunks(4)
            .fold(Column::new().spacing(10), |column, chunk| {
                let row = chunk.iter().fold(Row::new().spacing(10), |row, (index, channel)| {
                    row.push(
                        button(channel.name.as_str())
                            .on_press(ViewMessage::ChannelViewMessage(Message::ChannelSelected(*index)))
                            .padding(10)
                            .width(Length::Fill)
                            .height(50),
                    )
                });
                column.push(row)
            })
    }

    fn create_ui(&self) -> Column<ViewMessage> {
        let search_bar = self.create_search_bar();
        let back_button = self.create_back_button();
        let channels = self.create_channel_buttons();

        Column::new()
            .spacing(20)
            .push(
                Container::new(
                    Row::new()
                        .spacing(10)
                        .push(search_bar)
                        .push(back_button)
                )
                .padding(10)
                .center_x(Length::Fill),
            )
            .push(
                Container::new(
                    scrollable(channels)
                        .height(Length::Fill)
                        .width(Length::Fill)
                )
                .padding(10),
            )
    }
}

fn get_mpv_path() -> String {
    if OS == "linux" || which("mpv").is_ok() {
        return "mpv".to_string();
    } else if OS == "macos" {
        return find_macos_bin("mpv".to_string());
    }
    return get_mpv_path_win();
}

const MACOS_POTENTIAL_PATHS: [&str; 3] = [
    "/opt/local/bin",    // MacPorts
    "/opt/homebrew/bin", // Homebrew on AARCH64 Mac
    "/usr/local/bin",    // Homebrew on AMD64 Mac
];

fn find_macos_bin(bin: String) -> String {
    return MACOS_POTENTIAL_PATHS
        .iter()
        .map(|path| {
            let mut path = Path::new(path).to_path_buf();
            path.push(&bin);
            return path;
        })
        .find(|path| {        
            path.exists()
        })
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            println!("Could not find mpv binary in common paths, falling back to bundled binary");
            return bin;
        });
}

fn get_mpv_path_win() -> String {
    let mut path = current_exe().unwrap();
    path.pop();
    path.push("deps");
    path.push("mpv.exe");
    return path.to_string_lossy().to_string();
}

impl View for ChannelView {
    fn update(&mut self, message: ViewMessage) -> Option<Box<dyn View>> {
        match message {
            ViewMessage::ChannelViewMessage(msg) => match msg {
                Message::BackToGroups => {
                    return Some(Box::new(GroupView::new(
                        self.groups.clone(),
                        self.m3u_filepath.clone(),
                    )));
                }
                Message::ChannelSelected(index) => {
                    let selected_channel = self.filtered_channels[index].clone();
                    println!("Chaîne sélectionnée : {}", selected_channel.name);
                    let path = get_mpv_path();
                    let args = Self::get_play_args(&selected_channel, path.clone()).unwrap();
                    let mut cmd = std::process::Command::new(path)
                        .args(args)
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let status = cmd.wait().unwrap();

                    if !status.success() {
                        if let Some(stdout) = cmd.stdout.take() {
                            let mut error = String::new();
                            let mut lines = std::io::BufReader::new(stdout).lines();
                            let mut first = true;
                            while let Some(line) = lines.next() {
                                error += &line.unwrap();
                                if !first {
                                    error += "\n";
                                } else {
                                    first = false;
                                }
                            }
                            if !error.is_empty() {
                                eprintln!("{}", error);
                            } else {
                                eprintln!("Mpv encountered an unknown error");
                            }
                        }
                    }
                }
                Message::SearchTextChanged(new_text) => {
                    self.search_text = new_text;
                    update_filtered_list(self);
                }
            },
            _ => {}
        }
        None
    }

    fn view(&self) -> Element<ViewMessage> {
        Container::new(self.create_ui())
            .padding(20)
            .center_x(Length::Fill)
            .into()
    }
}

fn update_filtered_list(state: &mut ChannelView) {
    if state.search_text.is_empty() {
        state.filtered_channels = state.channels.clone();
    } else {
        let search_lower = state.search_text.to_lowercase().replace(' ', "");
        let mut filtered: Vec<_> = state
            .channels
            .iter()
            .filter(|channel| {
                let channel_name_lower = channel.name.to_lowercase().replace(' ', "");
                search_lower.chars().all(|c| channel_name_lower.contains(c))
            })
            .cloned()
            .collect();

        filtered.sort_by(|a, b| {
            let channel_a_name = a.name.to_lowercase().replace(' ', "");
            let channel_b_name = b.name.to_lowercase().replace(' ', "");

            let score_a = calculate_match_score(&search_lower, &channel_a_name);
            let score_b = calculate_match_score(&search_lower, &channel_b_name);

            score_b.partial_cmp(&score_a).unwrap_or(Ordering::Equal)
        });

        state.filtered_channels = filtered;
    }
}

fn calculate_match_score(search: &str, channel_name: &str) -> f64 {
    let mut score = 0.0;
    let mut match_index = 0;
    for (i, ch) in channel_name.chars().enumerate() {
        if match_index < search.len() && ch == search.chars().nth(match_index).unwrap() {
            match_index += 1;
            score += 1.0;
            if i == match_index - 1 {
                score += 0.5;
            }
        }
    }
    score
}
