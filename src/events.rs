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
		let now = std::time::Instant::now();
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
		print!("# events is {}, {} in {} \n",self.odd_event_queue.len(),self.even_event_queue.len(),now.elapsed().as_millis());	
	}
}

// Combat between a pair of mobiles
pub struct CombatEvent
{
	// Combatants
	pub attacker: usize,
	pub defender: usize,
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
				let outcome = a.roll_combat() > b.roll_combat();
				if outcome && a_has_actions
				{
					let damage = b.do_damage(a.damage_dice.roll());
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
						world.message_list.broadcast(a.name_with_article.clone()+" wounds "+&b.name_with_article+" with a "+&a.wielded+
							" for "+&damage.to_string()+"!",a_position.0,a_position.1);	
						world.add_mobile(a,a_position.0,a_position.1);
						world.add_mobile(b,b_position.0,b_position.1);
						event_q.insert(Box::new(CombatEvent { attacker: self.attacker, defender: self.defender }));
					}
				}
				else if !outcome && b_has_actions
				{
					let damage = a.do_damage(b.damage_dice.roll());
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
						world.message_list.broadcast(b.name_with_article.clone()+" wounds "+&a.name_with_article+" with a "+&b.wielded+	
							" for "+&damage.to_string()+"!",a_position.0,a_position.1);	
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
	pub uuid: usize,
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

// An active mobile that may wander, become aggressive, or perform
// other actions on its own
pub struct ActiveMonsterEvent
{
	pub id: usize,
}

impl Event for ActiveMonsterEvent
{
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
		// Find the mobile's position
		let xy = world.find_mobile_location(self.id);
		// If it has no position, then nothing to do
		if xy.is_none()
		{
			return;
		}
		let xy = xy.unwrap();
		// Get the mobile itself
		let mut mobile = world.fetch_mobile(self.id).unwrap();
		// Should we become aggressive?
		if mobile.aggressive
		{
			// Is there a target for us?
			if world.mobile_exists_at(xy.0, xy.1)
			{
				let target = world.fetch_mobile_at_random(xy.0, xy.1).unwrap();
				if target.name != mobile.name && mobile.use_action()
				{
					world.message_list.broadcast(mobile.name_with_article.clone()+" attacks "+&target.name_with_article+"!",xy.0,xy.1);	
					event_q.insert(Box::new(CombatEvent { attacker: self.id, defender: target.get_id() }));
				}
				world.add_mobile(target,xy.0,xy.1);
			}
		}
		// Should we wander
		if mobile.wanders
		{
			let die = Dice { number: 1 , die: 100 };
			let direction = match die.roll()
					{
						1 => { (1,0) },
						2 => { (-1,0) },
						3 => { (0,-1) },
						4 => { (0,1) },
						_ => { (0,0) }
					};
			if direction != (0,0) 
			{
				let next_location_type = world.get_location_type(xy.0+direction.0,xy.1+direction.1);
				let current_location_type = world.get_location_type(xy.0,xy.1);
				if current_location_type == next_location_type
				{
					event_q.insert(Box::new(MoveMobileEvent{ uuid: mobile.get_id(), dx: direction.0, dy: direction.1 }));
				}
			}
		}
		// Reschedule the event
		if mobile.is_active()
		{
			event_q.insert(Box::new(ActiveMonsterEvent { id: mobile.get_id() }));
		}
		// Put the mobile back into its place
		world.add_mobile(mobile, xy.0, xy.1);
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
				},
			LocationTypeCode::Town => 
				{
					let monster = self.town_wandering_monster();
					match monster
					{
						Some(monster) => { self.monster_list.push_back((location.x,location.y,monster)); },
						_ => { return; }
					}
				},
			LocationTypeCode::DeepWoods => 
				{
					let monster = self.deep_woods_wandering_monster();
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

	fn deep_woods_wandering_monster(&self) -> Option<Box<Mobile> >
	{
		let pick = random::<u8>();
		match pick
		{
			0 => { return Some(Mobile::goblin()); },
			1 => { return Some(Mobile::head_hunter()); },
			_ => { return None; }
		}
	}

	fn forest_wandering_monster(&self) -> Option<Box<Mobile> >
	{
		let pick = random::<u8>();
		match pick
		{
			0 => { return Some(Mobile::rodent()); },
			1 => { return Some(Mobile::lumber_jack()); },
			2 => { return Some(Mobile::rabbit()); },
			3 => { return Some(Mobile::small_woodland_creature()); },
			_ => { return None; }
		}
	}

	fn town_wandering_monster(&self) -> Option<Box<Mobile> >
	{
		let pick = random::<u8>();
		match pick
		{
			0 => { return Some(Mobile::beggar()); },
			1 => { return Some(Mobile::foppish_dandy()); },
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
		print!("density = {}",world.population_density());
		if world.population_density() < 0.25
		{
			let mut visitor = WanderingMonsterLocationVisitor { monster_list: LinkedList::new() };
			world.visit_all_locations(&mut visitor);
			for item in visitor.monster_list
			{
				let name = item.2.get_name();
				let msg = "A ".to_owned()+&name+" has arrived.";
				world.message_list.broadcast(msg,item.0,item.1);
				if item.2.is_active()
				{
					event_q.insert(Box::new(ActiveMonsterEvent { id: item.2.get_id() }));
				}
				world.add_mobile(item.2,item.0,item.1);
			}
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
	pub thief: usize,
	pub mark: usize,
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


// Make some item
pub struct MakeItemEvent
{
	pub maker: usize,
	pub item: ItemTypeCode,
}

impl Event for MakeItemEvent
{
	fn tick(&self, world: &mut WorldState, event_q: &mut EventList)
	{
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
			event_q.insert(Box::new(MakeItemEvent { maker: self.maker, item: self.item }));
		}
		else
		{
			match self.item
			{
				ItemTypeCode::HideArmor => self.make_hide_armor(&mut mobile,position,world),
				ItemTypeCode::LeatherArmor => self.make_leather_armor(&mut mobile,position,world),
				ItemTypeCode::Rawhide => self.make_rawhide(&mut mobile,position,world),
				ItemTypeCode::PointedStick => self.make_pointed_stick(&mut mobile,position,world),
				_ => { () }
			}
		}
		world.add_mobile(mobile,position.0,position.1);
	}
}

impl MakeItemEvent
{
	fn make_hide_armor(&self, mobile: &mut Box<Mobile>, position: (i16,i16), world: &mut WorldState)
	{
		let corpse = mobile.fetch_item_by_name(&"corpse".to_string());
		if corpse.is_some()
		{
			if mobile.roll_leatherwork_or_woodcraft() > Mobile::routine_task()
			{
				world.message_list.broadcast(mobile.name_with_article.clone()+&" makes some hide armor".to_string(),position.0,position.1);
				let armor = Item::hide_armor();
				world.add_item(position.0,position.1,armor);
			}
			else
			{
				world.message_list.broadcast(mobile.name_with_article.clone()+&" ruins a corpse".to_string(),position.0,position.1);	
			}
		}
	}

	fn make_rawhide(&self, mobile: &mut Box<Mobile>, position: (i16,i16), world: &mut WorldState)
	{
		let mut successes = 0;
		loop
		{
			let corpse = mobile.fetch_item_by_name(&"corpse".to_string());
			if corpse.is_none()
			{
				break;
			}
			if mobile.roll_leatherwork_or_woodcraft() > Mobile::routine_task()
			{
				successes += 1;
				mobile.add_item(Item::rawhide(),false);
			}
		}
		world.message_list.broadcast(mobile.name_with_article.clone()+&" makes ".to_string()+&successes.to_string()+
			&" pieces of rawhide!".to_string(),position.0,position.1);	
	}

	fn make_leather_armor(&self, mobile: &mut Box<Mobile>, position: (i16,i16), world: &mut WorldState)
	{
		let rawhide = mobile.fetch_item_by_name(&"rawhide".to_string());
		if rawhide.is_some()
		{
			if mobile.roll_leatherwork() > Mobile::skilled_task()
			{
				world.message_list.broadcast(mobile.name_with_article.clone()+&" makes some leather armor".to_string(),position.0,position.1);
				let armor = Item::leather_armor();
				world.add_item(position.0,position.1,armor);
			}
			else
			{
				world.message_list.broadcast(mobile.name_with_article.clone()+&" ruins some rawhide".to_string(),position.0,position.1);	
			}
		}
	}

	fn make_pointed_stick(&self, mobile: &mut Box<Mobile>, position: (i16,i16), world: &mut WorldState)
	{
		if mobile.roll_woodcraft() > Mobile::easy_task()
		{
			world.message_list.broadcast(mobile.name_with_article.clone()+&" sharpens a stick".to_string(),position.0,position.1);
			let armor = Item::pointed_stick();
			world.add_item(position.0,position.1,armor);
		}
		else
		{
			world.message_list.broadcast(mobile.name_with_article.clone()+&" ruins a stick".to_string(),position.0,position.1);	
		}
	}
}
