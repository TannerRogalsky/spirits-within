mod charactor_creator;
#[cfg(target_arch = "wasm32")]
pub mod web;

#[derive(Debug, Clone)]
pub enum Message {
    CharactorCreator(charactor_creator::Message),
}

impl From<charactor_creator::Message> for Message {
    fn from(inner: charactor_creator::Message) -> Self {
        Message::CharactorCreator(inner)
    }
}

#[derive(Debug, Clone)]
pub enum ApplicationState {
    CharacterCreator(charactor_creator::CharacterCreator),
}

#[derive(Debug, Clone)]
pub struct Application {
    rng: rand::rngs::SmallRng,
    state: ApplicationState,
}

impl Application {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: rand::SeedableRng::seed_from_u64(seed),
            state: ApplicationState::CharacterCreator(charactor_creator::CharacterCreator::new()),
        }
    }
}

impl iced_winit::Program for Application {
    type Renderer = iced_solstice::Renderer;
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced_winit::Command<Self::Message> {
        match (&mut self.state, message) {
            (ApplicationState::CharacterCreator(inner), Message::CharactorCreator(message)) => {
                inner.update(message, &mut self.rng);
            }
            _ => log::error!("Dispatched wrong message type for application state.")
        }

        iced_winit::Command::none()
    }

    fn view(&mut self) -> iced_winit::Element<'_, Self::Message, Self::Renderer> {
        match &mut self.state {
            ApplicationState::CharacterCreator(inner) => inner.view().map(Into::into),
        }
    }
}
