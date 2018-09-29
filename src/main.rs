#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate config;
extern crate futures;
extern crate mqttc;
extern crate netopt;
extern crate serde;
extern crate ws;
use actix::*;
use heartbeater::{beat, Heartbeater};
use publisher::new_client;
use publisher::Publisher;
use settings::Settings;
use std::thread;
use std::time::Duration;
mod heartbeater;
mod publisher;
mod settings;

fn main() {
    let settings = Settings::new().unwrap();
    let out_client = settings.out_client;
    let out_topic = settings.out_topic;
    let delay_seconds = settings.delay_seconds;
    let system = actix::System::new("heartbeat");

    let p_actor = Publisher {
        client: new_client(out_client),
        topic: out_topic,
    }.start();
    let h_actor = Heartbeater {
        publisher: p_actor.recipient(),
    }.start();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(delay_seconds));
        let a = h_actor.clone();
        beat(a.recipient());
    });

    system.run();
}
