use crate::location::*;
use crate::message::MessageList;
use crate::map::*;
use crate::mobile::*;
use crate::items::*;
use std::collections::BTreeMap;

pub struct WorldState
{
	pub message_list: MessageList,
	map: Map,
	mobile_uuid_to_location: BTreeMap<usize,(i16,i16)>,
	stash: BTreeMap<usize,(Box<Mobile>,i16,i16)>
}

impl WorldState
{
	pub fn new() -> WorldState
	{
		return WorldState
		{
			stash: BTreeMap::new(),
			map: Map::new(),
			message_list: MessageList::new(),
			mobile_uuid_to_location: BTreeMap::new(),
		}
	}

	pub fn population_density(&self) -> f32
	{
		let area = self.map.number_of_locations();
		let population = self.mobile_uuid_to_location.len();
		return population as f32 / area as f32;
	}

	pub fn stash_mobile(&mut self, uuid: usize)
	{
		let position = self.find_mobile_location(uuid).unwrap();
		let mobile = self.fetch_mobile(uuid).unwrap();
		let id = mobile.get_id();
		let tuple = (mobile,position.0,position.1);
		self.stash.insert(id,tuple);
	}

	pub fn find_mobile_location(&mut self, uuid: usize) -> Option<(i16,i16)>
	{
		return self.mobile_uuid_to_location.get(&uuid).copied();
	}

	pub fn mobile_active(&mut self, uuid: usize) -> bool
	{
		return self.mobile_uuid_to_location.contains_key(&uuid);
	}

	pub fn mobile_exists_at(&self, x: i16, y: i16) -> bool
	{
		return self.map.is_mobile_at_location(x,y);
	}

	pub fn mobile_exists(&mut self, uuid: usize) -> bool
	{
		if self.mobile_uuid_to_location.contains_key(&uuid)
		{
			return true;
		}
		let tuple = self.stash.remove(&uuid);
		match tuple
		{
			Some((mobile,x,y)) => { self.add_mobile(mobile,x,y); return true; }
			_ => { return false; }
		}
	}

	pub fn fetch_mobile_at_random(&mut self, x: i16, y: i16) -> Option<Box<Mobile> >
	{
		let mut location = self.map.fetch(x,y);
		let mobile = location.fetch_mobile_at_random();
		self.map.replace(location);
		return mobile;
	}

	// Find and return a mobile. This removes it from the world and it
	// must be added back to the world when you are done with it.
	pub fn fetch_mobile(&mut self, uuid: usize) -> Option<Box<Mobile> >
	{
		let position_ptr = self.mobile_uuid_to_location.remove(&uuid);
		match position_ptr
		{
			Some(position) =>
				{
					let mut location = self.map.fetch(position.0,position.1);
					let mobile = location.fetch_mobile_by_guid(uuid);
					self.map.replace(location);
					return mobile;
				},
			_ => { return None; }
		}
	}

	pub fn fetch_mobile_by_name(&mut self, x: i16, y: i16, key: &String) -> Option<Box<Mobile> >
	{
		let mut location = self.map.fetch(x,y);
		let mobile = location.fetch_mobile_by_name(key);
		self.map.replace(location);
		return mobile;
	}

	// Find and return a mobile by name. This removes it from the world and it
	// must be added back to the world when you are done with it.
	pub fn get_mobile_id_by_name(&mut self, x: i16, y: i16, key: &String) -> Option<usize>
	{
		let mut location = self.map.fetch(x,y);
		let mobile = location.fetch_mobile_by_name(key);
		match mobile
		{
			Some(mobile) =>
				{ 
					let uuid = mobile.get_id();
					location.add_mobile(mobile);
					self.map.replace(location);
					return Some(uuid);
				}
			_ => {
					self.map.replace(location);
					return None;
				}
		}
	}

	pub fn add_item(&mut self, x: i16, y: i16, item: Box<Item>)
	{
		let mut location = self.map.fetch(x,y);
		location.add_item(item);
		self.map.replace(location);
	}

	pub fn fetch_item_by_name(&mut self, x: i16, y: i16, key: &String) -> Option<Box<Item> >
	{
		let mut location = self.map.fetch(x,y);
		let item = location.fetch_item_by_name(key);
		self.map.replace(location);
		return item;	
	}

	pub fn fetch_item_at_random(&mut self, x: i16, y: i16) -> Option<Box<Item> >
	{
		let mut location = self.map.fetch(x,y);
		let item = location.fetch_item_at_random();
		self.map.replace(location);
		return item;	
	}

	pub fn get_location_description(&mut self, x: i16, y: i16) -> String
	{
		return self.map.get_location_description(x,y);
	}

	pub fn get_location_type(&self, x: i16, y: i16) -> LocationTypeCode
	{
		return self.map.get_location_type(x,y);
	}

	pub fn add_corpse(&mut self, mobile: &mut Box<Mobile>, x: i16, y: i16)
	{
		let mut location = self.map.fetch(x,y);
		location.add_corpse(mobile);
		self.map.replace(location);
	}

	pub fn add_mobile(&mut self, mobile: Box<Mobile>, x: i16, y: i16)
	{
		let mut location = self.map.fetch(x,y);
		self.mobile_uuid_to_location.insert(mobile.get_id(),(x,y));
		location.add_mobile(mobile);
		self.map.replace(location);
	}

	pub fn visit_all_locations(&mut self, visitor: &mut impl LocationVisitor)
	{
		self.map.visit_all_locations(visitor,&mut self.message_list);
	}
}

