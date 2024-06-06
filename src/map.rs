use crate::location::Location;
use crate::location::LocationTypeCode;
use std::collections::BTreeMap;
use uuid::Uuid;

pub trait LocationVisitor
{
	fn visit_location(&mut self, location: & Box<Location>);
}

pub struct Map
{
	location_by_position: BTreeMap<(i16,i16),Box<Location> >
}

impl Map
{
	// Create a new map with an initial location at 0,0
	pub fn new() -> Map
	{
		let mut map = Map {	
			location_by_position: BTreeMap::new()
		};
		let start_location = Box::new(
			Location::new(0,0,LocationTypeCode::Town,"Town of Midgaard".to_string())
		);
		map.location_by_position.insert((0,0),start_location);
		return map;
	}

	// Find the location that contains a mobile with the given uuid
	pub fn find_mobile(&mut self, uuid: Uuid) -> Option<(i16,i16)>
	{
		for (pos,location) in self.location_by_position.iter()
		{
			if location.contains_mobile(&uuid) { return Some(*pos); }
		}
		return None;
	}

	// Fetch the location at x,y. It must be replaced when you
	// are done with the location.
	pub fn fetch(&mut self, x: i16, y: i16) -> Box<Location>
	{
		let location = self.location_by_position.remove(&(x,y));
		match location
		{
			Some(location) => return location,
			_ => return self.make_new_location(x,y),
		}
	}

	// Replace a location that you extracted from the map
	pub fn replace(&mut self, location: Box<Location>)
	{
		let position = (location.x,location.y);
		self.location_by_position.insert(position,location);
		// Update all of the mobile positions
	}

	fn make_new_location(&mut self, x: i16, y: i16) -> Box<Location>
	{
		return Box::new(
			Location::new(x,y,LocationTypeCode::Forest,"In the forest".to_string())
		);
	}

	pub fn visit_all_locations(&self, visitor: &mut impl LocationVisitor)
	{
		for (_,location) in self.location_by_position.iter()
		{
			visitor.visit_location(location);
		}
	}
}

#[cfg(test)]
mod map_unit_test
{
	use super::*;

	#[test]
	fn new_map()
	{
		let mut map = Map::new();
		let mut location = map.fetch(0,0);
		assert_eq!(location.x,0);
		assert_eq!(location.x,0);
		match location.location_type
		{
			LocationTypeCode::Town => assert!(true),
			_ => assert!(false),
		}
		location.location_type = LocationTypeCode::Forest;
		map.replace(location);
		location = map.fetch(0,0);
		match location.location_type
		{
			LocationTypeCode::Forest => assert!(true),
			_ => assert!(false),
		}
	}
}
