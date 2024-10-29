use crate::m3u::Channel;
use gstreamer as gst;
use gstreamer::prelude::*;
use iced::widget::{Button, Column, Container, Scrollable, Text, TextInput};
use iced::{alignment, executor, Application, Command, Element, Length};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct M3UApp {
    channels: Vec<Channel>,
    filtered_channels: Vec<Channel>,
    search_text: String,
    video_manager: VideoManager,
    viewing_video: bool,
    current_channel: Option<String>,
}

impl Default for M3UApp {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            filtered_channels: Vec::new(),
            search_text: String::new(),
            video_manager: VideoManager::new(),
            viewing_video: false,
            current_channel: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChannelSelected(usize),
    SearchTextChanged(String),
    StopVideo,
    PauseVideo,
    ResumeVideo,
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
                video_manager: VideoManager::new(),
                viewing_video: false,
                current_channel: None,
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
                let selected_channel = self.filtered_channels[index].clone();
                println!("Chaîne sélectionnée : {}", selected_channel.name);
                self.video_manager.play_channel(&selected_channel.url);
                self.viewing_video = true;
                self.current_channel = Some(selected_channel.url);
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
            Message::StopVideo => {
                if let Some(channel_url) = &self.current_channel {
                    self.video_manager.stop_channel(channel_url);
                }
                self.viewing_video = false;
                self.current_channel = None;
            }
            Message::PauseVideo => {
                if let Some(channel_url) = &self.current_channel {
                    self.video_manager.pause_channel(channel_url);
                }
            }
            Message::ResumeVideo => {
                if let Some(channel_url) = &self.current_channel {
                    self.video_manager.resume_channel(channel_url);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content: Element<_> = if self.viewing_video {
            let stop_button = Button::new(Text::new("Retour"))
                .padding(20)
                .on_press(Message::StopVideo);

            let pause_button = Button::new(Text::new("Pause"))
                .padding(20)
                .on_press(Message::PauseVideo);

            let resume_button = Button::new(Text::new("Reprendre"))
                .padding(20)
                .on_press(Message::ResumeVideo);

            Column::new()
                .spacing(20)
                .align_items(alignment::Alignment::Center)
                .push(stop_button)
                .push(pause_button)
                .push(resume_button)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
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

            Column::new()
                .push(search_bar)
                .push(
                    Scrollable::new(channels_list)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct VideoManager {
    pipelines: HashMap<String, gst::Pipeline>,
}

impl VideoManager {
    fn new() -> Self {
        VideoManager {
            pipelines: HashMap::new(),
        }
    }

    fn play_channel(&mut self, channel_url: &str) {
        if let Some(pipeline) = self.pipelines.get(channel_url) {
            pipeline
                .set_state(gst::State::Playing)
                .expect("Impossible de démarrer le pipeline");
        } else {
            let pipeline = gst::parse_launch(&format!(
                "playbin uri={} video-sink=autovideosink buffer-size=5000000",
                channel_url
            ))
            .expect("Impossible de créer le pipeline GStreamer");
            // Set buffer duration in ms
            pipeline.set_property_from_str("buffer-size", "5000");
            pipeline
                .set_state(gst::State::Playing)
                .expect("Impossible de démarrer le pipeline");
            self.pipelines.insert(
                channel_url.to_string(),
                pipeline.downcast::<gst::Pipeline>().unwrap(),
            );
        }
    }

    fn stop_channel(&mut self, channel_url: &str) {
        if let Some(pipeline) = self.pipelines.get(channel_url) {
            pipeline
                .set_state(gst::State::Null)
                .expect("Impossible d'arrêter le pipeline");
        }
    }

    fn pause_channel(&mut self, channel_url: &str) {
        if let Some(pipeline) = self.pipelines.get(channel_url) {
            pipeline
                .set_state(gst::State::Paused)
                .expect("Impossible de mettre en pause le pipeline");
        }
    }

    fn resume_channel(&mut self, channel_url: &str) {
        if let Some(pipeline) = self.pipelines.get(channel_url) {
            pipeline
                .set_state(gst::State::Playing)
                .expect("Impossible de reprendre le pipeline");
        }
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
