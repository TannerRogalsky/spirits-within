mod charactor_creator;
mod game;
#[cfg(target_arch = "wasm32")]
pub mod web;

#[derive(Debug, Clone)]
pub enum ApplicationState {
    CharacterCreator,
    Game,
}

#[derive(Debug, Clone)]
pub struct Application {
    rng: rand::rngs::SmallRng,
    state: ApplicationState,
    game: game::Game,
    character_creator: charactor_creator::CharacterCreator,

    game_button: iced_winit::widget::button::State,
    character_creator_button: iced_winit::widget::button::State,
}

impl Application {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: rand::SeedableRng::seed_from_u64(seed),
            state: ApplicationState::CharacterCreator,
            game: Default::default(),
            character_creator: Default::default(),
            game_button: Default::default(),
            character_creator_button: Default::default(),
        }
    }
}

impl iced_winit::Program for Application {
    type Renderer = iced_solstice::Renderer;
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced_winit::Command<Self::Message> {
        match message {
            Message::TransitionCharacterCreator => {
                self.state = ApplicationState::CharacterCreator;
            }
            Message::TransitionGame => {
                self.state = ApplicationState::Game;
            }
            Message::CharactorCreator(message) => {
                self.character_creator.update(message, &mut self.rng);
            }
            Message::Game(message) => self.game.update(message),
        }

        iced_winit::Command::none()
    }

    fn view(&mut self) -> iced_winit::Element<'_, Self::Message, Self::Renderer> {
        use iced_winit::{
            widget::{Button, Column, Row, Text},
            Length,
        };
        let content = match &mut self.state {
            ApplicationState::CharacterCreator => self.character_creator.view().map(Into::into),
            ApplicationState::Game => self.game.view().map(Into::into),
        };
        Column::new()
            .push(
                iced_winit::widget::Container::new(
                    Row::new()
                        .push(iced_winit::widget::Space::with_width(Length::Units(2)))
                        .push(
                            Button::new(&mut self.game_button, Text::new("Game"))
                                .on_press(Message::TransitionGame),
                        )
                        .push(
                            Button::new(
                                &mut self.character_creator_button,
                                Text::new("Character Creator"),
                            )
                            .on_press(Message::TransitionCharacterCreator),
                        )
                        .spacing(2),
                )
                .width(Length::Fill),
            )
            .push(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(2)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TransitionCharacterCreator,
    TransitionGame,
    CharactorCreator(charactor_creator::Message),
    Game(game::Message),
}

impl From<charactor_creator::Message> for Message {
    fn from(inner: charactor_creator::Message) -> Self {
        Message::CharactorCreator(inner)
    }
}

impl From<game::Message> for Message {
    fn from(inner: game::Message) -> Self {
        Message::Game(inner)
    }
}
