use iced::alignment::Horizontal;
use iced::{Application, Command, Element, Length, Text};

#[derive(Debug, Default)]
struct State;

pub struct GuessWord {
    state: State,
}

impl Application for GuessWord {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GuessWord, Command<Self::Message>) {
        (
            GuessWord {
                state: State::default(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        "GuessWord".to_string()
    }
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
    fn view(&mut self) -> Element<Self::Message> {
        Text::new("GuessWord")
            .width(Length::Fill)
            .size(60)
            .color([0.5; 3])
            .horizontal_alignment(Horizontal::Center)
            .into()
    }
}
