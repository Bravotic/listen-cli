extern crate ws;
extern crate vlc;
extern crate serde_json;

use ws::*;
use serde_json::Value;
use std::thread;

use vlc::{Instance, Media, MediaPlayer};

const GRATI: ::ws::util::Token = ::ws::util::Token(1);

struct UpdateHandler{
  sender: Sender
}

impl Handler for UpdateHandler{
  fn on_open(&mut self, _: Handshake) -> Result<()>{
    self.sender.send("{\"op\":0,\"d\":{\"auth\":\"Bearer null\"}}").unwrap();
    Ok(())
  }

  fn on_message(&mut self, msg: Message) -> Result<()>{
   if let Message::Text(message_text) = msg {

    let message: Value = ::serde_json::from_str(&message_text).unwrap();

    if message["op"] == 0 {
      println!("[SYS] Connection Sucessful");
      println!("[0] {}", message["d"]["message"].as_str().unwrap().to_string());
      //println!("{}", message_text);
      let heartbeat = message["d"]["heartbeat"].as_u64().unwrap();
      self.sender.timeout(heartbeat, GRATI); 
    }

    else if message["op"] == 1{
      println!("[1] Currently playing: {} - {} ", message["d"]["song"]["title"].as_str().unwrap().to_string(), message["d"]["song"]["artists"][0]["name"].as_str().unwrap().to_string());
    }
    }

    Ok(()) 
  }

  fn on_timeout(&mut self, _: ::ws::util::Token) -> Result<()>{
    self.sender.send("{\"op\":9}");
    //println!("[9] Ping");
    self.sender.timeout(45000, GRATI)
  }
}
fn main() {

  println!("[SYS] Connecting...");
  let media_thread = thread::spawn( move || {
    let instance = Instance::new().unwrap();
    let stream = Media::new_location(&instance, "https://listen.moe/stream").unwrap();
    let player = MediaPlayer::new(&instance).unwrap();
    player.set_media(&stream);
    player.play().unwrap();
    thread::sleep(::std::time::Duration::from_secs(10000000000000000000));
  });
  let update_reel = thread::spawn( move || {
    connect("wss://listen.moe/gateway", |out| {
    UpdateHandler {
         sender: out
      }
    }).unwrap()
  });

update_reel.join();
media_thread.join();

}
