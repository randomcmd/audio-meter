use std::cmp::Ordering;

use iced::futures::sink::SinkExt;
use audio_recorder_rs::Recorder;

pub fn connect() -> impl iced::futures::Stream<Item = Event> {
    iced::stream::channel(100, |mut output| async move {
        println!("Started mic monitor channel!");
        let mut state = State {};

        let mut recorder = Recorder::new();
        let reciever = recorder.start(true).expect("Failed to start recording");

        loop {
            if let Ok(samples) = reciever.recv() {
                let max_volume = samples.iter().max_by(|a, b| {
                    a.partial_cmp(b).unwrap_or(Ordering::Less)
                });
                let send_result = output.send(Event::UpdateLevel { max_volume: *max_volume.unwrap_or(&0.0) }).await;
            }
        }
    })
}

#[derive(Debug)]
struct State {
    
}

#[derive(Debug, Clone)]
pub enum Event {
    UpdateLevel { max_volume: f32 },
}