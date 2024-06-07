use std::{
	io::{prelude::*,ErrorKind,stdout},
	net::{TcpListener,TcpStream},
	thread,
	sync::{Arc,Mutex},
	time::{Duration,SystemTime}
};
use uuid::Uuid;

mod world;
mod location;
mod map;
mod mobile;
mod object;
mod events;
mod message;
mod dice;
use crate::world::*;
use crate::object::*;
use crate::events::*;
use crate::mobile::*;

// Tick in milliseconds
const TICK : u16 = 250;
// Sub ticks for interval between checking
// messages, getting input, and so forth
const SUB_TICK : u16 = 50;

// If we are dead, then exit. Otherwise return our position
fn check_if_dead(uuid: Uuid, world: &mut WorldState, stream: &mut TcpStream) -> bool
{
	if world.mobile_exists(uuid)
	{
		return false;
	}
	else
	{
		stream.write_all(b"Goodbye!\n").unwrap();
		stream.flush().unwrap();
		return true;
	}
}

fn kill(uuid: Uuid, world: &mut WorldState, target: String, event_q: &mut EventList)
{
	let position = world.find_mobile_location(uuid).unwrap();
	let defender = world.get_mobile_id_by_name(position.0,position.1,target);
	match defender
	{
		Some(defender) => { event_q.insert(Box::new(CombatEvent { attacker: uuid, defender: defender })); },
		_ => { return; }
	}
}

fn goto(uuid: Uuid, dx: i16, dy: i16, event_q: &mut EventList)
{
	let move_event = Box::new(MoveMobileEvent
		{
			uuid: uuid,
			dx: dx,
			dy: dy
		});
	event_q.insert(move_event);
}

fn look(uuid: Uuid, world: &mut WorldState) -> String
{
	let position = world.find_mobile_location(uuid).unwrap();
	return world.get_location_description(position.0,position.1);
}

fn load_character(world_obj: Arc<Mutex<WorldState> >) -> Uuid
{
	let character = Mobile::new_character("Lord Jim".to_string());
	let uuid = character.get_id();
	let mut world = world_obj.lock().unwrap();
	world.add_mobile(character,0,0);
	return uuid;	
}

// Get input from the user and dispatch commands
fn handle_connection(mut stream: TcpStream, world_obj: Arc<Mutex<WorldState> >)
{
	let mut has_input = true;
	let mut print_prompt = true;
	let mut now = SystemTime::now();
	let mut last_msg_read_time = SystemTime::now();
	let mut last_msg_read = String::new();
	let mut event_q = EventList::new();
	let uuid = load_character(world_obj.clone());
	let mut input: Vec<u8> = vec![];
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
			let mut world = world_obj.lock().unwrap();
			// Got the lock, make sure we are alive before processing a command
			if check_if_dead(uuid,&mut world,&mut stream) { return; }
			// We are alive. Process the command.
			print_prompt = true;
			let mut tokens = clean_input_string.split_whitespace();
			match tokens.next().unwrap().as_ref()
			{
				"e" =>
					{
						goto(uuid,1,0,&mut event_q);
					},
				"w" =>
					{
						goto(uuid,-1,0,&mut event_q);
					},
				"n" =>
					{
						goto(uuid,0,1,&mut event_q);
					},
				"s" =>
					{
						goto(uuid,0,-1,&mut event_q);
					},
				"look" =>
					{
						stream.write_all(look(uuid,&mut *world).as_bytes()).unwrap();
					},
				"kill" =>
					{
						let target = tokens.next();
						match target
						{
							Some(target) => { kill(uuid,&mut* world,target.to_string(),&mut event_q); },
							None => { stream.write_all(b"Kill what?").unwrap(); }
						}
					}
				_ =>
					{
						stream.write_all(b"What?").unwrap();
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
							let mut world = world_obj.lock().unwrap();
							event_q.tick(&mut *world);
							// After the events, make sure we are alive
							if check_if_dead(uuid,&mut world,&mut stream) { return; }
							now = SystemTime::now();
							break;
						}
					} 
				_ => { now = SystemTime::now(); break; }
			}
		}
		// Display any messages in the global message list
		{
			let mut world = world_obj.lock().unwrap();
			// Got the lock, make sure we are alive
			if check_if_dead(uuid,&mut world,&mut stream) { return; }
			let position = world.find_mobile_location(uuid).unwrap();
			last_msg_read = world.message_list.read(position.0,position.1,uuid,last_msg_read_time);
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
fn background_events(world_obj: Arc<Mutex<WorldState> >)
{
	let tick_duration = Duration::from_millis(TICK.into());
	let mut event_q = EventList::new();
	// Default events
	let wandering_monsters = Box::new(WanderingMonsterEvent::new());
	event_q.insert(wandering_monsters);
	let age_event = Box::new(AgeEvent::new());
	event_q.insert(age_event);
	// Run the event loop
	loop
	{
		thread::sleep(tick_duration);
		{
			let mut world = world_obj.lock().unwrap();
			event_q.tick(&mut *world);
		}
	}
}

fn main()
{
	let world_obj = Arc::new(Mutex::new(WorldState::new()));
	// Start the background thread
	{
		let world_obj = Arc::clone(&world_obj);
		thread::spawn(move ||
			{
				background_events(world_obj);
			}
		);
	}
	// Accept connections
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	for stream in listener.incoming()
	{
		let stream = stream.unwrap();
		let world_obj = Arc::clone(&world_obj);
		thread::spawn(move ||
			{
				handle_connection(stream,world_obj);
			}
		);
	}
}
