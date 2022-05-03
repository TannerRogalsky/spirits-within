use iced_winit::{widget::Text, Element};

#[derive(Debug, Clone)]
pub enum Message {
    Test,
}

#[derive(Debug, Default, Clone)]
pub struct Game;

impl Game {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Test => unimplemented!(),
        }
    }

    pub fn view(
        &mut self,
    ) -> Element<'_, Message, <crate::Application as iced_winit::Program>::Renderer> {
        Text::new("unimplemented!").into()
    }
}
