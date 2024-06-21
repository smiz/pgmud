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
	Sword,
	Axe,
	PointedStick,
	Rawhide,
	LeatherArmor,
}

#[derive(Copy,Clone)]
pub enum ItemCategoryCode
{
	Misc,
	Weapon,
	Armor
}

pub struct Item
{
	pub description: String,
	pub name: String,
	pub effect: String,
	pub type_code: ItemTypeCode,
	pub category_code: ItemCategoryCode,
	// How many ticks until it goes away?
	pub lifetime: u32,
	// How much xp for getting this item?
	pub xp_value: i16,
	// How common is this item?
	pub frequency: i16,
	// Armor protection provided if this is armor?
	pub armor_value: i16,
}

impl Object for Item
{
	fn complete_description(&self) -> String
	{
		let mut result = self.effect.clone();
		match self.category_code
		{
			ItemCategoryCode::Armor =>
				{
					result += &(" Protection is ".to_string()+&self.armor_value.to_string());
					return result;
				},
			_ => { return result; }
		}
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

	pub fn eat(&mut self,  mobile: &mut Mobile) -> bool
	{
		match self.type_code
		{
			ItemTypeCode::HealthyNutsAndSeeds => { mobile.damage -= 1; return true; }
			_ => { return false; }
		}
	}

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
			ItemTypeCode::Sword => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 8}; }
			ItemTypeCode::PointedStick => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 4}; }
			ItemTypeCode::Axe => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 8}; }
			_ => { return; }
		}
	}

	pub fn drop_item(&self, mobile: &mut Mobile)
	{
		match self.type_code
		{
			ItemTypeCode::RabbitFoot => { mobile.luck -= 1; }
			ItemTypeCode::GreenPenny => { mobile.luck -= 1; }
			_ => { return; }
		}
	}

	pub fn tick(&mut self)
	{
		self.lifetime -= 1;
	}

	pub fn basic_item(type_code: ItemTypeCode, cat_code: ItemCategoryCode) -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "".to_string(),
				name: "".to_string(),
				effect: "".to_string(),
				frequency: 25,
				type_code: type_code,
				category_code: cat_code,
				xp_value: 0,
				lifetime: 100,
				armor_value: 0,
			});
	}

	pub fn corpse(in_life: String) -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A dead ".to_string()+&in_life+" is here.",
				name: in_life+" corpse",
				effect: "The clay left behind when the spirit is fled.".to_string(),
				frequency: 50,
				type_code: ItemTypeCode::Corpse,
				category_code: ItemCategoryCode::Misc,
				xp_value: 0,
				lifetime: 100,
				armor_value: 0,
			});
	}

	pub fn rawhide() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A bit of rawhide is here.".to_string(),
				name: "rawhide".to_string(),
				effect: "This can be made into many useful items.".to_string(),
				frequency: 25,
				type_code: ItemTypeCode::Rawhide,
				category_code: ItemCategoryCode::Misc,
				xp_value: 0,
				lifetime: 100,
				armor_value: 0,
			});
	}

	pub fn leather_armor() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A suit of leather armor is here.".to_string(),
				name: "leather armor".to_string(),
				effect: "Will protect you from harm!.".to_string(),
				frequency: 25,
				type_code: ItemTypeCode::LeatherArmor,
				category_code: ItemCategoryCode::Armor,
				xp_value: 1,
				lifetime: 1000,
				armor_value: 10,
			});
	}

	pub fn axe() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::Axe,ItemCategoryCode::Weapon);
		item.description = "A gleaming axe is here.".to_string();
		item.name = "axe".to_string();
		item.effect = "A sharp axe dealing 1d8 damage.".to_string();
		item.frequency = 50;
		item.xp_value = 1;
		item.lifetime = 1000;
		return item;
	}

	pub fn sword() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A sword is here.".to_string(),
				name: "sword".to_string(),
				effect: "A sharp sword dealing 1d8 damage.".to_string(),
				frequency: 50,
				type_code: ItemTypeCode::Sword,
				category_code: ItemCategoryCode::Weapon,
				xp_value: 1,
				lifetime: 1000,
				armor_value: 0,
			});
	}

	pub fn pointed_stick() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A pointed stick is here.".to_string(),
				name: "pointed stick".to_string(),
				effect: "A pointed stick deals 1d4 damage.".to_string(),
				frequency: 50,
				type_code: ItemTypeCode::PointedStick,
				category_code: ItemCategoryCode::Weapon,
				xp_value: 1,
				lifetime: 100,
				armor_value: 0,
			});
	}
	pub fn rabbit_foot() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A soft rabbits foot is here.".to_string(),
				name: "rabbit foot".to_string(),
				effect: "A lucky rabbit foot! +1 to luck.".to_string(),
				frequency: 100,
				type_code: ItemTypeCode::RabbitFoot,
				category_code: ItemCategoryCode::Misc,
				xp_value: 1,
				lifetime: 1000,
				armor_value: 0,
			});
	}
	pub fn green_penny() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A greenish penny is here.".to_string(),
				name: "greenish penny".to_string(),
				effect: "A lucky penny! +1 to luck.".to_string(),
				frequency: 100,
				type_code: ItemTypeCode::GreenPenny,
				category_code: ItemCategoryCode::Misc,
				xp_value: 1,
				lifetime: 10000,
				armor_value: 0,
			});
	}
	pub fn healthy_nuts_and_seeds() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "You see a healthy mix of nuts & seeds.".to_string(),
				name: "healthy nuts & seeds".to_string(),
				effect: "You should eat better! -1 to damage.".to_string(),
				frequency: 100,
				type_code: ItemTypeCode::HealthyNutsAndSeeds,
				category_code: ItemCategoryCode::Misc,
				xp_value: 1,
				lifetime: 10000,
				armor_value: 0,
			});
	}
	pub fn forest_debris() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "Some twigs and leaves litter the forest floor.".to_string(),
				name: "leaves and twigs".to_string(),
				effect: "Just forest dentritis.".to_string(),
				frequency: 0,
				type_code: ItemTypeCode::ForestDebris,
				category_code: ItemCategoryCode::Misc,
				xp_value: 0,
				lifetime: std::u32::MAX,
				armor_value: 0,
			});
	}

	pub fn woodland_trinket() -> Option<Box<Item> >
	{
		let die = Dice { number: 1, die: 8 };
		let roll = die.roll();
		match roll
		{
			1 => { return Some(Self::rabbit_foot()); },
			2 => { return Some(Self::healthy_nuts_and_seeds()); },
			_ => { return None; }
		}
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
		let foot = Item::rabbit_foot();
		let mut mobile = Mobile::new_character(&"Jim".to_string());
		let luck = mobile.luck;
		mobile.add_item(foot,true);
		assert_eq!(luck+1,mobile.luck);
		let new_foot = mobile.fetch_item_by_position(0);
		assert_eq!(luck,mobile.luck);
		assert!(new_foot.is_some());
	}

	#[test]
	fn eat_test()
	{
		let nuts = Item::healthy_nuts_and_seeds();
		let mut mobile = Mobile::new_character(&"Jim".to_string());
		let damage = mobile.damage;
		mobile.add_item(nuts,true);
		mobile.eat_item_by_name(&"nut".to_string());
		assert_eq!(damage-1,mobile.damage);
		let new_nut = mobile.fetch_item_by_name(&"nut".to_string());
		assert!(new_nut.is_none());
	}
}
