use crate::map::Map;
use crate::message::MessageList;
use std::collections::LinkedList;
use crate::map::LocationVisitor;
use crate::location::Location;
use crate::location::LocationTypeCode;
use crate::mobile::Mobile;
use crate::object::Object;
use rand::random;
use uuid::Uuid;
use crate::dice::*;

pub trait Event
{
	// Execute the event.
	fn tick(&self, map: &mut Map, event_q: &mut EventList, msg_list: &mut MessageList);
}

pub struct EventList
{
	even_event_queue: LinkedList<Box<dyn Event> >,
	odd_event_queue: LinkedList<Box<dyn Event> >,
	is_odd: bool
}

impl EventList
{
	pub fn new() -> EventList
	{
		return EventList
				{
					even_event_queue: LinkedList::new(),
					odd_event_queue: LinkedList::new(),
					is_odd : false
				};
	}

	pub fn insert(&mut self, event: Box<dyn Event>)
	{
		if self.is_odd
		{
			self.odd_event_queue.push_back(event);	
		}
		else
		{
			self.even_event_queue.push_back(event);	
		}
	}

	pub fn tick(&mut self, map: &mut Map, msg_list: &mut MessageList)
	{
		if self.is_odd
		{
			self.is_odd = false;
			while !self.odd_event_queue.is_empty()
			{
				let event = self.odd_event_queue.pop_front().unwrap();
				event.tick(map,self,msg_list)
			}
		}
		else
		{
			self.is_odd = true;
			while !self.even_event_queue.is_empty()
			{
				let event = self.even_event_queue.pop_front().unwrap();
				event.tick(map,self,msg_list)
			}
		}
	}
}

// Combat between a pair of mobiles
pub struct CombatEvent
{
	// Combatants
	pub A: Uuid,
	pub B: Uuid,
	// Location
	pub x: i16,
	pub y: i16,
}

impl Event for CombatEvent
{
	fn tick(&self, map: &mut Map, _: &mut EventList, msg_list: &mut MessageList)
	{
		let mut location = map.fetch(self.x,self.y);
		let a = location.fetch_mobile_by_guid(self.A);
		let b = location.fetch_mobile_by_guid(self.B);
		if a.is_some() && b.is_some()
		{
			let mut a = a.unwrap();
			let mut b = b.unwrap();
			let a_has_actions = a.actions_used < a.actions_per_tick;
			let b_has_actions = b.actions_used < b.actions_per_tick;
			a.actions_used += 1;
			b.actions_used += 1;
			if a_has_actions || b_has_actions
			{
				let a_modifier = a.strength+b.dexterity;
				let b_modifier = b.strength+a.dexterity;
				let outcome = Mobile::contest(a_modifier,b_modifier,&mut a.combat,&mut b.combat);
				if outcome && a_has_actions
				{
					a.damage += a.damage_dice.roll();
					if b.damage > b.max_hit_points()
					{
						msg_list.broadcast(a.name.clone()+" slays "+&b.name+"!",self.x,self.y);	
						location.add_mobile(a);
					}
					else
					{
						msg_list.broadcast(a.name.clone()+" wounds "+&b.name+"!",self.x,self.y);	
						location.add_mobile(a);
						location.add_mobile(b);
					}
				}
				else if !outcome && b_has_actions
				{
					a.damage += b.damage_dice.roll();
					if a.damage > a.max_hit_points()
					{
						msg_list.broadcast(b.name.clone()+" slays "+&a.name+"!",self.x,self.y);	
						location.add_mobile(b);
					}
					else
					{
						msg_list.broadcast(b.name.clone()+" wounds "+&a.name+"!",self.x,self.y);	
						location.add_mobile(a);
						location.add_mobile(b);
					}
				}
				else if outcome
				{
					msg_list.broadcast(b.name.clone()+" repulses "+&a.name+"!",self.x,self.y);	
					location.add_mobile(a);
					location.add_mobile(b);
				}
				else
				{
					msg_list.broadcast(a.name.clone()+" repulses "+&a.name+"!",self.x,self.y);	
					location.add_mobile(a);
					location.add_mobile(b);
				}
			}
		}
		map.replace(location);
	}
}

// Move a mobile
pub struct MoveMobileEvent
{
	pub uuid: Uuid,
	pub x: i16,
	pub y: i16,
	pub dx: i16,
	pub dy: i16
}

impl Event for MoveMobileEvent
{
	fn tick(&self, map: &mut Map, _: &mut EventList, msg_list: &mut MessageList)
	{
		let mut location = map.fetch(self.x,self.y);
		let mobile = location.fetch_mobile_by_guid(self.uuid);
		map.replace(location);
		match mobile
		{
			None => { return; }
			Some(mobile) => 
				{
					let id = mobile.get_id();
					let arrive_prefix = mobile.arrive_prefix.clone();
					let leave_prefix = mobile.leave_prefix.clone();
					let mut location = map.fetch(self.x+self.dx,self.y+self.dy);
					location.add_mobile(mobile);
					let location_description = location.description();
					map.replace(location);
					match (self.dx,self.dy)
					{
						(0,1) =>
						{
							msg_list.post_no_echo(arrive_prefix+" from the south.",self.x+self.dx,self.y+self.dy,id);
							msg_list.post_no_echo(leave_prefix+" to the north.",self.x,self.y,id);
							msg_list.post_for_target(location_description,self.x+self.dx,self.y+self.dy,id);
						}
						(0,-1) =>
						{
							msg_list.post_no_echo(arrive_prefix+" from the north.",self.x+self.dx,self.y+self.dy,id);
							msg_list.post_no_echo(leave_prefix+" to the south.",self.x,self.y,id);
							msg_list.post_for_target(location_description,self.x+self.dx,self.y+self.dy,id);
						}
						(1,0) =>
						{
							msg_list.post_no_echo(arrive_prefix+" from the west.",self.x+self.dx,self.y+self.dy,id);
							msg_list.post_no_echo(leave_prefix+" to the east.",self.x,self.y,id);
							msg_list.post_for_target(location_description,self.x+self.dx,self.y+self.dy,id);
						}
						(-1,0) =>
						{
							msg_list.post_no_echo(arrive_prefix+" from the east.",self.x+self.dx,self.y+self.dy,id);
							msg_list.post_no_echo(leave_prefix+" to the west.",self.x,self.y,id);
							msg_list.post_for_target(location_description,self.x+self.dx,self.y+self.dy,id);
						}
						_ => { return; }
				}
			}
		}
	}
}

// Create wandering monsters

pub struct WanderingMonsterLocationVisitor
{
	pub monster_list: LinkedList<(i16,i16,Box<Mobile>)>,
}

impl LocationVisitor for WanderingMonsterLocationVisitor
{
	fn visit_location(&mut self, location: & Box<Location>)
	{
		match location.location_type
		{
			LocationTypeCode::Forest => 
				{
					let monster = self.forest_wandering_monster();
					match monster
					{
						Some(monster) => { self.monster_list.push_back((location.x,location.y,monster)); },
						_ => { return; }
					}
				}
			LocationTypeCode::Town => 
				{
					let monster = self.town_wandering_monster();
					match monster
					{
						Some(monster) => { self.monster_list.push_back((location.x,location.y,monster)); },
						_ => { return; }
					}
				}
		}
	}
}

impl WanderingMonsterLocationVisitor
{
	fn forest_wandering_monster(&self) -> Option<Box<Mobile> >
	{
		let pick = random::<u8>();
		match pick
		{
			0 => { return Some(Mobile::rodent()); },
			1 => { return Some(Mobile::rodent()); },
			2 => { return Some(Mobile::rodent()); },
			3 => { return Some(Mobile::rodent()); },
			4 => { return Some(Mobile::rodent()); },
			5 => { return Some(Mobile::rodent()); },
			_ => { return None; }
		}
	}

	fn town_wandering_monster(&self) -> Option<Box<Mobile> >
	{
		let pick = random::<u8>();
		match pick
		{
			0 => { return Some(Mobile::beggar()); },
			1 => { return Some(Mobile::beggar()); },
			2 => { return Some(Mobile::beggar()); },
			3 => { return Some(Mobile::beggar()); },
			4 => { return Some(Mobile::beggar()); },
			5 => { return Some(Mobile::beggar()); },
			_ => { return None; }
		}
	}
}

pub struct WanderingMonsterEvent
{
}

impl Event for WanderingMonsterEvent
{
	fn tick(&self, map: &mut Map, event_q: &mut EventList, msg_list: &mut MessageList)
	{
		let mut visitor = WanderingMonsterLocationVisitor { monster_list: LinkedList::new() };
		map.visit_all_locations(&mut visitor);
		for item in visitor.monster_list
		{
			let name = item.2.get_name();
			let msg = "A ".to_owned()+&name+" has arrived.";
			msg_list.broadcast(msg,item.0,item.1);
			let mut location = map.fetch(item.0,item.1);
			location.add_mobile(item.2);
			map.replace(location);
		}
		let next_event = Box::new(WanderingMonsterEvent::new());
		event_q.insert(next_event);
	}
}

impl WanderingMonsterEvent
{
	pub fn new() -> WanderingMonsterEvent
	{
		return WanderingMonsterEvent {};
	}
}
