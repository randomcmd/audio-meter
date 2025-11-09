mod mic_monitor;

pub fn main() -> iced::Result {
    iced::application("AudioMeter - Never be too loud again", AudioMeter::update, AudioMeter::view)
        .subscription(AudioMeter::subscription)
        .theme(AudioMeter::theme)
        .run()
}

#[derive(Default)]
struct AudioMeter {
    too_loud: bool
}

#[derive(Debug, Clone)]
enum Message {
    MicMonitorUpdate(mic_monitor::Event)
}

impl AudioMeter {
    fn update(&mut self, message: Message) {
        // println!("Recieved message {:?}", message);
        match message {
            Message::MicMonitorUpdate(event) => { 
                match event {
                    mic_monitor::Event::UpdateLevel { too_loud } => {
                        self.too_loud = too_loud
                    },
                } 
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::run(mic_monitor::connect).map(Message::MicMonitorUpdate)
    }

    fn view(&self) -> iced::Element<Message> {
        let message = format!("Too loud: {}", self.too_loud);
        let text_widget = iced::widget::text(message);
        iced::widget::container(iced::widget::column![text_widget])
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}