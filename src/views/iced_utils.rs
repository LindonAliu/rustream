use crate::m3u::Named;

use super::ViewMessage;
use iced::widget::{button, Column, Row};
use iced::Length;

pub fn create_buttons<T: Named + 'static>(
    elements: Vec<T>,
    on_press: fn(usize) -> ViewMessage,
) -> Column<'static, ViewMessage> {
    elements
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .chunks(4)
        .fold(Column::new().spacing(10), |column, chunk| {
            let row = chunk
                .iter()
                .fold(Row::new().spacing(10), |row, (index, element)| {
                    row.push(
                        button(iced::widget::Text::new(element.name().to_string()))
                            .on_press(on_press(*index))
                            .padding(10)
                            .width(Length::Fill)
                            .height(50),
                    )
                });
            column.push(row)
        })
}
