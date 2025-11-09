mod mic_monitor;

pub fn main() -> iced::Result {
    iced::application("AudioMeter - Never be too loud again", AudioMeter::update, AudioMeter::view)
        .subscription(AudioMeter::subscription)
        .theme(AudioMeter::theme)
        .run()
}

#[derive(Default)]
struct AudioMeter {
    too_loud: bool,
    limit: f32,
    last_max_volume: f32,
}

#[derive(Debug, Clone)]
enum Message {
    MicMonitorUpdate(mic_monitor::Event),
    LimitChanged(f32),
}

impl AudioMeter {
    fn update(&mut self, message: Message) {
        match message {
            Message::MicMonitorUpdate(event) => { 
                match event {
                    mic_monitor::Event::UpdateLevel { max_volume } => {
                        self.too_loud = max_volume >= self.limit;
                        self.last_max_volume = max_volume;
                    },
                } 
            }
            Message::LimitChanged(limit) => {
                println!("Recieved message {:?}", message);
                self.limit = limit
            },
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::run(mic_monitor::connect).map(Message::MicMonitorUpdate)
    }

    fn view(&'_ self) -> iced::Element<'_, Message> {
        let message = format!("Too loud: {}", self.too_loud);
        let text_widget = iced::widget::text(message);
        let limit_slider = iced::widget::slider(0.0..=1.0, self.limit, Message::LimitChanged).step(0.01);
        let volume_slider = iced::widget::slider(0.0..=1.0, self.last_max_volume, Message::LimitChanged).step(0.01);
        iced::widget::container(iced::widget::column![text_widget, limit_slider, volume_slider])
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}