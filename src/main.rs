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
use futures::{future, Future};
use heartbeater::{Beat, Heartbeater};
use publisher::Publisher;
use settings::Settings;
mod heartbeater;
mod publisher;
mod settings;
use std::thread;
use std::time::Duration;

fn beat(addr: Recipient<Beat>) {
    let res = addr.send(Beat());
    Arbiter::spawn(res.then(|res| {
        match res {
            Ok(result) => println!("Beat: {}", result),
            Err(err) => panic!("Bad beat: {}", err),
        }

        future::result(Ok(()))
    }));
}

fn main() {
    let system = actix::System::new("test");

    let p_actor = Publisher.start();
    let h_actor = Heartbeater {
        publisher: p_actor.recipient(),
    }.start();

    let settings = Settings::new().unwrap();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(settings.delay_seconds));
        let a = h_actor.clone();
        beat(a.recipient());
    });

    system.run();
}
