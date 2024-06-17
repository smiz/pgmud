use crate::world::WorldState;
use std::collections::LinkedList;
use crate::map::LocationVisitor;
use crate::location::Location;
use crate::location::LocationTypeCode;
use crate::mobile::Mobile;
use crate::object::Object;
use crate::message::*;
use crate::dice::*;
use crate::items::*;
use rand::random;
use uuid::Uuid;


pub trait Event
{
	// Execute the event.
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList);
}

pub struct EventList
{
	even_event_queue: LinkedList<Box<dyn Event+Send> >,
	odd_event_queue: LinkedList<Box<dyn Event+Send> >,
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

	pub fn insert(&mut self, event: Box<dyn Event+Send>)
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
			world.message_list.post_for_target("Kill who?".to_string(),self.attacker);
			return;
		}
		let a_position = a_location.unwrap();
		let b_position = b_location.unwrap();
		if a_position != b_position
		{
			world.message_list.post_for_target("Kill who?".to_string(),self.attacker);
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
				let outcome = a.roll_combat() >= b.roll_combat();
				if outcome && a_has_actions
				{
					b.damage += a.damage_dice.roll();
					if b.damage > b.max_hit_points()
					{
						world.message_list.broadcast(a.name_with_article.clone()+" slays "+&b.name_with_article+"!",a_position.0,a_position.1);	
						world.message_list.post_for_target("You have been slain by ".to_string()+&a.name_with_article+"!",b.get_id());
						world.add_mobile(a,a_position.0,a_position.1);
						b.is_killed();
						world.add_corpse(&mut b,b_position.0,b_position.1);
					}
					else
					{
						world.message_list.broadcast(a.name_with_article.clone()+" wounds "+&b.name_with_article+" with a "+&a.wielded+"!",a_position.0,a_position.1);	
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
						world.message_list.broadcast(b.name_with_article.clone()+" slays "+&a.name_with_article+"!",a_position.0,a_position.1);	
						world.message_list.post_for_target("You have been slain by ".to_string()+&b.name_with_article+"!",a.get_id());
						world.add_mobile(b,b_position.0,b_position.1);
						a.is_killed();
						world.add_corpse(&mut a,a_position.0,a_position.1);
					}
					else
					{
						world.message_list.broadcast(b.name_with_article.clone()+" wounds "+&a.name_with_article+" with a "+&b.wielded+"!",a_position.0,a_position.1);	
						world.add_mobile(a,a_position.0,a_position.1);
						world.add_mobile(b,b_position.0,b_position.1);
						event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
					}
				}
				else if outcome
				{
					world.message_list.broadcast(b.name_with_article.clone()+" repulses "+&a.name_with_article+"!",a_position.0,a_position.1);	
					world.add_mobile(a,a_position.0,a_position.1);
					world.add_mobile(b,b_position.0,b_position.1);
					event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
				}
				else
				{
					world.message_list.broadcast(a.name_with_article.clone()+" repulses "+&b.name_with_article+"!",a_position.0,a_position.1);	
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
	fn tick(&self, world: &mut WorldState, _: &mut EventList)
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
							world.message_list.post_for_target(location_description,id);
						}
						(0,-1) =>
						{
							world.message_list.post_no_echo(arrive_prefix+" from the north.",xy.0+self.dx,xy.1+self.dy,id);
							world.message_list.post_no_echo(leave_prefix+" to the south.",xy.0,xy.1,id);
							world.message_list.post_for_target(location_description,id);
						}
						(1,0) =>
						{
							world.message_list.post_no_echo(arrive_prefix+" from the west.",xy.0+self.dx,xy.1+self.dy,id);
							world.message_list.post_no_echo(leave_prefix+" to the east.",xy.0,xy.1,id);
							world.message_list.post_for_target(location_description,id);
						}
						(-1,0) =>
						{
							world.message_list.post_no_echo(arrive_prefix+" from the east.",xy.0+self.dx,xy.1+self.dy,id);
							world.message_list.post_no_echo(leave_prefix+" to the west.",xy.0,xy.1,id);
							world.message_list.post_for_target(location_description,id);
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
	fn visit_location(&mut self, location: &mut Box<Location>, _messages: &mut MessageList)
	{
		if location.num_mobiles() > 3
		{
			return;
		}
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
			_ => { return; }
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
			5 => { return Some(Mobile::rabbit()); },
			6 => { return Some(Mobile::goblin()); },
			7 => { return Some(Mobile::small_woodland_creature()); },
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
			6 => { return Some(Mobile::foppish_dandy()); },
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
	fn visit_location(&mut self, location: &mut Box<Location>, messages: &mut MessageList)
	{
		location.age_all_mobiles();
		location.age_all_items(messages);
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

// One mobile steals from another
pub struct StealEvent
{
	pub thief: Uuid,
	pub mark: Uuid,
}

impl Event for StealEvent
{
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
		let a_location = world.find_mobile_location(self.thief);
		let b_location = world.find_mobile_location(self.mark);
		if !a_location.is_some()
		{
			return;
		}
		if !b_location.is_some()
		{
			world.message_list.post_for_target("Steal from who?".to_string(),self.thief);
			return;
		}
		let a_position = a_location.unwrap();
		let b_position = b_location.unwrap();
		if a_position != b_position
		{
			world.message_list.post_for_target("Steal from who?".to_string(),self.thief);
			return;
		}
		let a = world.fetch_mobile(self.thief);
		let b = world.fetch_mobile(self.mark);
		if a.is_some() && b.is_some()
		{
			let mut a = a.unwrap();
			let mut b = b.unwrap();
			// If we have an action available, then use it attempting to steal
			if a.use_action()
			{
				if a.roll_steal() > b.roll_perception()
				{
					let item = b.fetch_random_item();
					if item.is_some()
					{
						let item = item.unwrap();
						if a.has_room_for_item(&item)
						{
							world.message_list.post_for_target("You stole a ".to_string()+&item.get_name(),a.get_id());
							a.add_item(item,true);
						}
						else
						{
							world.message_list.post_for_target("You don't have room for ".to_string()+&item.get_name()+"!",a.get_id());
							b.add_item(item,false);
						}
					}
					else
					{
						world.message_list.post_for_target("You come up empty handed!".to_string(),a.get_id());
					}
				}
				else
				{
					world.message_list.broadcast(a.name.clone()+" is a thief!",a_position.0,a_position.1);	
					event_q.insert(Box::new(CombatEvent { attacker: self.mark, defender: self.thief }));
				}
			}
			// Otherwise wait until we have an action
			else
			{
				event_q.insert(Box::new(StealEvent { thief: self.thief, mark: self.mark }));
			}
			// Restore the mobiles to the map
			world.add_mobile(a,a_position.0,a_position.1);
			world.add_mobile(b,a_position.0,a_position.1);
			return;
		}
		if a.is_some()
		{
			world.message_list.post_for_target("Steal from who?".to_string(),self.thief);
			world.add_mobile(a.unwrap(),a_position.0,a_position.1);
		}
		if b.is_some()
		{
			world.add_mobile(b.unwrap(),b_position.0,b_position.1);
		}
	}
}

// Make some rawhide from corpses
pub struct MakeRawhideEvent
{
	pub maker: Uuid,
}

impl Event for MakeRawhideEvent
{
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
		let mut successes = 0;
		let die = Dice { number: 1, die: 100 };
		let position = world.find_mobile_location(self.maker);
		if position.is_none()
		{
			return;
		}
		let position = position.unwrap();
		let mut mobile = world.fetch_mobile(self.maker).unwrap();
		// Reschedule if we don't have an action
		if !mobile.use_action()
		{
			event_q.insert(Box::new(MakeRawhideEvent { maker: self.maker }));
		}
		else
		{
			// Find and transform each corpse
			loop
			{
				let corpse = mobile.fetch_item_by_name(&"corpse".to_string());
				if corpse.is_none()
				{
					break;
				}
				if mobile.roll_leatherwork() > 50+die.roll()
				{
					successes += 1;
					mobile.add_item(Item::rawhide(),false);
				}
			}
			world.message_list.broadcast(mobile.name_with_article.clone()+&" makes ".to_string()+&successes.to_string()+
				&" pieces of rawhide!".to_string(),position.0,position.1);	
		}
		world.add_mobile(mobile,position.0,position.1);
	}
}	
