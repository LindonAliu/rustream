use crate::views::SettingsView;
use crate::views::View;
use crate::views::ViewMessage;
use iced::{Element, Task};

pub struct App {
    current_view: Box<dyn View>,
}

impl App {
    pub fn new() -> (Self, Task<ViewMessage>) {
        (
            Self {
                current_view: Box::new(SettingsView::new(Vec::new(), None)),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: ViewMessage) {
        if let Some(new_view) = self.current_view.update(message) {
            self.current_view = new_view;
        }
    }

    pub fn view(&self) -> Element<ViewMessage> {
        self.current_view.view()
    }
}
