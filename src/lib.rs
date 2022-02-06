#[cfg(target_arch = "wasm32")]
pub mod web;

use iced_winit::{
    widget::{pick_list, Button, Column, Row, Text},
    Command, Element, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    SpiritSelected(spirits_within::Spirit, SelectionOption),
    BasePrerogativeSelected(usize, PrerogativeOption),
    OptionalBurdenSelected(usize, BurdenOption),
    OptionalPrerogativeSelected(usize, PrerogativeOption),
    AddOptionalBurden,
    RemoveOptionalBurden,
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
    random_button: iced_winit::widget::button::State,
    reset_button: iced_winit::widget::button::State,

    selected: SpiritSelection,
    selection_options: Vec<SelectionOption>,

    prerogatives: PrerogativesState,
}

impl Application {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: rand::SeedableRng::seed_from_u64(seed),
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
                self.prerogatives.clear()
            }
            Message::Randomize => {
                use rand::Rng;
                self.selected = Default::default();
                self.update_selection_options();
                for spirit in spirits_within::Spirit::LIST {
                    let index = self.rng.gen_range(1..self.selection_options.len());
                    self.selected[spirit].selection = self.selection_options[index];
                    self.update_selection_options();
                }
                for (prerogative, _) in self.prerogatives.base_prerogatives.iter_mut() {
                    let index = self
                        .rng
                        .gen_range(0..spirits_within::Prerogative::LIST.len());
                    *prerogative = spirits_within::Prerogative::LIST[index].into();
                }
                self.prerogatives.optional_burdens.clear();
                for _index in 0..4 {
                    let burden_index = self.rng.gen_range(0..spirits_within::Burden::LIST.len());
                    let prerog_index = self
                        .rng
                        .gen_range(0..spirits_within::Prerogative::LIST.len());
                    self.prerogatives.optional_burdens.push(OptionalBurden {
                        burden: spirits_within::Burden::LIST[burden_index].into(),
                        burden_state: Default::default(),
                        prerogative: spirits_within::Prerogative::LIST[prerog_index].into(),
                        prerogative_state: Default::default(),
                    })
                }
            }
            Message::AddOptionalBurden => {
                self.prerogatives.optional_burdens.push(Default::default());
            }
            Message::RemoveOptionalBurden => {
                self.prerogatives.optional_burdens.pop();
            }
            Message::OptionalBurdenSelected(index, burden) => {
                self.prerogatives
                    .optional_burdens
                    .get_mut(index)
                    .map(|state| {
                        state.burden = burden;
                    });
            }
            Message::OptionalPrerogativeSelected(index, prerogative) => {
                self.prerogatives
                    .optional_burdens
                    .get_mut(index)
                    .map(|state| {
                        state.prerogative = prerogative;
                    });
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Self::Renderer> {
        let mut root = Column::new()
            .push(
                iced_winit::widget::Container::new(
                    Row::new()
                        .push(
                            Button::new(&mut self.random_button, Text::new("Randomize"))
                                .on_press(Message::Randomize),
                        )
                        .push(
                            Button::new(&mut self.reset_button, Text::new("Reset"))
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
                .push(Row::with_children(row).spacing(2));
        }

        if let Some(selected) = selection {
            match selected {
                Ok(selected) => {
                    let stats = spirits_within::BaseStats::new(&selected);
                    let text = format!("{:#?}", stats);
                    root = root.push(Text::new(text));
                    root = root.push(self.prerogatives.view(stats)).width(Length::Fill);
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
    pub fn none() -> Self {
        Self(None)
    }
}

impl From<spirits_within::Prerogative> for PrerogativeOption {
    fn from(v: spirits_within::Prerogative) -> Self {
        Self(Some(v))
    }
}

impl std::fmt::Display for PrerogativeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, ""),
            Some(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct BurdenOption(Option<spirits_within::Burden>);

impl BurdenOption {
    pub fn none() -> Self {
        Self(None)
    }
}

impl From<spirits_within::Burden> for BurdenOption {
    fn from(v: spirits_within::Burden) -> Self {
        Self(Some(v))
    }
}

impl std::fmt::Display for BurdenOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, ""),
            Some(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Debug, Clone)]
struct OptionalBurden {
    burden: BurdenOption,
    burden_state: pick_list::State<BurdenOption>,
    prerogative: PrerogativeOption,
    prerogative_state: pick_list::State<PrerogativeOption>,
}

impl Default for OptionalBurden {
    fn default() -> Self {
        Self {
            burden: BurdenOption::none(),
            burden_state: Default::default(),
            prerogative: Default::default(),
            prerogative_state: Default::default(),
        }
    }
}

impl OptionalBurden {
    fn zip(&self) -> Option<(spirits_within::Burden, spirits_within::Prerogative)> {
        self.burden.0.zip(self.prerogative.0)
    }
}

#[derive(Debug, Clone)]
struct PrerogativesState {
    base_prerogatives: [(PrerogativeOption, pick_list::State<PrerogativeOption>); 4],
    prerogative_options: Vec<PrerogativeOption>,

    burden_options: Vec<BurdenOption>,
    optional_burdens: Vec<OptionalBurden>,
    add_button: iced_winit::widget::button::State,
    remove_button: iced_winit::widget::button::State,
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
            prerogative_options: std::iter::once(PrerogativeOption::none())
                .chain(spirits_within::Prerogative::LIST.map(Into::into))
                .collect(),
            burden_options: std::iter::once(BurdenOption::none())
                .chain(spirits_within::Burden::LIST.map(Into::into))
                .collect(),
            optional_burdens: vec![],
            add_button: Default::default(),
            remove_button: Default::default(),
        }
    }

    fn clear(&mut self) {
        *self = Self::new()
    }

    fn update_options(&mut self, mut stats: spirits_within::BaseStats) {
        use spirits_within::{Prerogative, Stats};
        self.prerogative_options.clear();
        self.prerogative_options.push(PrerogativeOption::none());

        let iter = self
            .base_prerogatives
            .iter()
            .map(|(prerog, _)| prerog.0)
            .chain(self.optional_burdens.iter().map(|b| b.prerogative.0));
        for prerog in iter {
            match prerog {
                None => {}
                Some(prerog) => match prerog {
                    Prerogative::Conviction => stats.discipline += 3,
                    Prerogative::Education => stats.knowledge += 3,
                    Prerogative::Vocation => stats.proficiency += 3,
                    _ => {}
                },
            }
        }

        for prerog in spirits_within::Prerogative::LIST {
            let should_add = match prerog {
                Prerogative::Conviction => stats.discipline < Stats::MAX_DISCIPLINE,
                Prerogative::Education => stats.knowledge < Stats::MAX_KNOWLEDGE,
                Prerogative::Vocation => stats.proficiency < Stats::MAX_PROFICIENCY,
                _ => true,
            };
            if should_add {
                self.prerogative_options.push(prerog.into());
            }
        }
    }

    fn view(
        &mut self,
        base_stats: spirits_within::BaseStats,
    ) -> Element<'_, Message, <Application as iced_winit::Program>::Renderer> {
        self.update_options(base_stats);

        let stats = if self
            .base_prerogatives
            .iter()
            .all(|(PrerogativeOption(p), _)| p.is_some())
        {
            let prerogatives = self
                .base_prerogatives
                .clone()
                .map(|(PrerogativeOption(p), _)| p.unwrap());
            let mut pb = spirits_within::PrerogativesAndBurdens::new(prerogatives);
            for optional_burden in self.optional_burdens.iter() {
                if let Some((b, p)) = optional_burden.zip() {
                    pb.add_burden(b, p);
                }
            }
            let stats = base_stats.with_prerogatives_and_burdens(&pb);
            match stats {
                Ok(stats) => Some(Text::new(format!("{:#?}", stats))),
                Err(_) => None,
            }
        } else {
            None
        };

        let base_prerogs = Row::with_children(
            self.base_prerogatives
                .iter_mut()
                .enumerate()
                .map(|(index, (selection, state))| {
                    iced_winit::widget::Container::new(pick_list::PickList::new(
                        state,
                        &self.prerogative_options,
                        Some(*selection),
                        move |prerogative| Message::BasePrerogativeSelected(index, prerogative),
                    ))
                    .align_y(iced_winit::alignment::Vertical::Top)
                    .width(Length::Fill)
                    .into()
                })
                .collect(),
        )
        .width(Length::Fill);

        let optional_prerogs = if stats.is_some() {
            let buttons = {
                let add_text = Text::new("+");
                let add = Button::new(&mut self.add_button, add_text).width(Length::Fill);
                let add = if self.optional_burdens.len() < 4 {
                    add.on_press(Message::AddOptionalBurden)
                } else {
                    add
                };
                let remove_text = Text::new("-");
                let remove = Button::new(&mut self.remove_button, remove_text).width(Length::Fill);
                let remove = if self.optional_burdens.is_empty() {
                    remove
                } else {
                    remove.on_press(Message::RemoveOptionalBurden)
                };
                Column::with_children(vec![add.into(), remove.into()])
                    .width(Length::Fill)
                    .into()
            };

            let mut optionals = self
                .optional_burdens
                .iter_mut()
                .enumerate()
                .map(|(index, optional_burden)| {
                    let burden_picker = pick_list::PickList::new(
                        &mut optional_burden.burden_state,
                        &self.burden_options,
                        Some(optional_burden.burden),
                        move |burden| Message::OptionalBurdenSelected(index, burden),
                    )
                    .width(Length::Fill)
                    .into();
                    let prerogative_picker = pick_list::PickList::new(
                        &mut optional_burden.prerogative_state,
                        &self.prerogative_options,
                        Some(optional_burden.prerogative),
                        move |prerog| Message::OptionalPrerogativeSelected(index, prerog),
                    )
                    .width(Length::Fill)
                    .into();
                    Column::with_children(vec![burden_picker, prerogative_picker])
                        .width(Length::Fill)
                        .into()
                })
                .collect::<Vec<_>>();
            while optionals.len() < 4 {
                optionals.push(iced_winit::widget::Space::new(Length::Fill, Length::Shrink).into());
            }
            let optionals = Row::with_children(optionals)
                .spacing(2)
                .width(Length::FillPortion(20))
                .into();

            Row::with_children(vec![buttons, optionals])
                .spacing(2)
                .width(Length::Fill)
                .into()
        } else {
            iced_winit::widget::Space::new(Length::Fill, Length::Shrink).into()
        };

        let titled_prerogs = Column::with_children(vec![
            Row::new()
                .push(Text::new("Prerogatives And Burdens").size(32))
                .padding(5)
                .into(),
            Row::with_children(vec![base_prerogs.into(), optional_prerogs])
                .width(Length::Fill)
                .into(),
        ])
        .width(Length::Fill);
        match stats {
            Some(stats) => Column::new()
                .push(titled_prerogs)
                .push(stats)
                .width(Length::Fill)
                .into(),
            None => titled_prerogs.into(),
        }
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
