#[cfg(target_arch = "wasm32")]
pub mod web;

use iced_winit::{pick_list, Column, Command, Element, Length, Row, Text};

#[derive(Debug, Clone)]
pub enum Message {
    SpiritSelected(spirits_within::Spirit, SelectionOption),
    BasePrerogativeSelected(usize, PrerogativeOption),
    Reset,
    Randomize,
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

#[derive(Debug, Clone)]
struct SelectionState {
    spirit: spirits_within::Spirit,
    selection: SelectionOption,
    state: pick_list::State<SelectionOption>,
}

impl SelectionState {
    pub fn spirit(spirit: spirits_within::Spirit) -> Self {
        Self {
            spirit,
            selection: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct SpiritSelection {
    the_veil: SelectionState,
    mirror: SelectionState,
    the_path: SelectionState,
    shadows: SelectionState,
    instinct: SelectionState,
    reason: SelectionState,
    whispers: SelectionState,
    respect: SelectionState,
    drama: SelectionState,
    motion: SelectionState,
    muscle: SelectionState,
    kinesis: SelectionState,
    glamour: SelectionState,
    balance: SelectionState,
    the_pulse: SelectionState,
}

impl Default for SpiritSelection {
    fn default() -> Self {
        use spirits_within::Spirit::*;
        Self {
            the_veil: SelectionState::spirit(TheVeil),
            mirror: SelectionState::spirit(Mirror),
            the_path: SelectionState::spirit(ThePath),
            shadows: SelectionState::spirit(Shadows),
            instinct: SelectionState::spirit(Instinct),
            reason: SelectionState::spirit(Reason),
            whispers: SelectionState::spirit(Whispers),
            respect: SelectionState::spirit(Respect),
            drama: SelectionState::spirit(Drama),
            motion: SelectionState::spirit(Motion),
            muscle: SelectionState::spirit(Muscle),
            kinesis: SelectionState::spirit(Kinesis),
            glamour: SelectionState::spirit(Glamour),
            balance: SelectionState::spirit(Balance),
            the_pulse: SelectionState::spirit(ThePulse),
        }
    }
}

impl<'a> IntoIterator for &'a SpiritSelection {
    type Item = &'a SelectionState;
    type IntoIter = std::array::IntoIter<Self::Item, 15>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([
            &self.the_veil,
            &self.mirror,
            &self.the_path,
            &self.shadows,
            &self.instinct,
            &self.reason,
            &self.whispers,
            &self.respect,
            &self.drama,
            &self.motion,
            &self.muscle,
            &self.kinesis,
            &self.glamour,
            &self.balance,
            &self.the_pulse,
        ])
    }
}

impl<'a> IntoIterator for &'a mut SpiritSelection {
    type Item = &'a mut SelectionState;
    type IntoIter = std::array::IntoIter<Self::Item, 15>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([
            &mut self.the_veil,
            &mut self.mirror,
            &mut self.the_path,
            &mut self.shadows,
            &mut self.instinct,
            &mut self.reason,
            &mut self.whispers,
            &mut self.respect,
            &mut self.drama,
            &mut self.motion,
            &mut self.muscle,
            &mut self.kinesis,
            &mut self.glamour,
            &mut self.balance,
            &mut self.the_pulse,
        ])
    }
}

impl std::ops::Index<spirits_within::Spirit> for SpiritSelection {
    type Output = SelectionState;

    fn index(&self, index: spirits_within::Spirit) -> &Self::Output {
        use spirits_within::Spirit::*;
        match index {
            TheVeil => &self.the_veil,
            Mirror => &self.mirror,
            ThePath => &self.the_path,
            Shadows => &self.shadows,
            Instinct => &self.instinct,
            Reason => &self.reason,
            Whispers => &self.whispers,
            Respect => &self.respect,
            Drama => &self.drama,
            Motion => &self.motion,
            Muscle => &self.muscle,
            Kinesis => &self.kinesis,
            Glamour => &self.glamour,
            Balance => &self.balance,
            ThePulse => &self.the_pulse,
        }
    }
}

impl std::ops::IndexMut<spirits_within::Spirit> for SpiritSelection {
    fn index_mut(&mut self, index: spirits_within::Spirit) -> &mut Self::Output {
        use spirits_within::Spirit::*;
        match index {
            TheVeil => &mut self.the_veil,
            Mirror => &mut self.mirror,
            ThePath => &mut self.the_path,
            Shadows => &mut self.shadows,
            Instinct => &mut self.instinct,
            Reason => &mut self.reason,
            Whispers => &mut self.whispers,
            Respect => &mut self.respect,
            Drama => &mut self.drama,
            Motion => &mut self.motion,
            Muscle => &mut self.muscle,
            Kinesis => &mut self.kinesis,
            Glamour => &mut self.glamour,
            Balance => &mut self.balance,
            ThePulse => &mut self.the_pulse,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Application {
    rng: rand::rngs::SmallRng,
    random_button: iced_winit::button::State,
    reset_button: iced_winit::button::State,

    selected: SpiritSelection,
    selection_options: Vec<SelectionOption>,

    prerogatives: PrerogativesState,
}

impl Application {
    pub fn new() -> Self {
        Self {
            rng: rand::SeedableRng::seed_from_u64(0),
            random_button: Default::default(),
            reset_button: Default::default(),
            selection_options: vec![
                SelectionOption::None,
                SelectionOption::Mastery,
                SelectionOption::Expertise,
                SelectionOption::Competence,
                SelectionOption::Ineptitude,
            ],
            selected: SpiritSelection::default(),
            prerogatives: PrerogativesState::new(),
        }
    }

    fn update_selection_options(&mut self) {
        self.selection_options.clear();
        self.selection_options.push(SelectionOption::None);
        let mut mastery = 0;
        let mut expertise = 0;
        let mut competence = 0;
        let mut inept = 0;
        for SelectionState { selection, .. } in &self.selected {
            match selection {
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
    type Renderer = iced_solstice::Renderer;
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SpiritSelected(index, connection) => {
                let SelectionState { selection, .. } = &mut self.selected[index];
                *selection = connection;
                self.update_selection_options();
            }
            Message::BasePrerogativeSelected(index, selection) => {
                self.prerogatives.base_prerogatives[index].0 = selection;
            }
            Message::Reset => {
                self.selected = Default::default();
                self.update_selection_options();
            }
            Message::Randomize => {
                self.selected = Default::default();
                self.update_selection_options();
                for spirit in spirits_within::Spirit::LIST {
                    use rand::Rng;
                    let index = self.rng.gen_range(1..self.selection_options.len());
                    self.selected[spirit].selection = self.selection_options[index];
                    self.update_selection_options();
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Self::Renderer> {
        let mut root = Column::new()
            .push(
                iced_winit::Container::new(
                    iced_winit::Row::new()
                        .push(
                            iced_winit::Button::new(
                                &mut self.random_button,
                                Text::new("Randomize"),
                            )
                            .on_press(Message::Randomize),
                        )
                        .push(
                            iced_winit::Button::new(&mut self.reset_button, Text::new("Reset"))
                                .on_press(Message::Reset)
                                .style(CustomStyle::with_bg(iced_winit::Color::from([
                                    0.9, 0.2, 0.3,
                                ]))),
                        ),
                )
                .align_x(iced_winit::alignment::Horizontal::Right)
                .width(Length::Fill)
                .style(CustomStyle::with_bg(iced_winit::Color::from([
                    0.7, 0.7, 0.7,
                ]))),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(2);

        let selection = (&self.selected)
            .into_iter()
            .all(|state| state.selection.is_some())
            .then(|| {
                spirits_within::SpiritSelection::try_from_iter(
                    spirits_within::Spirit::LIST.into_iter().map(
                        |spirit: spirits_within::Spirit| {
                            (
                                spirit,
                                Into::<Option<spirits_within::Connection>>::into(
                                    self.selected[spirit].selection,
                                )
                                .unwrap(),
                            )
                        },
                    ),
                )
            });

        struct Rows<T> {
            proficiency: Vec<T>,
            knowledge: Vec<T>,
            discipline: Vec<T>,
        }

        impl<T> Default for Rows<T> {
            fn default() -> Self {
                Self {
                    proficiency: Vec::with_capacity(5),
                    knowledge: Vec::with_capacity(5),
                    discipline: Vec::with_capacity(5),
                }
            }
        }

        impl<T> IntoIterator for Rows<T> {
            type Item = (spirits_within::Stat, Vec<T>);
            type IntoIter = std::array::IntoIter<Self::Item, 3>;

            fn into_iter(self) -> Self::IntoIter {
                use spirits_within::Stat::*;
                [
                    (Proficiency, self.proficiency),
                    (Knowledge, self.knowledge),
                    (Discipline, self.discipline),
                ]
                .into_iter()
            }
        }

        let mut rows = Rows::default();
        for state in &mut self.selected {
            let SelectionState {
                spirit,
                selection,
                ref mut state,
            } = *state;
            use spirits_within::Stat;
            let row = match spirit.stat() {
                Stat::Knowledge => &mut rows.knowledge,
                Stat::Discipline => &mut rows.discipline,
                Stat::Proficiency => &mut rows.proficiency,
            };
            row.push(
                Column::new()
                    .align_items(iced_winit::Alignment::Center)
                    .width(Length::Fill)
                    .push(Text::new(format!("{:?}", spirit)))
                    .push(pick_list::PickList::new(
                        state,
                        &self.selection_options,
                        Some(selection),
                        move |connection| Message::SpiritSelected(spirit, connection),
                    ))
                    .into(),
            )
        }

        for (stat, row) in rows {
            root = root
                .push(
                    Row::new()
                        .push(Text::new(format!("{:?}", stat)).size(32))
                        .padding(5),
                )
                .push(Row::with_children(row).spacing(2))
                .width(Length::Fill);
        }

        if let Some(selected) = selection {
            match selected {
                Ok(selected) => {
                    let stats = spirits_within::BaseStats::new(&selected);
                    let text = format!("{:#?}", stats);
                    root = root.push(Text::new(text));
                    root = root.push(self.prerogatives.view(stats));
                }
                Err(err) => {
                    log::error!("OH GOD OH NO: {:?}", err);
                }
            }
        }

        root.into()
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct PrerogativeOption(Option<spirits_within::Prerogative>);

impl PrerogativeOption {
    const LIST: [PrerogativeOption; 4] = [
        Self(None),
        Self(Some(spirits_within::Prerogative::Conviction)),
        Self(Some(spirits_within::Prerogative::Education)),
        Self(Some(spirits_within::Prerogative::Vocation)),
    ];
}

impl std::fmt::Display for PrerogativeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, ""),
            Some(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Debug, Clone)]
struct PrerogativesState {
    base_prerogatives: [(PrerogativeOption, pick_list::State<PrerogativeOption>); 4],
}

impl PrerogativesState {
    pub fn new() -> Self {
        Self {
            base_prerogatives: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
        }
    }

    fn view(
        &mut self,
        base_stats: spirits_within::BaseStats,
    ) -> Element<'_, Message, <Application as iced_winit::Program>::Renderer> {
        let stats = if self
            .base_prerogatives
            .iter()
            .all(|(PrerogativeOption(p), _)| p.is_some())
        {
            let prerogatives = self
                .base_prerogatives
                .clone()
                .map(|(PrerogativeOption(p), _)| p.unwrap());
            let pb = spirits_within::PrerogativesAndBurdens::new(prerogatives);
            let stats = base_stats.with_prerogatives_and_burdens(&pb);
            match stats {
                Ok(stats) => Some(Element::from(Text::new(format!("{:#?}", stats)))),
                Err(_) => None,
            }
        } else {
            None
        };

        let base_prerogs =
            self.base_prerogatives
                .iter_mut()
                .enumerate()
                .map(|(index, (selection, state))| {
                    pick_list::PickList::new(
                        state,
                        &PrerogativeOption::LIST[..],
                        Some(*selection),
                        move |prerogative| Message::BasePrerogativeSelected(index, prerogative),
                    )
                    .into()
                });
        let children = match stats {
            Some(stats) => base_prerogs.chain(std::iter::once(stats)).collect(),
            None => base_prerogs.collect(),
        };
        Row::with_children(children).width(Length::Fill).into()
    }
}

struct CustomStyle {
    style: iced_solstice::widget::container::Style,
}

impl CustomStyle {
    pub fn with_bg(color: iced_winit::Color) -> Self {
        Self {
            style: iced_solstice::widget::container::Style {
                background: Some(iced_solstice::Background::Color(color)),
                ..Default::default()
            },
        }
    }
}

impl iced_solstice::widget::container::StyleSheet for CustomStyle {
    fn style(&self) -> iced_solstice::widget::container::Style {
        self.style
    }
}

impl iced_solstice::button::StyleSheet for CustomStyle {
    fn active(&self) -> iced_solstice::button::Style {
        iced_solstice::button::Style {
            shadow_offset: Default::default(),
            background: self.style.background,
            border_radius: self.style.border_radius,
            border_width: self.style.border_width,
            border_color: iced_winit::Color::BLACK,
            text_color: self.style.text_color.unwrap_or(iced_winit::Color::BLACK),
        }
    }
}
