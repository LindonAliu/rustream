use super::iced_utils::create_buttons;
use super::{GroupView, View, ViewMessage};
use crate::m3u::{Channel, Group};
use crate::mpv::play;
use iced::widget::{button, scrollable, text_input, Column, Container, Row};
use iced::{Element, Length};
use std::cmp::Ordering;

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

    fn on_press(index: usize) -> ViewMessage {
        ViewMessage::ChannelViewMessage(Message::ChannelSelected(index))
    }

    fn create_ui(&self) -> Column<ViewMessage> {
        let search_bar = self.create_search_bar();
        let back_button = self.create_back_button();
        let channels = create_buttons::<Channel>(self.filtered_channels.clone(), Self::on_press);

        Column::new()
            .spacing(20)
            .push(
                Container::new(Row::new().spacing(10).push(search_bar).push(back_button))
                    .padding(10)
                    .center_x(Length::Fill),
            )
            .push(
                Container::new(
                    scrollable(channels)
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
                .padding(10),
            )
    }
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

                    play(selected_channel);
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
