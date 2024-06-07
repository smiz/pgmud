use crate::world::WorldState;
use std::collections::LinkedList;
use crate::map::LocationVisitor;
use crate::location::Location;
use crate::location::LocationTypeCode;
use crate::mobile::Mobile;
use crate::object::Object;
use rand::random;
use uuid::Uuid;

pub trait Event
{
	// Execute the event.
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList);
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

	pub fn tick(&mut self, world: &mut WorldState)
	{
		if self.is_odd
		{
			self.is_odd = false;
			while !self.odd_event_queue.is_empty()
			{
				let event = self.odd_event_queue.pop_front().unwrap();
				event.tick(world,self)
			}
		}
		else
		{
			self.is_odd = true;
			while !self.even_event_queue.is_empty()
			{
				let event = self.even_event_queue.pop_front().unwrap();
				event.tick(world,self)
			}
		}
	}
}

// Combat between a pair of mobiles
pub struct CombatEvent
{
	// Combatants
	pub attacker: Uuid,
	pub defender: Uuid,
}

impl Event for CombatEvent
{
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
		let a_location = world.find_mobile_location(self.attacker);
		let b_location = world.find_mobile_location(self.defender);
		if !a_location.is_some()
		{
			return;
		}
		if !b_location.is_some()
		{
			let a_position = a_location.unwrap();
			world.message_list.post_for_target("Kill who?".to_string(),a_position.0,a_position.1,self.attacker);
			return;
		}
		let a_position = a_location.unwrap();
		let b_position = b_location.unwrap();
		if a_position != b_position
		{
			world.message_list.post_for_target("Kill who?".to_string(),a_position.0,a_position.1,self.attacker);
			return;
		}
		let a = world.fetch_mobile(self.attacker);
		let b = world.fetch_mobile(self.defender);
		if a.is_some() && b.is_some()
		{
			let mut a = a.unwrap();
			let mut b = b.unwrap();
			let a_has_actions = a.use_action();
			let b_has_actions = b.use_action();
			if a_has_actions || b_has_actions
			{
				let a_modifier = a.strength+a.dexterity;
				let b_modifier = b.strength+b.dexterity;
				let outcome = Mobile::contest(a_modifier,b_modifier,&mut a.combat,&mut b.combat);
				if outcome && a_has_actions
				{
					b.damage += a.damage_dice.roll();
					if b.damage > b.max_hit_points()
					{
						world.message_list.broadcast(a.name.clone()+" slays "+&b.name+"!",a_position.0,a_position.1);	
						world.add_mobile(a,a_position.0,a_position.1);
					}
					else
					{
						world.message_list.broadcast(a.name.clone()+" wounds "+&b.name+"!",a_position.0,a_position.1);	
						world.add_mobile(a,a_position.0,a_position.1);
						world.add_mobile(b,b_position.0,b_position.1);
						event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
					}
				}
				else if !outcome && b_has_actions
				{
					a.damage += b.damage_dice.roll();
					if a.damage > a.max_hit_points()
					{
						world.message_list.broadcast(b.name.clone()+" slays "+&a.name+"!",a_position.0,a_position.1);	
						world.add_mobile(b,b_position.0,b_position.1);
					}
					else
					{
						world.message_list.broadcast(b.name.clone()+" wounds "+&a.name+"!",a_position.0,a_position.1);	
						world.add_mobile(a,a_position.0,a_position.1);
						world.add_mobile(b,b_position.0,b_position.1);
						event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
					}
				}
				else if outcome
				{
					world.message_list.broadcast(b.name.clone()+" repulses "+&a.name+"!",a_position.0,a_position.1);	
					world.add_mobile(a,a_position.0,a_position.1);
					world.add_mobile(b,b_position.0,b_position.1);
					event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
				}
				else
				{
					world.message_list.broadcast(a.name.clone()+" repulses "+&b.name+"!",a_position.0,a_position.1);	
					world.add_mobile(a,a_position.0,a_position.1);
					world.add_mobile(b,b_position.0,b_position.1);
					event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
				}
			}
			else
			{
				world.add_mobile(a,a_position.0,a_position.1);
				world.add_mobile(b,b_position.0,b_position.1);
				event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
			}
		}
		else
		{
			if a.is_some() { world.add_mobile(a.unwrap(),a_position.0,a_position.1); }
			if b.is_some() { world.add_mobile(b.unwrap(),b_position.0,b_position.1); }
		}
	}
}

// Move a mobile
pub struct MoveMobileEvent
{
	pub uuid: Uuid,
	pub dx: i16,
	pub dy: i16
}

impl Event for MoveMobileEvent
{
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
		let coordinate = world.find_mobile_location(self.uuid);
		if !coordinate.is_some()
		{
			return;
		}
		let mobile = world.fetch_mobile(self.uuid);
		match mobile
		{
			None => { return; }
			Some(mobile) => 
				{
					let xy = coordinate.unwrap();
					let id = mobile.get_id();
					let arrive_prefix = mobile.arrive_prefix.clone();
					let leave_prefix = mobile.leave_prefix.clone();
					world.add_mobile(mobile,xy.0+self.dx,xy.1+self.dy);
					let location_description = world.get_location_description(xy.0+self.dx,xy.1+self.dy);
					match (self.dx,self.dy)
					{
						(0,1) =>
						{
							world.message_list.post_no_echo(arrive_prefix+" from the south.",xy.0+self.dx,xy.1+self.dy,id);
							world.message_list.post_no_echo(leave_prefix+" to the north.",xy.0,xy.1,id);
							world.message_list.post_for_target(location_description,xy.0+self.dx,xy.1+self.dy,id);
						}
						(0,-1) =>
						{
							world.message_list.post_no_echo(arrive_prefix+" from the north.",xy.0+self.dx,xy.1+self.dy,id);
							world.message_list.post_no_echo(leave_prefix+" to the south.",xy.0,xy.1,id);
							world.message_list.post_for_target(location_description,xy.0+self.dx,xy.1+self.dy,id);
						}
						(1,0) =>
						{
							world.message_list.post_no_echo(arrive_prefix+" from the west.",xy.0+self.dx,xy.1+self.dy,id);
							world.message_list.post_no_echo(leave_prefix+" to the east.",xy.0,xy.1,id);
							world.message_list.post_for_target(location_description,xy.0+self.dx,xy.1+self.dy,id);
						}
						(-1,0) =>
						{
							world.message_list.post_no_echo(arrive_prefix+" from the east.",xy.0+self.dx,xy.1+self.dy,id);
							world.message_list.post_no_echo(leave_prefix+" to the west.",xy.0,xy.1,id);
							world.message_list.post_for_target(location_description,xy.0+self.dx,xy.1+self.dy,id);
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
	fn visit_location(&mut self, location: &mut Box<Location>)
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
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
		let mut visitor = WanderingMonsterLocationVisitor { monster_list: LinkedList::new() };
		world.visit_all_locations(&mut visitor);
		for item in visitor.monster_list
		{
			let name = item.2.get_name();
			let msg = "A ".to_owned()+&name+" has arrived.";
			world.message_list.broadcast(msg,item.0,item.1);
			world.add_mobile(item.2,item.0,item.1);
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

// Age, rest, etc.
pub struct AgeLocationVisitor
{
}

impl LocationVisitor for AgeLocationVisitor
{
	fn visit_location(&mut self, location: &mut Box<Location>)
	{
		location.age_all_mobiles();
	}
}

pub struct AgeEvent
{
}

impl Event for AgeEvent
{
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
		let mut visitor = AgeLocationVisitor { };
		world.visit_all_locations(&mut visitor);
		let next_event = Box::new(AgeEvent::new());
		event_q.insert(next_event);
	}
}

impl AgeEvent
{
	pub fn new() -> AgeEvent
	{
		return AgeEvent {};
	}
}

