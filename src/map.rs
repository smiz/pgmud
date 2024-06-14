use crate::location::Location;
use crate::location::LocationTypeCode;
use crate::dice::*;
use std::collections::BTreeMap;
use crate::message::*;

pub trait LocationVisitor
{
	fn visit_location(&mut self, location: &mut Box<Location>, messages: &mut MessageList);
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

	pub fn get_location_type(&self, x: i16, y: i16) -> LocationTypeCode
	{
		let location = self.location_by_position.get(&(x,y));
		match location
		{
			Some(location) => { return location.location_type; },
			_ => { return LocationTypeCode::Unexplored; }
		}
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

	fn count_adjacent(&self, x:i16, y: i16, location_type: LocationTypeCode) -> usize
	{
		let mut count = 0;
		if self.get_location_type(y-1,x) == location_type { count += 1; }
		if self.get_location_type(y+1,x) == location_type { count += 1; }
		if self.get_location_type(y,x-1) == location_type { count += 1; }
		if self.get_location_type(y,x+1) == location_type { count += 1; }
		return count;
	}

	fn make_new_location(&mut self, x: i16, y: i16) -> Box<Location>
	{
		let dice = Dice { number: 1, die: 10 };
		let _forest_count = self.count_adjacent(x,y,LocationTypeCode::Forest);
		let town_count = self.count_adjacent(x,y,LocationTypeCode::Town);
		let unexplored_count = self.count_adjacent(x,y,LocationTypeCode::Unexplored);
		if town_count > 0 || unexplored_count > 1 || dice.roll() < 10
		{
			return Box::new(
				Location::new(x,y,LocationTypeCode::Forest,"In the forest".to_string()));
		}
		else
		{
			return Box::new(
				Location::new(x,y,LocationTypeCode::Town,"A small town".to_string()));
		}
	}

	pub fn visit_all_locations(&mut self, visitor: &mut impl LocationVisitor, messages: &mut MessageList)
	{
		for (_,mut location) in self.location_by_position.iter_mut()
		{
			visitor.visit_location(&mut location,messages);
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
