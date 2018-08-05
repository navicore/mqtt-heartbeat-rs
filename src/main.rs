#[macro_use]
extern crate serde_derive;
extern crate config;
extern crate mqttc;
extern crate netopt;
extern crate serde;
extern crate ws;

//use mqttc::{PubOpt, PubSub};
use settings::Settings;
//use std::str;
//use std::sync::Arc;
//use std::time;
mod settings;

fn main() {
    let settings = Settings::new().unwrap();

    let json = format!(
        r#"{{"heartbeat": "{} {}"}}"#,
        settings.heartbeat_template, "me"
    );
    println!("{}", json);
}
