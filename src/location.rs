// Basic types of locations for map generation
use crate::object::Object;
use crate::mobile::Mobile;
use std::collections::BTreeMap;
use crate::items::*;
use crate::message::*;
use uuid::Uuid;

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum LocationTypeCode
{
	Town,
	Forest,
	Unexplored
}

// A location on the map
pub struct Location
{
	pub x: i16,
	pub y: i16,
	description: String,
	pub location_type: LocationTypeCode,
	mobiles: BTreeMap<Uuid,Box<Mobile> >,
	items: Vec<Box<Item> >
}

impl Object for Location
{
	fn complete_description(&self) -> String
	{
		return self.description();
	}

	fn description(&self) -> String
	{
		let mut result = self.description.clone();
		for (_,mobile) in self.mobiles.iter()
		{
			let description = mobile.description();
			result += "\n";
			result += &description;
		}
		for item in self.items.iter()
		{
			let description = item.description();
			result += "\n";
			result += &description;
		}
		return result;
	}

	fn get_name(&self) -> String { return self.description.clone(); }
}

impl Location
{

	pub fn num_mobiles(&self) -> usize
	{
		return self.mobiles.len();
	}

	pub fn new(x: i16, y: i16, code: LocationTypeCode,
		description: String) -> Location
	{
		let mut result = Location
		{
			x: x,
			y: y,
			description: description.clone(),
			location_type: code.clone(),
			mobiles: BTreeMap::new(),
			items: Vec::new()
		};
		match result.location_type
		{
			LocationTypeCode::Forest =>
				{
					result.items.push(Item::forest_debris());
					return result;
				},
			_ => { return result; }
		}
	}

	pub fn add_item(&mut self, item: Box<Item>)
	{
		self.items.push(item);
	}

	pub fn add_corpse(&mut self, mobile: &mut Box<Mobile>)
	{
		self.add_item(Item::corpse(mobile.name.clone()));
		loop
		{
			let item = mobile.fetch_last_item();
			if item.is_some()
			{
				self.add_item(item.unwrap());
			}
			else
			{
				return;
			}
		}
	}

	pub fn add_mobile(&mut self, mobile: Box<Mobile>)
	{
		self.mobiles.insert(mobile.get_id(),mobile);
	}

	pub fn fetch_mobile_by_guid(&mut self, key: Uuid) -> Option<Box<Mobile> >
	{
		return self.mobiles.remove(&key);
	}

	pub fn age_all_items(&mut self, messages: &mut MessageList)
	{
		let mut i = 0;
		while i < self.items.len()
		{
			self.items[i].tick();
    		if self.items[i].lifetime == 0
			{
				messages.post_for_all(self.items[i].name.clone()+" decays into dust",self.x,self.y);
				self.items.remove(i);
			}
			else
			{
				i += 1;
			}
		}
	}

	pub fn age_all_mobiles(&mut self)
	{
		for (_,mobile) in self.mobiles.iter_mut()
		{
			mobile.tick();
		}
	}

	pub fn fetch_mobile_by_name(&mut self, key: &String) -> Option<Box<Mobile> >
	{
		let mut lower_case_key = key.clone();
		lower_case_key.make_ascii_lowercase();
		for (_,mobile) in self.mobiles.iter()
		{
			let mut mobile_name = mobile.name.clone();
			mobile_name.make_ascii_lowercase();	
			if mobile_name.contains(&lower_case_key)
			{
				return self.fetch_mobile_by_guid(mobile.get_id());
			}
		}
		return None;
	}

	pub fn fetch_item_by_name(&mut self, key: &String) -> Option<Box<Item> >
	{
		let mut i = 0;
		let mut lower_case_key = key.clone();
		lower_case_key.make_ascii_lowercase();
		while i < self.items.len()
		{
			let mut name = self.items[i].name.clone();
			name.make_ascii_lowercase();
    		if name.contains(&lower_case_key)
			{
				return Some(self.items.remove(i));
			}
			i += 1;
		}
		return None;
	}
}

#[cfg(test)]
mod location_unit_test
{
	use super::*;
	use crate::items::*;
	use crate::mobile::*;

	#[test]
	fn add_and_fetch_test()
	{
		let mut location = Location::new(0,0,LocationTypeCode::Forest,"Forest".to_string());
		let foot = Item::rabbit_foot();
		location.add_item(foot);
		let found_item = location.fetch_item_by_name(&"foot".to_string());
		assert!(found_item.is_some());
		let found_again = location.fetch_item_by_name(&"foot".to_string());
		assert!(found_again.is_none());
	}

	#[test]
	fn add_corpse()
	{
		let mut location = Location::new(0,0,LocationTypeCode::Forest,"Forest".to_string());
		let mut rabbit = Mobile::rabbit();
		location.add_corpse(&mut rabbit);
		let found_item = location.fetch_item_by_name(&"foot".to_string());
		assert!(found_item.is_some());
	}
}
