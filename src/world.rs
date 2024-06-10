use crate::message::MessageList;
use crate::map::*;
use crate::mobile::*;
use crate::object::*;
use std::collections::BTreeMap;
use uuid::Uuid;

pub struct WorldState
{
	pub message_list: MessageList,
	map: Map,
	mobile_uuid_to_location: BTreeMap<Uuid,(i16,i16)>
}

impl WorldState
{
	pub fn new() -> WorldState
	{
		return WorldState
		{
			map: Map::new(),
			message_list: MessageList::new(),
			mobile_uuid_to_location: BTreeMap::new()
		}
	}

	pub fn find_mobile_location(&mut self, uuid: Uuid) -> Option<(i16,i16)>
	{
		return self.mobile_uuid_to_location.get(&uuid).copied();
	}

	pub fn mobile_exists(&mut self, uuid: Uuid) -> bool
	{
		return self.mobile_uuid_to_location.contains_key(&uuid);
	}

	// Find and return a mobile. This removes it from the world and it
	// must be added back to the world when you are done with it.
	pub fn fetch_mobile(&mut self, uuid: Uuid) -> Option<Box<Mobile> >
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

	// Find and return a mobile by name. This removes it from the world and it
	// must be added back to the world when you are done with it.
	pub fn get_mobile_id_by_name(&mut self, x: i16, y: i16, key: String) -> Option<Uuid>
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

	pub fn get_location_description(&mut self, x: i16, y: i16) -> String
	{
		let location = self.map.fetch(x,y);
		let result = location.description();
		self.map.replace(location);
		return result;
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
		self.map.visit_all_locations(visitor);
	}
}

