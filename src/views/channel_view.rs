use super::{GroupView, View, ViewMessage};
use crate::m3u::{Channel, Group};
use iced::widget::{button, scrollable, text, text_input, Column, Row};
use iced::{Element, Length};

use std::cmp::Ordering;
use std::io::{BufRead, BufReader};
use std::process::Stdio;

pub struct ChannelView {
    group: Group,
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
    pub fn new(group: Group, groups: Vec<Group>) -> Self {
        Self {
            group: group.clone(),
            groups,
            channels: group.channels.clone(),
            filtered_channels: group.channels,
            search_text: String::new(),
        }
    }

    fn get_play_args(channel: &Channel) -> Result<Vec<String>, std::io::Error> {
        let mut args = Vec::new();
        args.push(channel.url.clone());

        // if is not stream
        if channel.url.ends_with(".mkv") || channel.url.ends_with(".mp4") {
            args.push("--save-position-on-quit".to_string());
        }

        // args.push("--cache=no".to_string());
        args.push(format!("--title={}", channel.name));
        args.push("--msg-level=all=error".to_string());

        Ok(args)
    }
}

impl View for ChannelView {
    fn update(&mut self, message: ViewMessage) -> Option<Box<dyn View>> {
        match message {
            ViewMessage::ChannelViewMessage(msg) => match msg {
                Message::BackToGroups => {
                    return Some(Box::new(GroupView::new(self.groups.clone())));
                }
                Message::ChannelSelected(index) => {
                    let selected_channel = self.filtered_channels[index].clone();
                    println!("Chaîne sélectionnée : {}", selected_channel.name);
                    let args = Self::get_play_args(&selected_channel).unwrap();
                    let mut cmd = std::process::Command::new("mpv")
                        .args(args)
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let status = cmd.wait().unwrap();

                    if !status.success() {
                        let stdout = cmd.stdout.take();
                        if let Some(stdout) = stdout {
                            let mut error: String = "".to_string();
                            let mut lines = std::io::BufReader::new(stdout).lines();
                            let mut first = true;
                            while let Some(line) = lines.next() {
                                error += &line.unwrap();
                                if !first {
                                    error += "\n"
                                } else {
                                    first = false;
                                }
                            }
                            if error != "" {
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
        let search_bar = text_input("Rechercher", &self.search_text)
            .padding(10)
            .size(20)
            .on_input(|s| ViewMessage::ChannelViewMessage(Message::SearchTextChanged(s)));

        let back_button = button("Retour")
            .on_press(ViewMessage::ChannelViewMessage(Message::BackToGroups))
            .padding(10)
            .width(Length::Fill)
            .height(50);

        let channels = self
            .filtered_channels
            .iter()
            .enumerate()
            .collect::<Vec<_>>()
            .chunks(4)
            .fold(Column::new().spacing(10), |column, chunk| {
                let row = chunk
                    .iter()
                    .fold(Row::new().spacing(10), |row, (index, channel)| {
                        row.push(
                            button(channel.name.as_str())
                                .on_press(ViewMessage::ChannelViewMessage(
                                    Message::ChannelSelected(*index),
                                ))
                                .padding(10)
                                .width(Length::Fill)
                                .height(50),
                        )
                    });
                column.push(row)
            });

        Column::new()
            .spacing(20)
            .push(search_bar)
            .push(back_button)
            .push(
                scrollable(channels)
                    .height(Length::Fill)
                    .width(Length::Fill),
            )
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
