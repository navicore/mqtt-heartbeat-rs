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
mod heartbeater;
mod publisher;
mod settings;

fn main() {
    let system = actix::System::new("test");

    let paddr = Publisher.start();
    let addr = Heartbeater {
        publisher: paddr.recipient(),
    }.start();
    let res = addr.send(Beat());
    Arbiter::spawn(res.then(|res| {
        match res {
            Ok(result) => println!("Beat: {}", result),
            Err(err) => panic!("Bad beat: {}", err),
        }

        //System::current().stop();
        future::result(Ok(()))
    }));
    system.run();
}
