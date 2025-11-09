use iced::futures::sink::SinkExt;

pub fn connect() -> impl iced::futures::Stream<Item = Event> {
    iced::stream::channel(100, |mut output| async move {
        println!("Started mic monitor channel!");
        let mut state = State {};

        loop {
            let send_result = output.send(Event::UpdateLevel { too_loud: false }).await;
        }
    })
}

#[derive(Debug)]
struct State {
    
}

#[derive(Debug, Clone)]
pub enum Event {
    UpdateLevel { too_loud: bool },
}

#[derive(Debug, Clone)]
pub struct Connection(iced::futures::channel::mpsc::Sender<Message>);

impl Connection {
    pub fn send(&mut self, message: Message) {

    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateLevel { too_loud: bool },
}