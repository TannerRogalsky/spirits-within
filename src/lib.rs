#[cfg(target_arch = "wasm32")]
pub mod web;

use iced_solstice::Renderer;
use iced_winit::{pick_list, Column, Command, Element, Length, Row, Text};

#[derive(Debug, Clone)]
pub enum Message {
    Selected(usize, spirits_within::Connection),
}

#[derive(Debug, Clone, Default)]
pub struct Application {
    selected: [Option<spirits_within::Connection>; 15],
    spirit_connections: [pick_list::State<spirits_within::Connection>; 15],
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}

impl iced_winit::Program for Application {
    type Renderer = Renderer;
    type Message = Message;
    // type Clipboard = iced_winit::Clipboard;

    fn update(
        &mut self,
        message: Self::Message,
        // _clipboard: &mut Self::Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Selected(index, connection) => {
                self.selected[index] = Some(connection);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Self::Renderer> {
        let mut spirits = self
            .spirit_connections
            .iter_mut()
            .zip(spirits_within::Spirit::list())
            .zip(self.selected.iter().copied())
            .enumerate()
            .map(|(index, ((state, spirit), selected))| {
                Column::new()
                    .width(Length::Fill)
                    .push(Text::new(format!("{:?}", spirit)))
                    .push(pick_list::PickList::new(
                        state,
                        &[
                            spirits_within::Connection::Ineptitude,
                            spirits_within::Connection::Competence,
                            spirits_within::Connection::Expertise,
                            spirits_within::Connection::Mastery,
                        ][..],
                        selected,
                        move |connection| Message::Selected(index, connection),
                    ))
                    .into()
            })
            .peekable();

        let mut root = Column::new()
            .push(
                iced_winit::Container::new(Text::new("test"))
                    .width(Length::Fill)
                    .style(CustomStyle::with_bg(iced_winit::Color::from([1., 0., 0.]))),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(2);

        while spirits.peek().is_some() {
            root = root
                .push(Row::with_children((&mut spirits).take(5).collect()).spacing(2))
                .width(Length::Fill);
        }

        if self.selected.iter().all(|s| s.is_some()) {
            let selected = self
                .selected
                .iter()
                .copied()
                .filter_map(|s| s)
                .collect::<Vec<_>>();
            let selected = spirits_within::SpiritSelection::try_from(selected);
            match selected {
                Ok(selected) => {
                    let stats = spirits_within::Stats::new(&selected);
                    let text = format!("{:#?}", stats);
                    root = root.push(Text::new(text));
                }
                Err(_) => {
                    eprintln!("OH GOD OH NO")
                }
            }
        }

        root.into()
    }
}

use iced_solstice::{widget, Background};
struct CustomStyle {
    style: widget::container::Style,
}

impl CustomStyle {
    pub fn with_bg(color: iced_winit::Color) -> Self {
        Self {
            style: widget::container::Style {
                background: Some(Background::Color(color)),
                ..Default::default()
            },
        }
    }
}

impl widget::container::StyleSheet for CustomStyle {
    fn style(&self) -> widget::container::Style {
        self.style
    }
}
