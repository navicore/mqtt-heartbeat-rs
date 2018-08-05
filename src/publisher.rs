use actix::*;

pub struct Publisher;
pub struct Report {
    pub json: String,
}
impl Message for Report {
    type Result = String;
}

impl Actor for Publisher {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("{}", "publisher started");
    }
}

impl Handler<Report> for Publisher {
    type Result = String; // <- Message response type

    fn handle(&mut self, msg: Report, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{}", msg.json);
        String::from("ok")
    }
}
