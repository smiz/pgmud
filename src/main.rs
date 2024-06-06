use std::{
	io::{prelude::*,ErrorKind,stdout},
	net::{TcpListener,TcpStream},
	thread,
	sync::{Arc,Mutex},
	time::{Duration,SystemTime}
};
use uuid::Uuid;

mod location;
mod map;
mod mobile;
mod object;
mod events;
mod message;
mod dice;
use crate::map::*;
use crate::object::*;
use crate::events::*;
use crate::message::*;
use crate::mobile::*;

// Tick in milliseconds
const TICK : u16 = 250;
// Sub ticks for interval between checking
// messages, getting input, and so forth
const SUB_TICK : u16 = 50;

struct Position
{
	pub x: i16,
	pub y: i16,
	pub uuid: Uuid
}

fn goto(position: Position, dx: i16, dy: i16, event_q: &mut EventList) -> Position
{
	let new_position = Position { x: position.x+dx, y: position.y+dy, uuid: position.uuid };
	let move_event = Box::new(MoveMobileEvent
		{
			uuid: position.uuid,
			x: position.x,
			y: position.y,
			dx: dx,
			dy: dy
		});
	event_q.insert(move_event);
	return new_position;
}

fn look(position: &Position, map: &mut Map) -> String
{
	let location = map.fetch(position.x,position.y);
	let description = location.description();
	map.replace(location);
	return description;
}

fn load_character(map_obj: Arc<Mutex<Map> >) -> Position
{
	let character = Mobile::new_character("Lord Jim".to_string());
	let position = Position { x: 0, y: 0, uuid: character.get_id() };
	let mut map = map_obj.lock().unwrap();
	let mut location = map.fetch(position.x,position.y);
	location.add_mobile(character);
	map.replace(location);
	return position;
}

// Get input from the user and dispatch commands
fn handle_connection(mut stream: TcpStream, map_obj: Arc<Mutex<Map> >, global_msg_obj: Arc<Mutex<MessageList> >)
{
	let mut has_input = true;
	let mut print_prompt = true;
	let mut now = SystemTime::now();
	let mut last_msg_read_time = SystemTime::now();
	let mut last_msg_read = String::new();
	let mut event_q = EventList::new();
	let mut position = load_character(map_obj.clone());
	let mut input: Vec<u8> = vec![];
	let mut pause_for_tick = true;
	let _ = stream.set_read_timeout(Some(Duration::from_millis(SUB_TICK.into())));
	stream.write_all(b"Welcome!").unwrap();
	loop
	{
		if print_prompt
		{
			stream.write_all(b"\n>> ").unwrap();
			stream.flush().unwrap();
		}
		print_prompt = false;
		input.clear();
		loop
		{
			has_input = false;
			let mut buf = vec![0;128];
			let n = match stream.read(&mut buf)
			{
				Err(e) =>
					{
						match e.kind()
						{
							ErrorKind::WouldBlock => { break; },
							_ => { return; }
						}
					}
				Ok(m) => { if m == 0 { return; } m }
			};
			buf.truncate(n);
			input.extend_from_slice(&buf);
			let end = input.last();
			match end
			{
				Some(end) => if *end == b'\n' { has_input = true; break; },
				_ => { continue; }
			}
		}
		// Process commands
		if has_input
		{
			let input_string = String::from_utf8_lossy(&input);
			let clean_input_string = input_string.trim();
			let mut map = map_obj.lock().unwrap();
			print_prompt = true;
			match clean_input_string.as_ref()
			{
				"e" =>
					{
						position = goto(position,1,0,&mut event_q);
						pause_for_tick = true;
					},
				"w" =>
					{
						position = goto(position,-1,0,&mut event_q);
						pause_for_tick = true;
					},
				"n" =>
					{
						position = goto(position,0,1,&mut event_q);
						pause_for_tick = true;
					},
				"s" =>
					{
						position = goto(position,0,-1,&mut event_q);
						pause_for_tick = true;
					},
				"look" =>
					{
						stream.write_all(look(&position,&mut *map).as_bytes()).unwrap();
						pause_for_tick = false;
					},
				_ =>
					{
						stream.write_all(b"What?").unwrap();
						pause_for_tick = false;
					}
			}
		}
		// Run a tick
		loop
		{
			match now.elapsed()
			{
				Ok(elapsed) =>
					{
						if elapsed.as_millis() > TICK.into()
						{
							let mut map = map_obj.lock().unwrap();
							let mut msg_list = global_msg_obj.lock().unwrap();
							event_q.tick(&mut *map, &mut *msg_list);
							now = SystemTime::now();
							break;
						}
						else if !pause_for_tick
						{
							break;
						}
					} 
				_ => { now = SystemTime::now(); break; }
			}
		}
		// Display any messages in the global message list
		{
			let mut msg_list = global_msg_obj.lock().unwrap();
			last_msg_read = msg_list.read(position.x,position.y,position.uuid,last_msg_read_time);
			last_msg_read_time = SystemTime::now();
		}
		if !last_msg_read.is_empty()
		{
			print_prompt = true;
			stream.write_all(b"\n").unwrap();
			stream.write_all(last_msg_read.as_bytes()).unwrap();
			stream.flush().unwrap();
		}
	}
}

// Automatic events that run in the background
fn background_events(map_obj: Arc<Mutex<Map> >, global_msg_obj: Arc<Mutex<MessageList> >)
{
	let tick_duration = Duration::from_millis(TICK.into());
	let mut event_q = EventList::new();
	// Default events
	let wandering_monsters = Box::new(WanderingMonsterEvent::new());
	event_q.insert(wandering_monsters);
	// Run the event loop
	loop
	{
		thread::sleep(tick_duration);
		{
			let mut map = map_obj.lock().unwrap();
			let mut msg_list = global_msg_obj.lock().unwrap();
			event_q.tick(&mut *map, &mut *msg_list);
		}
	}
}

fn main()
{
	let global_msg_obj = Arc::new(Mutex::new(MessageList::new()));
	let map_obj = Arc::new(Mutex::new(Map::new()));
	// Start the background thread
	{
		let map_obj = Arc::clone(&map_obj);
		let global_msg_obj = Arc::clone(&global_msg_obj);
		thread::spawn(move ||
			{
				background_events(map_obj,global_msg_obj);
			}
		);
	}
	// Accept connections
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	for stream in listener.incoming()
	{
		let stream = stream.unwrap();
		let map_obj = Arc::clone(&map_obj);
		let global_msg_obj = Arc::clone(&global_msg_obj);
		thread::spawn(move ||
			{
				handle_connection(stream,map_obj,global_msg_obj);
			}
		);
	}
}
