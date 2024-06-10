use crate::object::Object;
use crate::mobile::*;

#[derive(Copy,Clone)]
pub enum ItemTypeCode
{
	RabbitFoot,
	ForestDebris,
	Corpse
}

pub struct Item
{
	pub description: String,
	pub name: String,
	pub type_code: ItemTypeCode,
	// How many ticks until it goes away?
	pub lifetime: u32,
	// How much xp for getting this item?
	pub xp_value: i16
}

impl Object for Item
{
	fn complete_description(&self) -> String
	{
		return self.description.clone();
	}

	fn description(&self) -> String
	{
		return self.description.clone();
	}

	fn get_name(&self) -> String
	{
		return self.name.clone();
	}
}

impl Item
{
	pub fn got_item(&mut self, mobile: &mut Mobile)
	{
		mobile.xp += self.xp_value;
		self.xp_value = 0;
		match self.type_code
		{
			ItemTypeCode::RabbitFoot => { mobile.luck += 1; }
			_ => { return; }
		}
	}

	pub fn drop_item(&self, mobile: &mut Mobile)
	{
		match self.type_code
		{
			ItemTypeCode::RabbitFoot => { mobile.luck -= 1; }
			_ => { return; }
		}
	}

	pub fn tick(&mut self)
	{
		self.lifetime -= 1;
	}

	pub fn corpse(in_life: String) -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A dead ".to_string()+&in_life+" is here.",
				name: in_life+" corpse",
				type_code: ItemTypeCode::Corpse,
				xp_value: 0,
				lifetime: 100,
			});
	}

	pub fn rabbit_foot() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A soft rabbits foot is here.".to_string(),
				name: "rabbit foot".to_string(),
				type_code: ItemTypeCode::RabbitFoot,
				xp_value: 1,
				lifetime: 1000,
			});
	}
	pub fn forest_debris() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "Some twigs and leaves litter the forest floor.".to_string(),
				name: "leaves and twigs".to_string(),
				type_code: ItemTypeCode::ForestDebris,
				xp_value: 0,
				lifetime: std::u32::MAX,
			});
	}
}

#[cfg(test)]
mod items_unit_test
{
	use super::*;
	use crate::mobile::*;

	#[test]
	fn add_rabbit_foot_test()
	{
		let mut foot = Item::rabbit_foot();
		let mut mobile = Mobile::new_character("Jim".to_string());
		let luck = mobile.luck;
		mobile.add_item(foot);
		assert_eq!(luck+1,mobile.luck);
		let new_foot = mobile.fetch_item_by_position(0);
		assert_eq!(luck,mobile.luck);
		assert!(new_foot.is_some());
	}
}
