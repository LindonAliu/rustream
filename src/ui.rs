use crate::m3u::Channel;
use iced::widget::{Button, Column, Container, Scrollable, Text, TextInput};
use iced::{alignment, executor, Application, Command, Element, Length};
use std::cmp::Ordering;

pub struct M3UApp {
    channels: Vec<Channel>,
    filtered_channels: Vec<Channel>,
    search_text: String,
}

impl Default for M3UApp {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            filtered_channels: Vec::new(),
            search_text: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChannelSelected(usize),
    SearchTextChanged(String),
}

impl Application for M3UApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Vec<Channel>;
    type Theme = iced::Theme;

    fn new(channels: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            M3UApp {
                channels: channels.clone(),
                filtered_channels: channels,
                search_text: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Lecteur M3U")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::ChannelSelected(index) => {
                if let Some(selected_channel) = self.filtered_channels.get(index) {
                    println!("Chaîne sélectionnée : {}", selected_channel.name);

                    if let Err(e) = webbrowser::open(&selected_channel.url) {
                        eprintln!("Erreur lors de l'ouverture du flux : {}", e);
                    }
                }
            }
            Message::SearchTextChanged(new_text) => {
                self.search_text = new_text;
                if self.search_text.is_empty() {
                    self.filtered_channels = self.channels.clone();
                } else {
                    let search_lower = self.search_text.to_lowercase().replace(' ', "");
                    let mut filtered: Vec<_> = self
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

                    self.filtered_channels = filtered;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let search_bar = TextInput::new("Rechercher une chaîne...", &self.search_text)
            .on_input(Message::SearchTextChanged)
            .padding(10)
            .size(16);

        let channels_list = if self.filtered_channels.is_empty() {
            Column::new().push(Text::new("Aucune chaîne trouvée").size(16))
        } else {
            self.filtered_channels.iter().enumerate().take(100).fold(
                Column::new().spacing(10).padding(20),
                |column, (i, channel)| {
                    let button = Button::new(
                        Text::new(&channel.name)
                            .size(16)
                            .horizontal_alignment(alignment::Horizontal::Left),
                    )
                    .on_press(Message::ChannelSelected(i))
                    .padding(10)
                    .width(Length::Fill);

                    column.push(button)
                },
            )
        };

        let content = Column::new()
            .push(search_bar)
            .push(
                Scrollable::new(channels_list)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
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
