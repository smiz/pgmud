// Basic types of locations for map generation
use crate::object::Object;
use crate::mobile::Mobile;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Copy,Clone)]
pub enum LocationTypeCode
{
	Town,
	Forest,
}

// A location on the map
pub struct Location
{
	pub x: i16,
	pub y: i16,
	description: String,
	pub location_type: LocationTypeCode,
	mobiles: BTreeMap<Uuid,Box<Mobile> >,
	id: Uuid
}

impl Object for Location
{
	fn description(&self) -> String
	{
		let mut result = self.description.clone();
		for (_,mobile) in self.mobiles.iter()
		{
			let description = mobile.description();
			result += "\n";
			result += &description;
		}
		return result;
	}

	fn get_id(&self) -> Uuid { return self.id; }
	fn get_name(&self) -> String { return self.description.clone(); }
}

impl Location
{
	pub fn new(x: i16, y: i16, code: LocationTypeCode,
		description: String) -> Location
	{
		return Location
		{
			x: x,
			y: y,
			description: description.clone(),
			location_type: code.clone(),
			mobiles: BTreeMap::new(),
			id: Uuid::new_v4(),
		};
	}

	pub fn add_mobile(&mut self, mobile: Box<Mobile>)
	{
		self.mobiles.insert(mobile.get_id(),mobile);
	}

	pub fn contains_mobile(&self, key: &Uuid) -> bool
	{
		return self.mobiles.contains_key(&key);
	}

	pub fn fetch_mobile_by_guid(&mut self, key: Uuid) -> Option<Box<Mobile> >
	{
		return self.mobiles.remove(&key);
	}
}

#[cfg(test)]
mod location_unit_test
{
	use super::*;
}
