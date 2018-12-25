extern crate ws;
extern crate vlc;
extern crate serde_json;
extern crate notify_rust;

use ws::*;
use serde_json::Value;
use std::thread;
use std::str::FromStr;
use std::env;
use notify_rust::Notification;
use vlc::{Instance, Media, MediaPlayer};
use vlc::MediaPlayerAudioEx;   

const GRATI: ::ws::util::Token = ::ws::util::Token(1);

fn checkArgs( mini: &str, full: &str ) -> bool{
	let args: Vec<_> = env::args().collect();
	let maxArgs = args.len();
	let mut ret = 0;
	if maxArgs == 1{
		false
	}
	else{
	for value in args.iter(){
		if value == mini || value == full{
			ret = 1;
			break;
		}
		}
		if ret == 1{
			true
		}
		else{
		false
		}	
	}
}

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

if checkArgs("-n", "--notify"){

Notification::new()
    .summary("Listen-cli")
    .body(format!("{} - {} ", message["d"]["song"]["title"].as_str().unwrap().to_string(), message["d"]["song"]["artists"][0]["name"].as_str().unwrap().to_string()).as_str())
    .icon("play")
    .show().unwrap();}
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
   let args: Vec<_> = env::args().collect();
  println!("[SYS] Connecting...");
  let media_thread = thread::spawn( move || {
    let instance = Instance::new().unwrap();
    let stream = Media::new_location(&instance, "https://listen.moe/stream").unwrap();
    let player = MediaPlayer::new(&instance).unwrap();
    player.set_media(&stream);
    if args.len() != 1{
    if args[1].as_str() == "-v" || args[1].as_str() == "--volume" {
    let n: i32 = FromStr::from_str(&args[2]).unwrap();
    player.set_volume(n);
    }}
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
