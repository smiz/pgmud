use std::{
	io::{prelude::*,ErrorKind},
	net::{TcpListener,TcpStream},
	thread,
	sync::{Arc,Mutex},
	time::{Duration,SystemTime},
	collections::LinkedList
};
use location::{Location, LocationTypeCode};
use uuid::Uuid;

mod items;
mod world;
mod location;
mod map;
mod mobile;
mod object;
mod events;
mod message;
mod dice;
mod uid;
use crate::world::*;
use crate::object::*;
use crate::events::*;
use crate::mobile::*;

// Tick in milliseconds
const TICK : u16 = 250;
// Sub ticks for interval between checking
// messages, getting input, and so forth
const SUB_TICK : u16 = 50;

// If we are removed, then exit. Otherwise return our position
fn check_if_removed(uuid: Uuid, world: &mut WorldState, stream: &mut TcpStream, last_msg_read_time: SystemTime) -> bool
{
	if world.mobile_active(uuid)
	{
		return false;
	}
	else
	{
		let last_msgs = world.message_list.read_targetted(uuid,last_msg_read_time);
		stream.write_all(b"\n").unwrap();
		stream.write_all(last_msgs.as_bytes()).unwrap();
		stream.flush().unwrap();
		stream.write_all(b"Goodbye!\n").unwrap();
		stream.flush().unwrap();
		return true;
	}
}

fn get_item(uuid: Uuid, world: &mut WorldState, target: &String) -> String
{
	let mut result = "Got it!".to_string();
	let position = world.find_mobile_location(uuid).unwrap();
	let mut mobile = world.fetch_mobile(uuid).unwrap();
	let item = world.fetch_item_by_name(position.0,position.1,target);
	if item.is_some()
	{
		let item = item.unwrap();
		if mobile.has_room_for_item(&item)
		{
			mobile.add_item(item,true);
		}
		else
		{
			result = "You don't have space for that!".to_string();
			world.add_item(position.0,position.1,item);
		}
	}
	else
	{
		result = "Get what?".to_string();
	}
	world.add_mobile(mobile,position.0,position.1);
	return result;
}

fn make_item(uuid: Uuid, event_q: &mut EventList, target: &String) -> String
{
	match target.as_ref()
	{
		"rawhide" =>
			{
				event_q.insert(Box::new(MakeRawhideEvent { maker: uuid }));
				return "You begin making rawhide".to_string();
			}
		"leatherarmor" =>
			{
				event_q.insert(Box::new(MakeLeatherArmorEvent { maker: uuid }));
				return "You begin making leather armor".to_string();
			}
		_ => { return "What is ".to_string()+target+&"?".to_string(); }
	}
}

fn practice(uuid: Uuid, world: &mut WorldState, skill: &String) -> String
{
	let mut found_skill = true;
	let mut success = false;
	let position = world.find_mobile_location(uuid).unwrap();
	let mut mobile = world.fetch_mobile(uuid).unwrap();
	if skill == "combat"
	{
		success = mobile.practice_combat();
	}
	else if skill == "steal"
	{
		success = mobile.practice_steal();
	}
	else if skill == "perception"
	{
		success = mobile.practice_perception();
	}
	else if skill == "knowledge"
	{
		success = mobile.practice_knowledge();
	}
	else if skill == "leatherwork"
	{
		success = mobile.practice_leatherwork();
	}
	else
	{
		found_skill = false; 
	}
	world.add_mobile(mobile,position.0,position.1);
	if !found_skill
	{
		return "Practice what?".to_string();
	}
	else if !success
	{
		return "Not enough xp!".to_string();
	}
	return "You have improved at ".to_string()+&skill+"!";
}

fn drop_item(uuid: Uuid, world: &mut WorldState, target: &String) -> String
{
	let mut result = "Dropped it!".to_string();
	let position = world.find_mobile_location(uuid).unwrap();
	let mut mobile = world.fetch_mobile(uuid).unwrap();
	let item = mobile.fetch_item_by_name(&target);
	if item.is_some()
	{
		let item = item.unwrap();
		if item.xp_in_town_only && world.get_location_type(position.0, position.1) == LocationTypeCode::Town
		{
			result = "A collector eagerly accepts the ".to_string()+&item.name+"!";
			mobile.xp += item.xp_value;
		}
		else
		{
			world.add_item(position.0,position.1,item);
		}
	}
	else
	{
		result = "Drop what?".to_string();
	}
	world.add_mobile(mobile,position.0,position.1);
	return result;
}

fn eat_item(uuid: Uuid, world: &mut WorldState, target: &String) -> String
{
	let position = world.find_mobile_location(uuid).unwrap();
	let mut mobile = world.fetch_mobile(uuid).unwrap();
	let result = mobile.eat_item_by_name(target);
	world.add_mobile(mobile,position.0,position.1);
	return result;
}

fn kill(uuid: Uuid, world: &mut WorldState, event_q: &mut EventList, target: &String)
{
	let position = world.find_mobile_location(uuid).unwrap();
	let defender = world.get_mobile_id_by_name(position.0,position.1,&target);
	match defender
	{
		Some(defender) => { event_q.insert(Box::new(CombatEvent { attacker: uuid, defender: defender })); },
		_ => { return; }
	}
}

fn steal(uuid: Uuid, world: &mut WorldState, event_q: &mut EventList, target: &String)
{
	let position = world.find_mobile_location(uuid).unwrap();
	let mark = world.get_mobile_id_by_name(position.0,position.1,&target);
	match mark
	{
		Some(mark) => { event_q.insert(Box::new(StealEvent { thief: uuid, mark: mark })); },
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

fn look_at(uuid: Uuid, world: &mut WorldState, target: &String) -> String
{
	let position = world.find_mobile_location(uuid).unwrap();
	let mobile = world.fetch_mobile_by_name(position.0,position.1,&target);
	if mobile.is_some()
	{
		let mobile = mobile.unwrap();
		let description = mobile.description()+"\nCarrying:\n"+&mobile.list_inventory();
		world.add_mobile(mobile,position.0,position.1);
		return description;
	}
	let item = world.fetch_item_by_name(position.0,position.1,&target);
	if item.is_some()
	{
		let item = item.unwrap();
		let description = item.description();
		world.add_item(position.0,position.1,item);
		return description;
	}
	return "Look at what?".to_string();	
}

fn look(uuid: Uuid, world: &mut WorldState) -> String
{
	let position = world.find_mobile_location(uuid).unwrap();
	return world.get_location_description(position.0,position.1);
}

fn show_inventory(uuid: Uuid, world: &mut WorldState) -> String
{
	let position = world.find_mobile_location(uuid).unwrap();
	let mobile = world.fetch_mobile(uuid).unwrap();
	let inventory = mobile.list_inventory();
	world.add_mobile(mobile,position.0,position.1);
	return ("You have:\n".to_owned()+&inventory).to_string();
}

fn show_stats_of(uuid: Uuid, world: &mut WorldState, target: &String) -> String
{
	let position = world.find_mobile_location(uuid).unwrap();
	let scholar = world.fetch_mobile(uuid).unwrap();
	// Is this a mobile at our location?
	let mobile = world.fetch_mobile_by_name(position.0,position.1,&target);
	if mobile.is_some()
	{
		let mobile = mobile.unwrap();
		let success = scholar.roll_knowledge() > mobile.frequency;
		let description =
			if !success { "Perhaps you should study harder?".to_string() } 
			else { mobile.complete_description() };
		world.add_mobile(mobile,position.0,position.1);
		world.add_mobile(scholar,position.0,position.1);
		return description;
	}
	// Is this an item at our location?
	let item = world.fetch_item_by_name(position.0,position.1,&target);
	if item.is_some()
	{
		let item = item.unwrap();
		let success = scholar.roll_knowledge() > item.frequency;
		let description =
			if !success { "Perhaps you should study harder?".to_string() } 
			else { item.complete_description() };
		world.add_item(position.0,position.1,item);
		world.add_mobile(scholar,position.0,position.1);
		return description;
	}
	// Nope. Nothing to stat.
	world.add_mobile(scholar,position.0,position.1);
	return "Stat what?".to_string();	
}

fn show_stats(uuid: Uuid, world: &mut WorldState) -> String
{
	let position = world.find_mobile_location(uuid).unwrap();
	let mobile = world.fetch_mobile(uuid).unwrap();
	let result = mobile.complete_description();
	world.add_mobile(mobile,position.0,position.1);
	return result;
}

fn load_character(world_obj: Arc<Mutex<WorldState> >, mut stream: &TcpStream) -> Option<Uuid>
{
	let mut result = None;
	stream.write_all(b"Welcome!\n").unwrap();
	stream.flush().unwrap();
	stream.write_all(b"What is your name? ").unwrap();
	stream.flush().unwrap();
	let mut buf = vec![0;128];
	let n = match stream.read(&mut buf)
	{
		Err(_e) => { return None; },
		Ok(m) => { m }
	};
	if n == 0 { return None; }
	buf.truncate(n);
	let line = String::from_utf8_lossy(&buf);
	let name = line.trim();
	loop
	{
		let mut character = Mobile::new_character(&name.to_string());
		{
			let mut world = world_obj.lock().unwrap();
			if character.load_from_file()
			{
				let uuid = character.get_id();
				if !world.mobile_exists(uuid)
				{
					world.add_mobile(character,0,0);
				}
				result = Some(uuid);
				break;
			}
		}
		stream.write_all(character.complete_description().as_bytes()).unwrap();
		stream.write_all(b"Keep this character (y/n)? ").unwrap();
		stream.flush().unwrap();
		let mut buf = vec![0;128];
		let n = match stream.read(&mut buf)
		{
			Err(_e) => { break; },
			Ok(m) => { m }
		};
		if n == 0 { break; }
		buf.truncate(n);
		let line = String::from_utf8_lossy(&buf);
		let clean_line = line.trim();
		if clean_line.contains(&"y")
		{
			let id = character.get_id();
			let mut world = world_obj.lock().unwrap();
			world.add_mobile(character,0,0);
			result = Some(id);
			break;
		}
	}
	return result;
}

fn process_command(command: &mut LinkedList<String>, uuid: Uuid, world: &mut WorldState, event_q: &mut EventList) -> String
{
	if command.is_empty()
	{
		return "What?".to_string();
	}
	let action = command.pop_front();
	match action.unwrap().as_ref()
	{
		"e" => { goto(uuid,1,0,event_q); return String::new(); }
		"w" => { goto(uuid,-1,0,event_q); return String::new(); }
		"n" => { goto(uuid,0,1,event_q); return String::new(); } 
		"s" => { goto(uuid,0,-1,event_q); return String::new(); }
		"eat" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { return eat_item(uuid,world,target); },
					None => { return "Eat what?".to_string(); }
				}
			},
		"look" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { return look_at(uuid,world,target); },
					None => { return look(uuid,world); }
				}
			},
		"kill" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { kill(uuid,world,event_q,&target); return String::new(); },
					None => { return "Kill what?".to_string(); }
				}
			},
		"steal" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { steal(uuid,world,event_q,target); return String::new(); },
					None => { return "Steal from whom?".to_string(); }
				}
			},
		"get" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { return get_item(uuid,world,target); },
					None => { return "Get what?".to_string(); }
				}
			},
		"drop" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { return drop_item(uuid,world,target); },
					None => { return "Drop what?".to_string(); }
				}
			},
		"prac" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { return practice(uuid,world,target); },
					None => { return "Practice what?".to_string(); }
				}
			},
		"quit" =>
			{
				world.stash_mobile(uuid);
				return "Goodbye!".to_string();
			},
		"stat" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { return show_stats_of(uuid,world,target); },
					None => { return show_stats(uuid,world); }
				}
			},
		"i" => { return show_inventory(uuid,world); },
		"make" =>
			{
				let target = command.pop_front();
				match target.as_ref()
				{
					Some(target) => { return make_item(uuid,event_q,target); },
					None => { return "Make what?".to_string(); }
				}
			},
		_ =>
			{
				return "What?".to_string();
			}
	}
}

// Get input from the user and dispatch commands
fn handle_connection(mut stream: TcpStream, world_obj: Arc<Mutex<WorldState> >, event_q_obj: Arc<Mutex<EventList> >)
{
	let mut command : LinkedList<String> = LinkedList::new();
	let mut print_prompt = true;
	let mut _now = SystemTime::now();
	let mut last_message_list_read_time = SystemTime::now();
	let mut message_for_user = String::new();
	let uuid = 
		match load_character(world_obj.clone(),&stream)
		{
			Some(uuid) => { uuid },
			None => { return; }
		};
	let mut input: Vec<u8> = vec![];
	let _ = stream.set_read_timeout(Some(Duration::from_millis(SUB_TICK.into())));
	let mut last_output_char = '\n';
	loop
	{
		let mut has_input = false;
		if print_prompt
		{
			if last_output_char != '\n'
			{
				stream.write_all(b"\n").unwrap();
			}
			stream.write_all(b">> ").unwrap();
			stream.flush().unwrap();
			last_output_char = ' ';
		}
		print_prompt = false;
		let mut buf = vec![0;128];
		let n = match stream.read(&mut buf)
		{
			Err(e) =>
				{
					match e.kind()
					{
						ErrorKind::WouldBlock => { 0 },
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
			Some(end) => if *end == b'\n' { has_input = true; },
			_ => { () }
		}
		{
			// Lock the world
			let mut world = world_obj.lock().unwrap();
			// Got the lock, make sure we are alive before processing a command
			if check_if_removed(uuid,&mut world,&mut stream,last_message_list_read_time) { return; }
			// Process commands
			if has_input
			{
				// Got the input
				has_input = false;
				print_prompt = true;
				last_output_char = '\n';
				{
					// Process the input
					let input_string = String::from_utf8_lossy(&input);
					let clean_input_string = input_string.trim();
					let tokens = clean_input_string.split_whitespace();
					for word in tokens
					{
						command.push_back(word.to_string());
					}
					let mut event_q = event_q_obj.lock().unwrap();
					message_for_user = process_command(&mut command, uuid, &mut world, &mut event_q);
					}
				command.clear();
				input.clear();
			}
			// Display any messages in the global message list
			// Got the lock, make sure we are alive
			if check_if_removed(uuid,&mut world,&mut stream,last_message_list_read_time) { return; }
			let position = world.find_mobile_location(uuid).unwrap();
			let mobile = world.fetch_mobile(uuid).unwrap();
			mobile.save_to_file();
			world.add_mobile(mobile,position.0,position.1);	
			message_for_user += &world.message_list.read(position.0,position.1,uuid,last_message_list_read_time);
			last_message_list_read_time = SystemTime::now();
		}
		if !message_for_user.is_empty()
		{
			print_prompt = true;
			if last_output_char != '\n'
			{
				stream.write_all(b"\n").unwrap();
			}
			last_output_char = message_for_user.chars().last().unwrap();
			stream.write_all(message_for_user.as_bytes()).unwrap();
			stream.flush().unwrap();
			message_for_user.clear();
		}
	}
}

// Automatic events that run in the background
fn background_events(world_obj: Arc<Mutex<WorldState> >, event_q_obj: Arc<Mutex<EventList> >)
{
	let tick_duration = Duration::from_millis(TICK.into());
	{
		let mut event_q = event_q_obj.lock().unwrap();
		// Default events
		let wandering_monsters = Box::new(WanderingMonsterEvent::new());
		event_q.insert(wandering_monsters);
		let age_event = Box::new(AgeEvent::new());
		event_q.insert(age_event);
	}
	// Run the event loop
	loop
	{
		thread::sleep(tick_duration);
		{
			let mut event_q = event_q_obj.lock().unwrap();
			let mut world = world_obj.lock().unwrap();
			event_q.tick(&mut *world);
		}
	}
}

fn main()
{
	let world_obj = Arc::new(Mutex::new(WorldState::new()));
	let event_q_obj = Arc::new(Mutex::new(EventList::new()));
	// Start the background thread
	{
		let world_obj = Arc::clone(&world_obj);
		let event_q_obj = Arc::clone(&event_q_obj);
		thread::spawn(||
			{
				background_events(world_obj,event_q_obj);
			}
		);
	}
	// Accept connections
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	for stream in listener.incoming()
	{
		let stream = stream.unwrap();
		let world_obj = Arc::clone(&world_obj);
		let event_q_obj = Arc::clone(&event_q_obj);
		thread::spawn(||
			{
				handle_connection(stream,world_obj,event_q_obj);
			}
		);
	}
}
