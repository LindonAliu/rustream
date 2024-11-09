use super::{GroupView, View, ViewMessage};
use crate::m3u::{parse_m3u, Group};

use iced::widget::{button, text, Column};
use iced::Element;

use rfd::FileDialog;

#[derive(Debug, Clone)]
pub enum Message {
    BackToGroups,
    SelectFile,
}

pub struct SettingsView {
    m3u_path: Option<String>,
    groups: Vec<Group>,
}

impl SettingsView {
    pub fn new(groups: Vec<Group>, m3u_path: Option<String>) -> Self {
        Self { groups, m3u_path }
    }

    pub fn refresh_playlist(&mut self) {
        let filepath = FileDialog::new()
            .set_title("Choose a M3U file...")
            .pick_file()
            .map(|path| path.to_string_lossy().into_owned());

        self.m3u_path = filepath.clone();
        self.groups = match filepath {
            Some(path) => {
                let groups = parse_m3u(&path).unwrap_or_default();
                println!("Loaded {} groups from {}", groups.len(), path);
                groups
            }
            None => vec![],
        };
    }
}

impl View for SettingsView {
    fn update(&mut self, message: ViewMessage) -> Option<Box<dyn View>> {
        match message {
            ViewMessage::SettingsViewMessage(msg) => match msg {
                Message::SelectFile => {
                    self.refresh_playlist();
                }
                Message::BackToGroups => {
                    return Some(Box::new(GroupView::new(
                        self.groups.clone(),
                        self.m3u_path.clone(),
                    )));
                }
            },
            _ => {}
        }
        None
    }

    fn view(&self) -> Element<ViewMessage> {
        let file_picker = button("Sélectionner un fichier M3U")
            .on_press(ViewMessage::SettingsViewMessage(Message::SelectFile));

        let back_button = button("Retour")
            .on_press(ViewMessage::SettingsViewMessage(Message::BackToGroups))
            .padding(10);

        let m3u_path = match &self.m3u_path {
            Some(path) => text(path),
            None => text("Aucun fichier M3U sélectionné"),
        };

        Column::new()
            .push(file_picker)
            .push(m3u_path)
            .push(back_button)
            .into()
    }
}
