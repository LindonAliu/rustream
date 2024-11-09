use crate::m3u::Channel;
use crate::m3u::Group;
use crate::types::Result;
use crate::views::group_view::GroupView;
use crate::views::View;
use crate::views::ViewMessage;
use iced::{Element, Task, Theme};

pub fn run(groups: Vec<Group>) -> Result<()> {
    iced::application("Rustream", App::update, App::view)
        .theme(|_| Theme::Dark)
        .resizable(true)
        .centered()
        .run_with(|| App::new(groups))?;
    Ok(())
}

struct App {
    current_view: Box<dyn View>,
    m3u_path: Option<String>,
    groups: Vec<Group>,
}

impl App {
    fn new(groups: Vec<Group>) -> (Self, Task<ViewMessage>) {
        (
            Self {
                current_view: Box::new(GroupView::new(groups.clone())),
                m3u_path: None,
                groups,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: ViewMessage) {
        if let Some(new_view) = self.current_view.update(message) {
            self.current_view = new_view;
        }
    }

    fn view(&self) -> Element<ViewMessage> {
        self.current_view.view()
    }
}
