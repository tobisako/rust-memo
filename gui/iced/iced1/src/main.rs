use iced::{executor, Application, Column, Command, Element, Image, Length, Settings, Text};

struct Iced1;

impl Application for Iced1 {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Iced1, Command<Self::Message>) {
        (Iced1, Command::none())
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Hello, World!"))
            .push(Image::new("images/neko.png").width(Length::Units(240)))
            .into()
    }
}

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (420u32, 240u32);
    Iced1::run(settings);
}
