use crate::object::Object;
use crate::mobile::*;
use crate::dice::*;

#[derive(Copy,Clone,PartialEq)]
pub enum ItemTypeCode
{
	UncutGemstone,
	UselessRock,
	HealthyNutsAndSeeds,
	MetalIngot,
	DwarfBeard,
	RabbitFoot,
	GreenPenny,
	ForestDebris,
	Corpse,
	Sword,
	Pick,
	Axe,
	PointedStick,
	Rawhide,
	LeatherArmor,
	ChainArmor,
	HideArmor,
	BoneJewelry,
	GoldBauble,
	ShrunkenHead,
	StoneKnife,
	HealingPotion
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
	// Do you only get this xp by dropping the item in a town?
	pub xp_in_town_only: bool,
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
			ItemTypeCode::HealingPotion =>
				{
					let die = Dice { number: 2 , die: 4 };
					mobile.damage -= die.roll();
					return true;
				}
			_ => { return false; }
		}
	}

	pub fn got_item(&mut self, mobile: &mut Mobile, take_xp: bool)
	{
		if take_xp && !self.xp_in_town_only
		{
			mobile.xp += self.xp_value;
			self.xp_value = 0;
		}
		match self.type_code
		{
			ItemTypeCode::ShrunkenHead => { mobile.luck -= 1; }
			ItemTypeCode::DwarfBeard => { mobile.luck -= 1; }
			ItemTypeCode::RabbitFoot => { mobile.luck += 1; }
			ItemTypeCode::GreenPenny => { mobile.luck += 1; }
			ItemTypeCode::Sword => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 8}; }
			ItemTypeCode::Pick => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 6}; }
			ItemTypeCode::PointedStick => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 3}; }
			ItemTypeCode::Axe => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 8}; }
			ItemTypeCode::StoneKnife => { mobile.wielded = self.name.clone(); mobile.damage_dice = Dice { number: 1, die: 4}; }
			_ => { return; }
		}
	}

	pub fn drop_item(&mut self, mobile: &mut Mobile)
	{
		match self.type_code
		{
			ItemTypeCode::ShrunkenHead => { mobile.luck += 1; }
			ItemTypeCode::DwarfBeard => { mobile.luck += 1; }
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
				frequency: Mobile::easy_task(),
				type_code: type_code,
				category_code: cat_code,
				xp_value: 0,
				lifetime: 100,
				armor_value: 0,
				xp_in_town_only: false,
			});
	}

	pub fn healing_potion() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::HealingPotion,ItemCategoryCode::Misc);
		item.description = "A bottle of swirling liquid has been discarded here.".to_string();
		item.name = "potion".to_string();
		item.frequency = Mobile::skilled_task();
		item.effect = "This is a potion of healing!".to_string();
		item.xp_value = 1;
		return item;
	}

	pub fn corpse(in_life: String) -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::Corpse,ItemCategoryCode::Misc);
		item.description = "A dead ".to_string()+&in_life+&" is here.".to_string();
		item.name = in_life+&" corpse".to_string();
		item.frequency = Mobile::trivial_task();
		item.effect = "The clay left behind when the spirit is fled.".to_string();
		return item;
	}

	pub fn metal_ingot() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::MetalIngot,ItemCategoryCode::Misc);
		item.description = "An ignot of metal shines brighly in the sun.".to_string();
		item.name = "metal ingot".to_string();
		item.xp_value = 1;
		item.effect = "This can be made into many useful items.".to_string();
		return item;
	}

	pub fn rawhide() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::Rawhide,ItemCategoryCode::Misc);
		item.description = "A bit of rawhide is here.".to_string();
		item.name = "rawhide".to_string();
		item.effect = "This can be made into many useful items.".to_string();
		return item;
	}

	pub fn chainmail() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::ChainArmor,ItemCategoryCode::Armor);
		item.description = "A gleaning suit of chainmail sits in a pile.".to_string();
		item.name = "chainmail".to_string();
		item.effect = "Will protect you from harm!.".to_string();
		item.xp_value = 2;
		item.lifetime = 1000;
		item.armor_value = 4;
		return item;
	}

	pub fn hide_armor() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::HideArmor,ItemCategoryCode::Armor);
		item.description = "A stinking suit of hide armor is here.".to_string();
		item.name = "hide armor".to_string();
		item.effect = "Will protect you from harm!.".to_string();
		item.xp_value = 1;
		item.armor_value = 2;
		return item;
	}

	pub fn leather_armor() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::LeatherArmor,ItemCategoryCode::Armor);
		item.description = "A suit of leather armor is here.".to_string();
		item.name = "leather armor".to_string();
		item.effect = "Will protect you from harm!.".to_string();
		item.xp_value = 1;
		item.lifetime = 1000;
		item.armor_value = 3;
		return item;
	}

	pub fn axe() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::Axe,ItemCategoryCode::Weapon);
		item.description = "A gleaming axe is here.".to_string();
		item.name = "axe".to_string();
		item.effect = "A sharp axe dealing 1d8 damage.".to_string();
		item.xp_value = 1;
		item.lifetime = 1000;
		return item;
	}

	pub fn sword() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::Sword,ItemCategoryCode::Weapon);
		item.description = "A sword is here.".to_string();
		item.name = "sword".to_string();
		item.effect = "A sharp sword dealing 1d8 damage.".to_string();
		item.xp_value = 1;
		item.lifetime = 1000;
		return item;
	}

	pub fn pointed_stick() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::PointedStick,ItemCategoryCode::Weapon);
		item.description = "A pointed stick is here.".to_string();
		item.name = "pointed stick".to_string();
		item.effect = "A pointed stick deals 1d3 damage.".to_string();
		item.xp_value = 1;
		return item;
	}

	pub fn pick() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::Pick,ItemCategoryCode::Weapon);
		item.description = "A finely made pick axe.".to_string();
		item.name = "pick axe".to_string();
		item.effect = "A pick axe deals 1d6 damage.".to_string();
		item.xp_value = 1;
		item.lifetime = 1000;
		return item;
	}

	pub fn stone_knife() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::StoneKnife,ItemCategoryCode::Weapon);
		item.description = "A finely made and sharp stone knife.".to_string();
		item.name = "stone knife".to_string();
		item.effect = "A stone knife deals 1d4 damage.".to_string();
		item.xp_value = 1;
		item.lifetime = 1000;
		return item;
	}

	pub fn rabbit_foot() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::RabbitFoot,ItemCategoryCode::Misc);
		item.description = "A soft rabbits foot is here.".to_string();
		item.name = "rabbit foot".to_string();
		item.effect = "A lucky rabbit foot! +1 to luck.".to_string();
		item.frequency = Mobile::easy_task();
		item.xp_value = 1;
		return item;
	}

	pub fn green_penny() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::GreenPenny,ItemCategoryCode::Misc);
		item.description = "A greenish penny is here.".to_string();
		item.name = "greenish penny".to_string();
		item.effect = "A lucky penny! +1 to luck.".to_string();
		item.frequency = Mobile::easy_task();
		item.xp_value = 1;
		item.lifetime = 1000;
		return item;
	}

	pub fn healthy_nuts_and_seeds() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::HealthyNutsAndSeeds,ItemCategoryCode::Misc);
		item.description = "You see a healthy mix of nuts & seeds.".to_string();
		item.name = "healthy nuts & seeds".to_string();
		item.effect = "You should eat better! -1 to damage.".to_string();
		item.frequency = Mobile::easy_task();
		item.xp_value = 1;
		return item;
	}

	pub fn forest_debris() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::ForestDebris,ItemCategoryCode::Misc);
		item.description = "Some twigs and leaves litter the forest floor.".to_string();
		item.name = "leaves and twigs".to_string();
		item.effect = "Just forest dentritis.".to_string();
		item.lifetime = std::u32::MAX;
		return item;
	}

	pub fn bone_jewelry() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::BoneJewelry,ItemCategoryCode::Misc);
		item.description = "Some primitive bone jewelry has been discarded here.".to_string();
		item.name = "bone jewlery".to_string();
		item.effect = "This may be worth something in town.".to_string();
		item.frequency = Mobile::easy_task();
		item.xp_value = 5;
		item.lifetime = 1000;
		item.xp_in_town_only = true;
		return item;
	}

	pub fn gold_bauble() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::GoldBauble,ItemCategoryCode::Misc);
		item.description = "A golden bauble shines brightly in the sunlight.".to_string();
		item.name = "golden bauble".to_string();
		item.effect = "This may be worth something in town.".to_string();
		item.frequency = Mobile::easy_task();
		item.xp_value = 5;
		item.lifetime = 1000;
		item.xp_in_town_only = true;
		return item;
	}

	pub fn shrunken_head() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::ShrunkenHead,ItemCategoryCode::Misc);
		item.description = "A shrunken head discarded here fills you with misgivings.".to_string();
		item.name = "shrunken head".to_string();
		item.effect = "This horrid curio might interst a strange collector in town.".to_string();
		item.frequency = Mobile::easy_task();
		item.xp_value = 5;
		item.xp_in_town_only = true;
		return item;
	}

	pub fn dwarf_beard() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::DwarfBeard,ItemCategoryCode::Misc);
		item.description = "A dwarf's beard is crumpled here, but the dwarf is missing!".to_string();
		item.name = "dwarf's beard".to_string();
		item.effect = "This horrid curio might interst a strange collector in town.".to_string();
		item.frequency = Mobile::easy_task();
		item.xp_value = 5;
		item.xp_in_town_only = true;
		return item;
	}

	pub fn uncut_precious_stone() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::UncutGemstone,ItemCategoryCode::Misc);
		item.description = "A strangely colorful stone catches your eye.".to_string();
		item.name = "colorful stone".to_string();
		item.effect = "This colorful rock is a precious gem, yet uncit. It will fetch a good price in town.".to_string();
		item.frequency = Mobile::skilled_task();
		item.xp_value = 10;
		item.lifetime = 10000;
		item.xp_in_town_only = true;
		return item;
	}

	pub fn useless_rock() -> Box<Item>
	{
		let mut item = Item::basic_item(ItemTypeCode::UselessRock,ItemCategoryCode::Misc);
		item.description = "A strangely colorful stone catches your eye.".to_string();
		item.name = "colorful stone".to_string();
		item.effect = "This is a glinty bit of worthless rock.".to_string();
		item.frequency = Mobile::skilled_task();
		item.xp_value = 0;
		item.lifetime = 10000;
		return item;
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

	pub fn minor_treasure() -> Option<Box<Item> >
	{
		let die = Dice { number: 1, die: 20 };
		let roll = die.roll();
		match roll
		{
			1 => { return Some(Self::gold_bauble()); },
			2 => { return Some(Self::gold_bauble()); },
			3 => { return Some(Self::bone_jewelry()); },
			4 => { return Some(Self::bone_jewelry()); },
			5 => { return Some(Self::bone_jewelry()); },
			6 => { return Some(Self::shrunken_head()); },
			7 => { return Some(Self::shrunken_head()); },
			8 => { return Some(Self::dwarf_beard()); },
			9 => { return Some(Self::dwarf_beard()); },
			10 => { return Some(Self::uncut_precious_stone()); },
			11 => { return Some(Self::useless_rock()); },
			12 => { return Some(Self::healing_potion()); },
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
