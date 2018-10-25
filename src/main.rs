extern crate ws;

use ws::{connect, CloseCode};
use ws::Handler;
use ws::Message;
use ws::Result;
use ws::Sender;

struct MyHandler {
    counter: usize,
    out: Sender,
}

impl MyHandler {
    fn new(out: Sender) -> MyHandler {
        MyHandler{ counter: 0, out: out }
    }
}

impl Handler for MyHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Received message {:?}", msg);
        println!("counter {:?}", self.counter);
        self.counter += 1;

        if self.counter < 10 {
            self.out.send("Hello WebSocket")
        } else {
            self.out.close(CloseCode::Normal)
        }
    }
}

fn create_connection() {
  connect("ws://127.0.0.1:3012", |out| {
      out.send("Hello WebSocket").unwrap();

      MyHandler::new(out)
  }).unwrap()
}

fn main() {
    create_connection();
} 
