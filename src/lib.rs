#[cfg(target_arch = "wasm32")]
pub mod web;

use iced_solstice::Renderer;
use iced_winit::{pick_list, Column, Command, Element, Length, Row, Text};

#[derive(Debug, Clone)]
pub enum Message {
    Selected(usize, SelectionOption),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SelectionOption {
    None,
    Mastery,
    Expertise,
    Competence,
    Ineptitude,
}

impl SelectionOption {
    pub fn is_some(self) -> bool {
        self != SelectionOption::None
    }
}

impl Default for SelectionOption {
    fn default() -> Self {
        Self::None
    }
}

impl From<SelectionOption> for Option<spirits_within::Connection> {
    fn from(v: SelectionOption) -> Self {
        match v {
            SelectionOption::None => None,
            SelectionOption::Mastery => Some(spirits_within::Connection::Mastery),
            SelectionOption::Expertise => Some(spirits_within::Connection::Expertise),
            SelectionOption::Competence => Some(spirits_within::Connection::Competence),
            SelectionOption::Ineptitude => Some(spirits_within::Connection::Ineptitude),
        }
    }
}

impl std::fmt::Display for SelectionOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match Into::<Option<spirits_within::Connection>>::into(*self) {
            Some(connection) => connection.fmt(f),
            None => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Application {
    selected: [SelectionOption; 15],
    spirit_connections: [pick_list::State<SelectionOption>; 15],
    selection_options: Vec<SelectionOption>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            selection_options: vec![
                SelectionOption::None,
                SelectionOption::Mastery,
                SelectionOption::Expertise,
                SelectionOption::Competence,
                SelectionOption::Ineptitude,
            ],
            ..Default::default()
        }
    }

    fn update_selection_options(&mut self) {
        self.selection_options.clear();
        self.selection_options.push(SelectionOption::None);
        let mut mastery = 0;
        let mut expertise = 0;
        let mut competence = 0;
        let mut inept = 0;
        for selected in self.selected.iter() {
            match selected {
                SelectionOption::None => {}
                SelectionOption::Mastery => mastery += 1,
                SelectionOption::Expertise => expertise += 1,
                SelectionOption::Competence => competence += 1,
                SelectionOption::Ineptitude => inept += 1,
            }
        }
        if mastery < spirits_within::SpiritSelection::MASTERY_COUNT {
            self.selection_options.push(SelectionOption::Mastery);
        }
        if expertise < spirits_within::SpiritSelection::EXPERTISE_COUNT {
            self.selection_options.push(SelectionOption::Expertise);
        }
        if competence < spirits_within::SpiritSelection::COMPETENCE_COUNT {
            self.selection_options.push(SelectionOption::Competence);
        }
        if inept < spirits_within::SpiritSelection::INEPTITUDE_COUNT {
            self.selection_options.push(SelectionOption::Ineptitude);
        }
    }
}

impl iced_winit::Program for Application {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Selected(index, connection) => {
                self.selected[index] = connection;
                self.update_selection_options();
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
                        &self.selection_options,
                        Some(selected),
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
                .filter_map(|s| s.into())
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
