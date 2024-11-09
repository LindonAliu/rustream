pub mod channel_view;
pub mod group_view;

use iced::Element;

pub use channel_view::ChannelView;
pub use group_view::GroupView;

pub trait View {
    fn update(&mut self, message: ViewMessage) -> Option<Box<dyn View>>;
    fn view(&self) -> Element<ViewMessage>;
}

#[derive(Debug, Clone)]
pub enum ViewMessage {
    // Common messages
    GroupViewMessage(group_view::Message),
    ChannelViewMessage(channel_view::Message),
}
