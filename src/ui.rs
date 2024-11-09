use crate::m3u::{Channel, Group};
use crate::types::Result;

use iced::widget::{button, container, image, scrollable, text, text_input, Column, Row};
use iced::{Element, Length, Task, Theme};

use reqwest::blocking::get;

use std::cmp::Ordering;
use std::io::BufRead;
use std::process::Stdio;

pub fn run(groups: Vec<Group>) -> Result<()> {
    iced::application("Rustream", State::update, State::view)
        .theme(|_| Theme::Dark)
        .resizable(true)
        .centered()
        .run_with(|| State::new(groups))?;
    Ok(())
}

pub struct State {
    groups: Vec<Group>,
    filtered_groups: Vec<Group>,
    selected_group: Option<Group>,
    channels: Vec<Channel>,
    filtered_channels: Vec<Channel>,
    search_text: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            filtered_groups: Vec::new(),
            selected_group: None,
            channels: Vec::new(),
            filtered_channels: Vec::new(),
            search_text: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackToGroups,
    GroupSelected(usize),
    ChannelSelected(usize),
    SearchTextChanged(String),
}

impl State {
    fn new(groups: Vec<Group>) -> (Self, Task<Message>) {
        (
            Self {
                groups: groups.clone(),
                filtered_groups: groups,
                ..Default::default()
            },
            Task::none(),
        )
    }
    fn update(state: &mut State, message: Message) {
        match message {
            Message::BackToGroups => {
                state.selected_group = None;
                state.channels.clear();
                state.filtered_channels.clear();
                state.search_text.clear();
                update_filtered_list(state);
            }
            Message::GroupSelected(index) => {
                state.selected_group = Some(state.groups[index].clone());
                state.channels = state.groups[index].channels.clone();
                state.search_text.clear();
                update_filtered_list(state);
            }
            Message::ChannelSelected(index) => {
                let selected_channel = state.filtered_channels[index].clone();
                println!("Chaîne sélectionnée : {}", selected_channel.name);
                let args = Self::get_play_args(selected_channel).unwrap();
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
                state.search_text = new_text;
                update_filtered_list(state);
            }
        };
    }

    fn get_play_args(channel: Channel) -> Result<Vec<String>> {
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

    fn view(state: &State) -> Element<'_, Message> {
        let search_bar = text_input("Rechercher", &state.search_text)
            .padding(10)
            .size(20)
            .on_input(Message::SearchTextChanged);

        let content = if let Some(selected_group) = &state.selected_group {
            let back_button = button("Retour")
                .on_press(Message::BackToGroups)
                .padding(10)
                .width(Length::Fill)
                .height(50);

            let channels = state
                .filtered_channels
                .iter()
                .filter(|channel| channel.group == selected_group.name)
                .enumerate()
                .collect::<Vec<_>>()
                .chunks(4)
                .fold(Column::new().spacing(10), |column, chunk| {
                    let row = chunk
                        .iter()
                        .fold(Row::new().spacing(10), |row, (index, channel)| {
                            row.push(create_button_channel(channel, *index))
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
        } else {
            let groups = state
                .filtered_groups
                .iter()
                .enumerate()
                .collect::<Vec<_>>()
                .chunks(4)
                .fold(Column::new().spacing(10), |column, chunk| {
                    let row = chunk
                        .iter()
                        .fold(Row::new().spacing(10), |row, (index, group)| {
                            row.push(
                                button(group.name.as_str())
                                    .on_press(Message::GroupSelected(*index))
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
                .push(scrollable(groups).height(Length::Fill).width(Length::Fill))
        };

        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn create_button_channel<'a>(channel: &'a Channel, index: usize) -> Element<'a, Message> {
    let mut row = Row::new().spacing(10);

    // if let Some(logo_url) = &channel.logo_url {
    //     if let Ok(logo) = get(logo_url) {
    //         if let Ok(bytes) = logo.bytes() {
    //             println!("Logo téléchargé : {}", bytes.len());
    //             let handle = iced::widget::image::Handle::from_bytes(bytes.to_vec());
    //             let image = image(handle).width(50).height(50);
    //             row = row.push(image);
    //         }
    //     }
    // }

    row = row.push(text(channel.name.as_str()));

    button(row)
        .on_press(Message::ChannelSelected(index))
        .padding(10)
        .width(Length::Fill)
        .height(50)
        .into()
}

fn update_filtered_list(state: &mut State) {
    if state.selected_group.is_none() {
        update_filtered_groups(state);
    } else {
        update_filtered_channels(state);
    }
}

fn update_filtered_groups(state: &mut State) {
    if state.search_text.is_empty() {
        state.filtered_groups = state.groups.clone();
    } else {
        let search_lower = state.search_text.to_lowercase().replace(' ', "");
        let mut filtered: Vec<_> = state
            .groups
            .iter()
            .filter(|group| {
                let group_name_lower = group.name.to_lowercase().replace(' ', "");
                search_lower.chars().all(|c| group_name_lower.contains(c))
            })
            .cloned()
            .collect();

        filtered.sort_by(|a, b| {
            let group_a_name = a.name.to_lowercase().replace(' ', "");
            let group_b_name = b.name.to_lowercase().replace(' ', "");

            let score_a = calculate_match_score(&search_lower, &group_a_name);
            let score_b = calculate_match_score(&search_lower, &group_b_name);

            score_b.partial_cmp(&score_a).unwrap_or(Ordering::Equal)
        });

        state.filtered_groups = filtered;
    }
}

fn update_filtered_channels(state: &mut State) {
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
