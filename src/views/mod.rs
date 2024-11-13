pub mod channel_view;
pub mod group_view;
pub mod iced_utils;
pub mod settings_view;

use iced::Element;

pub use channel_view::ChannelView;
pub use group_view::GroupView;
pub use settings_view::SettingsView;

pub trait View {
    fn update(&mut self, message: ViewMessage) -> Option<Box<dyn View>>;
    fn view(&self) -> Element<ViewMessage>;
}

#[derive(Debug, Clone)]
pub enum ViewMessage {
    // Common messages
    GroupViewMessage(group_view::Message),
    ChannelViewMessage(channel_view::Message),
    SettingsViewMessage(settings_view::Message),
}
