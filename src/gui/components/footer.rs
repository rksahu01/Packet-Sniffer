//! GUI bottom footer


use iced::alignment::{Horizontal, Vertical};
use iced::widget::{ Container, Row, Text};
use iced::widget::horizontal_space;
use iced::{Alignment, Font, Length};
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;

pub fn footer(
    thumbnail: bool,
    color_gradient: GradientType,
    font_footer: Font,
) -> Container<'static, Message, StyleType> {
    if thumbnail {
        return thumbnail_footer();
    }


    let footer_row = Row::new()
        .spacing(10)
        .padding([0, 20])
        .align_items(Alignment::Center)
        .push(
            Text::new("Made with ❤ by NIT Rourkela")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        );

    Container::new(footer_row)
        .height(45)
        .align_y(Vertical::Center)
        .style(ContainerType::Gradient(color_gradient))
}



fn thumbnail_footer() -> Container<'static, Message, StyleType> {
    Container::new(horizontal_space()).height(0)
}
