use crate::location::Location;
use crate::location::LocationTypeCode;
use crate::dice::*;
use crate::Object;
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

	pub fn number_of_locations(&self) -> usize
	{
		return self.location_by_position.len();
	}

	pub fn is_mobile_at_location(&self, x: i16, y: i16) -> bool
	{
		let location = self.location_by_position.get(&(x,y));
		match location
		{
			Some(location) => { return location.has_mobiles(); },
			_ => { return false; }
		}
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

	pub fn get_location_description(&self, x: i16, y: i16) -> String
	{
		let location = self.location_by_position.get(&(x,y));
		match location
		{
			Some(location) => { return location.description(); },
			_ => { return "Unexplored".to_string(); }
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

	fn count_adjacent(&self, x:i16, y: i16, location_type: LocationTypeCode) -> i16 
	{
		let mut count = 0;
		for dx in -1..2
		{
			for dy in -1..2
			{
				if dx == 0 && dy == 0 { continue; }
				if self.get_location_type(x+dx,y+dy) == location_type { count += 1; }
			}
		}
		return count;
	}

	fn make_new_location(&mut self, x: i16, y: i16) -> Box<Location>
	{
		let d8 = Dice { number: 1, die: 8 };
		let deep_woods_count = self.count_adjacent(x,y,LocationTypeCode::DeepWoods);
		let hills_count = self.count_adjacent(x,y,LocationTypeCode::Hills);
		// Get our manhattan distance from the origin
		let distance = x.abs() + y.abs();
		if distance <= 2 || d8.roll() > distance
		{
			return Box::new(Location::new(x,y,LocationTypeCode::Forest,"In the forest".to_string()));
		}
		else
		{
			let roll = d8.roll();
			if roll <= hills_count
			{
				return Box::new(Location::new(x,y,LocationTypeCode::Hills,"In the hills".to_string()));
			}
			else if roll <= deep_woods_count+hills_count
			{
				return Box::new(Location::new(x,y,LocationTypeCode::DeepWoods,"In the deep woods".to_string()));
			}
			else if roll % 2 == 0
			{
				return Box::new(Location::new(x,y,LocationTypeCode::DeepWoods,"In the deep woods".to_string()));
			}
			else 
			{
				return Box::new(Location::new(x,y,LocationTypeCode::Hills,"In the hills".to_string()));
			}
		}
	}

	pub fn visit_all_locations(&mut self, visitor: &mut impl LocationVisitor, messages: &mut MessageList)
	{
		for (_,mut location) in self.location_by_position.iter_mut()
		{
			visitor.visit_location(&mut location,messages);
		}
	}

	pub fn draw_map(&self) -> String
	{
		let mut result = String::new();
		let keys = self.location_by_position.keys();
		let mut x_min = 9999;
		let mut x_max = 0;
		let mut y_min = 9999;
		let mut y_max = 0;

		for key in keys
		{
			if key.0 < x_min { x_min = key.0; }
			if key.0 > x_max { x_max = key.0; }
			if key.1 < y_min { y_min = key.1; }
			if key.1 > y_max { y_max = key.1; }
		}

		for y in y_min..y_max+1
		{
			for x in x_min..x_max+1
			{
				let location = self.get_location_type(x, y);
				match location
				{
					LocationTypeCode::Town => result.push_str("T"),
					LocationTypeCode::Forest => result.push_str("-"),
					LocationTypeCode::DeepWoods => result.push_str("*"),
					LocationTypeCode::Hills => result.push_str("^"),
					LocationTypeCode::Unexplored => result.push_str(" "),
				}
			}
			result.push_str("\n");
		}
		return result;
	}
}

#[cfg(test)]
mod map_unit_test
{
	use super::*;
	use std::fs::File;
	use std::io::Write;
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

	#[test]
	fn random_walk_map()
	{
		let die = Dice { number: 1, die: 4 };
		let mut x = 0;
		let mut y = 0;
		let mut map = Map::new();
		for _ in 0..1000
		{
			let direction = match die.roll()
			{
				1 => (0,1),
				2 => (0,-1),
				3 => (1,0),
				_ => (-1,0),
			};
			x += direction.0;
			y += direction.1;
			let location = map.fetch(x,y);
			map.replace(location); 
		}
		let mut file = File::create("map.txt").unwrap();
		file.write_all(map.draw_map().as_bytes()).unwrap();
		file.flush();
	}

}
