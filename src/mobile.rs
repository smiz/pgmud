use crate::object::Object;
use crate::items::*;
use uuid::Uuid;
use crate::dice::*;

// A mobile object or creature
pub struct Mobile
{
	id: Uuid,
	pub description: String,
	pub name: String,
	// Arrival signified by message {arrive_prefix} from the {direction}
	pub arrive_prefix: String,
	// Leaving signified by message {leave_prefix} to the {direction}
	pub leave_prefix: String,
	// Attributes
	pub strength: i16,
	pub dexterity: i16,
	pub constitution: i16,
	pub intelligence: i16,
	pub wisdom: i16,
	pub charisma: i16,
	pub luck: i16,
	pub xp: i16,
	// Hit points of damage sustained
	pub damage: i16,
	pub max_damage: i16,
	// Actions per tick,
	pub actions_per_tick: i16,
	// Actions used in current tick
	pub actions_used: i16,
	// Skills
	pub combat: i16,
	pub steal: i16,
	pub perception: i16,
	// Description of the object we are using as a weapon
	pub wielded: String,
	// Damage done by our attack
	pub damage_dice: Dice,
	// Inventory
	inventory: Vec<Box<Item > >,
	// Maximum miscellaneous that can be carried
	pub misc_items_slots: u8,
	pub weapon_slots: u8
}

impl Object for Mobile
{
	fn complete_description(&self) -> String
	{
		let mut result = self.description().clone();
		result += "\n";
		result += &("str: ".to_string()+&(self.strength.to_string()));
		result += &(", dex: ".to_string()+&(self.dexterity.to_string()));
		result += &(", con: ".to_string()+&(self.constitution.to_string()));
		result += &(", int: ".to_string()+&(self.intelligence.to_string()));
		result += &(", wis: ".to_string()+&(self.wisdom.to_string()));
		result += &(", chr: ".to_string()+&(self.charisma.to_string()));
		result += &(", luck: ".to_string()+&(self.luck.to_string()));
		result += &(", xp: ".to_string()+&(self.xp.to_string()));
		result += &(", dmg: ".to_string()+&(self.damage.to_string())+&"/".to_string()+&(self.max_hit_points().to_string()));
		result += "\n";
		result += &("combat: ".to_string()+&(self.combat.to_string())+"\n");
		result += &("steal: ".to_string()+&(self.steal.to_string())+"\n");
		result += &("perception: ".to_string()+&(self.perception.to_string())+"\n");
		return result;
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

impl Mobile
{

	fn xp_cost(&self, skill_level: i16) -> i16
	{
		let cost = 2*(skill_level+1);
		if self.xp >= cost
		{
			return cost;
		}
		return 0;
	}

	pub fn practice_combat(&mut self) -> bool
	{
		let cost = self.xp_cost(self.combat);
		if cost > 0
		{
			self.xp -= cost;
			self.combat += 1;
			return true;
		}
		return false;
	}

	pub fn practice_perception(&mut self) -> bool
	{
		let cost = self.xp_cost(self.perception);
		if cost > 0
		{
			self.xp -= cost;
			self.perception += 1;
			return true;
		}
		return false;
	}

	pub fn practice_steal(&mut self) -> bool
	{
		let cost = self.xp_cost(self.steal);
		if cost > 0
		{
			self.xp -= cost;
			self.steal += 1;
			return true;
		}
		return false;
	}

	pub fn unwield(&mut self)
	{
		self.wielded = "fist".to_string();
		self.damage_dice = Dice { number: 1, die: 2};
	}

	pub fn list_inventory(&self) -> String
	{
		let mut result = String::new();
		for item in self.inventory.iter()
		{
			result += &item.description();
			result += "\n";
		}
		return result;
	}

	fn attribute_modifier(attribute: i16) -> i16
	{
		if attribute <= 1 { return -5; }
		if attribute <= 3 { return -4; }
		if attribute <= 5 { return -3; }
		if attribute <= 7 { return -2; }
		if attribute <= 9 { return -1; }
		if attribute <= 11 { return 0; }
		if attribute <= 13 { return 1; }
		if attribute <= 15 { return 2; }
		if attribute <= 17 { return 3; }
		if attribute <= 19 { return 4; }
		if attribute <= 21 { return 5; }
		if attribute <= 23 { return 6; }
		if attribute <= 25 { return 7; }
		if attribute <= 27 { return 8; }
		if attribute <= 29 { return 9; }
		return 10;
	}

	fn roll_skill(&self, attribute: i16, skill: i16) -> i16
	{
		let die = Dice { number: 1, die: 100 };
		return 5*(Mobile::attribute_modifier(attribute)+skill)+self.luck+die.roll();
	}

	pub fn roll_combat(&self) -> i16
	{
		return self.roll_skill(self.strength,self.combat);
	}

	pub fn roll_steal(&self) -> i16
	{
		return self.roll_skill(self.dexterity,self.steal);
	}

	pub fn roll_perception(&self) -> i16
	{
		return self.roll_skill(self.wisdom,self.perception);
	}

	pub fn get_id(&self) -> Uuid
	{
		return self.id;
	}

	pub fn use_action(&mut self) -> bool
	{
		self.actions_used += 1;
		return self.actions_used <= self.actions_per_tick;
	}

	pub fn tick(&mut self)
	{
		let die = Dice { number: 1, die: 100 };
		self.actions_used = 0;
		if self.damage > 0 && die.roll() <= self.constitution
		{
			self.damage -= 1;
		}
	}

	pub fn max_hit_points(&self) -> i16
	{
		return self.max_damage;
	}

	pub fn fetch_random_item(&mut self) -> Option<Box<Item> >
	{
		if self.inventory.is_empty()
		{
			return None;
		}
		let index = (rand::random::<usize>()) % self.inventory.len();
		return self.fetch_item_by_position(index);
	}

	pub fn fetch_last_item(&mut self) -> Option<Box<Item> >
	{
		let item_ptr = self.inventory.pop();
		match item_ptr
		{
			Some(ref item) => { item.drop_item(self); return item_ptr; }
			_ => { return None; }
		}
	}

	pub fn fetch_item_by_position(&mut self, pos: usize) -> Option<Box<Item> >
	{
		if pos >= self.inventory.len()
		{
			return None;
		}
		let item = self.inventory.remove(pos);
		let slot_code = item.category_code.clone();
		item.drop_item(self);
		match slot_code
		{
			ItemCategoryCode::Misc => { self.misc_items_slots += 1; return Some(item); },
			ItemCategoryCode::Weapon => { self.weapon_slots += 1; return Some(item); },
		}
	}

	pub fn fetch_item_by_name(&mut self, key: String) -> Option<Box<Item> >
	{
		let mut i = 0;
		let mut lower_case_key = key.clone();
		lower_case_key.make_ascii_lowercase();
		while i < self.inventory.len()
		{
			let mut name = self.inventory[i].name.clone();
			name.make_ascii_lowercase();
    		if name.contains(&key)
			{
				return self.fetch_item_by_position(i);
			}
			i += 1;
		}
		return None;
	}

	pub fn has_room_for_item(&mut self, item: &Box<Item>) -> bool
	{
		match item.category_code
		{
			ItemCategoryCode::Misc => { return self.misc_items_slots > 0; },
			ItemCategoryCode::Weapon => { return self.weapon_slots > 0; },
		}
	}

	pub fn add_item(&mut self, mut item: Box<Item>, take_xp: bool)
	{
		let slot_code = item.category_code.clone();
		item.got_item(self,take_xp);
		self.inventory.push(item);
		match slot_code
		{
			ItemCategoryCode::Misc => { self.misc_items_slots -= 1; },
			ItemCategoryCode::Weapon => { self.weapon_slots -= 1; },
		}
	}

	pub fn new_character(name: String) -> Box<Mobile>
	{
		let die = Dice { number: 3, die: 6 };
		let constitution = die.roll();
		return Box::new(
			Mobile
			{
				id: Uuid::new_v4(),
				name: name.clone(),
				description: name.clone()+" looks relaxed.",
				arrive_prefix: name.clone()+" arrives",
				leave_prefix: name+" leaves",
				strength: die.roll(),
				dexterity: die.roll(),
				constitution: constitution,
				max_damage: constitution,
				intelligence: die.roll(),
				wisdom: die.roll(),
				charisma: die.roll(),
				luck: 0,
				xp: 0,
				combat: 0,
				steal: 0,
				perception: 0,
				damage: 0,	
				actions_per_tick: 1,
				actions_used: 0,
				wielded: "fist".to_string(),
				damage_dice: Dice { number: 1, die: 2 },
				inventory: Vec::new(),
				misc_items_slots: 10,
				weapon_slots: 1
			});
	}

	pub fn rodent() -> Box<Mobile>
	{
		let mut mobile = Box::new(
			Mobile {
				id: Uuid::new_v4(),
				name: "rodent".to_string(),
				description: "A small rodent watches you keenly.".to_string(),
				arrive_prefix: "A small rodent scurries in".to_string(),
				leave_prefix: "A small rodent scurries away".to_string(),
				strength: 1,
				dexterity: 18,
				constitution: 3,
				max_damage: 3,
				intelligence: 2,
				wisdom: 1,
				charisma: 3,
				luck: 0,
				xp: 0,
				combat: 0,
				damage: 0,
				steal: 0,
				perception: 10,
				actions_per_tick: 1,
				actions_used: 0,
				wielded: "bite".to_string(),
				damage_dice: Dice { number: 1, die: 1 },
				inventory: Vec::new(),
				misc_items_slots: 1,
				weapon_slots: 0
			});
		mobile.add_item(Item::healthy_nuts_and_seeds(),false);
		return mobile;
	}
	pub fn rabbit() -> Box<Mobile>
	{
		let mut mobile = Box::new(
			Mobile {
				id: Uuid::new_v4(),
				name: "rabbit".to_string(),
				description: "A rabbit watches you carefully.".to_string(),
				arrive_prefix: "A rabbit hops ".to_string(),
				leave_prefix: "A rabbit hops in from the".to_string(),
				strength: 1,
				dexterity: 18,
				constitution: 3,
				max_damage: 3,
				intelligence: 2,
				wisdom: 1,
				charisma: 3,
				luck: 1,
				xp: 0,
				combat: 0,
				damage: 0,
				steal: 0,
				perception: 10,
				actions_per_tick: 1,
				actions_used: 0,
				wielded: "bite".to_string(),
				damage_dice: Dice { number: 1, die: 1 },
				inventory: Vec::new(),
				misc_items_slots: 1,
				weapon_slots: 0
			});
		mobile.add_item(Item::rabbit_foot(),false);
		return mobile;
	}

	pub fn beggar() -> Box<Mobile>	
	{
		let mut mobile = Box::new(
			Mobile {
				id: Uuid::new_v4(),
				name: "beggar".to_string(),
				description: "A beggar is asking for alms.".to_string(),
				arrive_prefix: "A beggar arrives".to_string(),
				leave_prefix: "A beggar leaves".to_string(),
				strength: 5,
				dexterity: 10,
				constitution: 10,
				max_damage: 10,
				intelligence: 10,
				wisdom: 3,
				charisma: 10,
				combat: 0,
				damage: 0,
				luck: 0,
				xp: 0,
				steal: 0,
				perception: 1,
				actions_per_tick: 1,
				actions_used: 0,
				wielded: "fist".to_string(),
				damage_dice: Dice { number: 1, die: 2 },
				inventory: Vec::new(),
				misc_items_slots: 1,
				weapon_slots: 1
			});
		mobile.add_item(Item::green_penny(),false);
		return mobile;
	}

	pub fn foppish_dandy() -> Box<Mobile>	
	{
		let mut mobile = Box::new(
			Mobile {
				id: Uuid::new_v4(),
				name: "foppish dandy".to_string(),
				description: "A foppish dandy looks at you disdainfully.".to_string(),
				arrive_prefix: "A foppish dandy arrives".to_string(),
				leave_prefix: "A foppish dandy leaves".to_string(),
				strength: 10,
				dexterity: 10,
				constitution: 10,
				max_damage: 10,
				intelligence: 10,
				wisdom: 8,
				charisma: 13,
				combat: 0,
				damage: 0,
				luck: 0,
				xp: 0,
				steal: 0,
				perception: 0,
				actions_per_tick: 1,
				actions_used: 0,
				wielded: "fist".to_string(),
				damage_dice: Dice { number: 1, die: 2 },
				inventory: Vec::new(),
				misc_items_slots: 1,
				weapon_slots: 1
			});
		let weapon = Item::sword();
		mobile.add_item(weapon,false);
		return mobile;
	}
}

#[cfg(test)]
mod mobile_unit_test
{
	use super::*;
	use crate::items::*;

	#[test]
	fn test_slots()
	{
		let mut foot = Item::rabbit_foot();
		let mut mobile = Mobile::new_character("Jim".to_string());
		while mobile.has_room_for_item(&foot)
		{
			mobile.add_item(foot,true);
			foot = Item::rabbit_foot();
		}
		mobile.fetch_item_by_position(0);
		assert!(mobile.has_room_for_item(&foot));
	}
}
