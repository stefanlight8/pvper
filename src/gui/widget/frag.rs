use iced::{
    Alignment, Length,
    alignment::Horizontal,
    widget::{column, container, row, text, text::Wrapping},
};

use crate::{
    frags::{Frag, Target},
    gui::element::{Column, Container},
};

pub fn frag<'a, Message: 'a>(frag: &'a Frag) -> Container<'a, Message> {
    let killer: Column<'a, Message> = match &frag.killer {
        Target::You => column![
            text("You"),
            text(format!("{}", frag.ship)).wrapping(Wrapping::WordOrGlyph)
        ]
        .into(), // TODO: replace with one variable
        Target::Player(name) => column![text(name).wrapping(Wrapping::WordOrGlyph)], // TODO: replace with one variable
    };
    let victim: Column<'a, Message> = match &frag.victim {
        Target::You => column![
            text("You").align_x(Alignment::End),
            text(format!("{}", frag.ship))
                .align_x(Alignment::End)
                .wrapping(Wrapping::WordOrGlyph)
        ]
        .align_x(Horizontal::Right)
        .into(), // TODO: replace with one variable
        Target::Player(name) => column![text(name).wrapping(Wrapping::WordOrGlyph)], // TODO: replace with one variable
    };

    container(
        column![
            text(frag.timestamp.to_string()),
            row![
                container(killer)
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Left),
                container(text("killed"))
                    .width(Length::Shrink)
                    .align_x(Horizontal::Center),
                container(victim)
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Right),
            ]
            .width(Length::Fill)
        ]
        .spacing(2)
        .padding(2),
    )
    .style(|theme| {
        if frag.is_kill() {
            container::success(theme)
        } else {
            container::danger(theme)
        }
    })
    .width(Length::Fill)
    .into()
}
