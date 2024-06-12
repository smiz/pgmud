use crate::object::Object;
use crate::mobile::*;
use crate::dice::*;

#[derive(Copy,Clone)]
pub enum ItemTypeCode
{
	HealthyNutsAndSeeds,
	RabbitFoot,
	GreenPenny,
	ForestDebris,
	Corpse,
	Sword
}

#[derive(Copy,Clone)]
pub enum ItemCategoryCode
{
	Misc,
	Weapon
}

pub struct Item
{
	pub description: String,
	pub name: String,
	pub type_code: ItemTypeCode,
	pub category_code: ItemCategoryCode,
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

	pub fn got_item(&mut self, mobile: &mut Mobile, take_xp: bool)
	{
		if take_xp
		{
			mobile.xp += self.xp_value;
			self.xp_value = 0;
		}
		match self.type_code
		{
			ItemTypeCode::RabbitFoot => { mobile.luck += 1; }
			ItemTypeCode::GreenPenny => { mobile.luck += 1; }
			ItemTypeCode::HealthyNutsAndSeeds => { mobile.max_damage += 1; }
			ItemTypeCode::Sword => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 8}; }
			_ => { return; }
		}
	}

	pub fn drop_item(&self, mobile: &mut Mobile)
	{
		match self.type_code
		{
			ItemTypeCode::RabbitFoot => { mobile.luck -= 1; }
			ItemTypeCode::GreenPenny => { mobile.luck -= 1; }
			ItemTypeCode::HealthyNutsAndSeeds => { mobile.max_damage -= 1; }
			ItemTypeCode::Sword => { mobile.unwield(); }
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
				category_code: ItemCategoryCode::Misc,
				xp_value: 0,
				lifetime: 100,
			});
	}

	pub fn sword() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A sword is here.".to_string(),
				name: "sword".to_string(),
				type_code: ItemTypeCode::Sword,
				category_code: ItemCategoryCode::Weapon,
				xp_value: 1,
				lifetime: 1000,
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
				category_code: ItemCategoryCode::Misc,
				xp_value: 1,
				lifetime: 1000,
			});
	}
	pub fn green_penny() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A greenish penny is here.".to_string(),
				name: "greenish penny".to_string(),
				type_code: ItemTypeCode::GreenPenny,
				category_code: ItemCategoryCode::Misc,
				xp_value: 1,
				lifetime: 10000,
			});
	}
	pub fn healthy_nuts_and_seeds() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "You see a healthy mix of nuts & seeds.".to_string(),
				name: "healthy nuts & seeds".to_string(),
				type_code: ItemTypeCode::HealthyNutsAndSeeds,
				category_code: ItemCategoryCode::Misc,
				xp_value: 1,
				lifetime: 10000,
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
				category_code: ItemCategoryCode::Misc,
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
