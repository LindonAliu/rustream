use super::iced_utils::create_buttons;
use super::{ChannelView, SettingsView, View, ViewMessage};
use crate::m3u::Group;
use iced::widget::{button, scrollable, text_input, Column, Container};
use iced::{Element, Length};

use std::cmp::Ordering;

pub struct GroupView {
    m3u_path: Option<String>,
    groups: Vec<Group>,
    filtered_groups: Vec<Group>,
    search_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    GroupSelected(usize),
    SettingsSelected,
    SearchTextChanged(String),
}

impl GroupView {
    pub fn new(groups: Vec<Group>, m3u_path: Option<String>) -> Self {
        Self {
            m3u_path,
            groups: groups.clone(),
            filtered_groups: groups,
            search_text: String::new(),
        }
    }
}

impl View for GroupView {
    fn update(&mut self, message: ViewMessage) -> Option<Box<dyn View>> {
        match message {
            ViewMessage::GroupViewMessage(msg) => match msg {
                Message::GroupSelected(index) => {
                    let selected_group = self.filtered_groups[index].clone();
                    return Some(Box::new(ChannelView::new(
                        selected_group,
                        self.groups.clone(),
                        self.m3u_path.clone(),
                    )));
                }
                Message::SettingsSelected => {
                    return Some(Box::new(SettingsView::new(
                        self.groups.clone(),
                        self.m3u_path.clone(),
                    )));
                }
                Message::SearchTextChanged(new_text) => {
                    self.search_text = new_text;
                    update_filtered_groups(self);
                }
            },
            _ => {}
        }
        None
    }

    fn view(&self) -> Element<ViewMessage> {
        let settings_button = button("Paramètres")
            .on_press(ViewMessage::GroupViewMessage(Message::SettingsSelected))
            .padding(10);

        let search_bar = text_input("Rechercher", &self.search_text)
            .padding(10)
            .size(20)
            .on_input(|s| ViewMessage::GroupViewMessage(Message::SearchTextChanged(s)));

        let groups = create_buttons(self.filtered_groups.clone(), |index| {
            ViewMessage::GroupViewMessage(Message::GroupSelected(index))
        });

        Container::new(
            Column::new()
                .spacing(20)
                .push(
                    Container::new(settings_button)
                        .padding(10)
                        .center_x(Length::Fill),
                )
                .push(
                    Container::new(search_bar)
                        .padding(10)
                        .center_x(Length::Fill),
                )
                .push(
                    Container::new(scrollable(groups).height(Length::Fill).width(Length::Fill))
                        .padding(10),
                ),
        )
        .padding(20)
        .center_x(Length::Fill)
        .into()
    }
}

fn update_filtered_groups(state: &mut GroupView) {
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

fn calculate_match_score(search: &str, group_name: &str) -> f64 {
    let mut score = 0.0;
    let mut match_index = 0;
    for (i, ch) in group_name.chars().enumerate() {
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
