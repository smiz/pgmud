use crate::object::Object;
use crate::items::*;
use crate::dice::*;
use crate::uid;
use std::cmp::max;

// A mobile object or creature
pub struct Mobile
{
	id: usize,
	pub description: String,
	pub name: String,
	pub name_with_article: String,
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
	pub leatherwork: i16,
	pub woodcraft: i16,
	pub knowledge: i16,
	// Description of the object we are using as a weapon
	pub wielded: String,
	// Damage done by our attack
	pub damage_dice: Dice,
	// How much armor protection do we have?,
	pub armor: i16,
	// Inventory
	pub inventory: Vec<Box<Item > >,
	// Maximum miscellaneous that can be carried
	pub misc_items_slots: u8,
	// Has a weapon?
	pub is_armed: bool,
	// Wearing armor?
	pub is_armored: bool,
	// Target for knowledge rolls
	pub frequency: i16,
	// Does this mobile wander about of its own accord?
	pub wanders: bool,
	// Is this mobile aggressive?
	pub aggressive: bool
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
		result += &("leatherwork: ".to_string()+&(self.leatherwork.to_string())+"\n");
		result += &("woodcraft: ".to_string()+&(self.woodcraft.to_string())+"\n");
		result += &("knowledge: ".to_string()+&(self.knowledge.to_string())+"\n");
		result += &("misc. slots: ".to_string()+&(self.misc_items_slots).to_string()+"\n");
		result += &("armed: ".to_string()+&(self.is_armed).to_string()+"\n");
		result += &("armor protection: ".to_string()+&(self.armor).to_string()+"\n");
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

	pub fn load_from_file(&mut self) -> bool 
	{
		let mut file_name = self.name.clone()+&".csv".to_string();
		file_name.make_ascii_lowercase();
    	let result = csv::Reader::from_path(file_name);
		match result
		{
			Ok(mut reader) => 
				{
					for item in reader.records()
					{
						match item
						{
							Ok(item) => 
								{
									let key = item.get(0).unwrap();
									let value = item.get(1).unwrap();
									match key.as_ref()
									{
										"str" => { self.strength = value.parse::<i16>().unwrap(); },
										"int" => { self.intelligence = value.parse::<i16>().unwrap(); },
										"wis" => { self.wisdom = value.parse::<i16>().unwrap(); },
										"dex" => { self.dexterity = value.parse::<i16>().unwrap(); },
										"con" => { self.constitution = value.parse::<i16>().unwrap(); },
										"chr" => { self.charisma = value.parse::<i16>().unwrap(); },
										"xp" => { self.xp = value.parse::<i16>().unwrap(); },
										"hp" => { self.max_damage = value.parse::<i16>().unwrap(); },
										"combat" => { self.combat = value.parse::<i16>().unwrap(); },
										"steal" => { self.steal = value.parse::<i16>().unwrap(); },
										"knowledge" => { self.knowledge = value.parse::<i16>().unwrap(); },
										"perception" => { self.perception = value.parse::<i16>().unwrap(); },
										"leatherwork" => { self.leatherwork = value.parse::<i16>().unwrap(); },
										"woodcraft" => { self.woodcraft = value.parse::<i16>().unwrap(); },
										"id" => { self.id = value.parse::<usize>().unwrap(); },
										_ => { () }
									}
								},
							_ => { return false; }
						}
					}
				},
			_ => { return false; }
		}
		return true;
	}

	pub fn save_to_file(&self)
	{
		let mut file_name = self.name.clone()+&".csv".to_string();
		file_name.make_ascii_lowercase();
		let mut wtr = csv::Writer::from_path(file_name).unwrap();
		let _ = wtr.write_record(&["name",&self.name]).unwrap();
		let _ = wtr.write_record(&["id",&self.id.to_string()]).unwrap();
		let _ = wtr.write_record(&["str",&self.strength.to_string()]).unwrap();
		let _ = wtr.write_record(&["dex",&self.dexterity.to_string()]).unwrap();
		let _ = wtr.write_record(&["con",&self.constitution.to_string()]).unwrap();
		let _ = wtr.write_record(&["chr",&self.charisma.to_string()]).unwrap();
		let _ = wtr.write_record(&["int",&self.intelligence.to_string()]).unwrap();
		let _ = wtr.write_record(&["wis",&self.wisdom.to_string()]).unwrap();
		let _ = wtr.write_record(&["xp",&self.xp.to_string()]).unwrap();
		let _ = wtr.write_record(&["dmg",&self.damage.to_string()]).unwrap();
		let _ = wtr.write_record(&["hp",&self.max_hit_points().to_string()]).unwrap();
		let _ = wtr.write_record(&["combat",&self.combat.to_string()]).unwrap();
		let _ = wtr.write_record(&["steal",&self.steal.to_string()]).unwrap();
		let _ = wtr.write_record(&["perception",&self.perception.to_string()]).unwrap();
		let _ = wtr.write_record(&["leatherwork",&self.leatherwork.to_string()]).unwrap();
		let _ = wtr.write_record(&["woodcraft",&self.woodcraft.to_string()]).unwrap();
		let _ = wtr.write_record(&["knowledge",&self.knowledge.to_string()]).unwrap();
		let _ = wtr.flush().unwrap();
	}

	pub fn do_damage(&mut self, damage: i16) -> i16
	{
		let mut damage_applied = damage;
		// Damage that penetrates the armor reduces its integrity
		if damage > self.armor
		{
			damage_applied -= self.armor;
			if self.armor > 0
			{
				self.armor -= 1;
			}
		}
		// Damage that does not penetrate the armor is stopped completely
		else
		{
			damage_applied = 0;
		}
		self.damage += damage_applied;
		return damage_applied;
	}

	pub fn is_killed(&self)
	{
		let mut old_file_name = self.name.clone()+&".csv".to_string();
		old_file_name.make_ascii_lowercase();
		let mut new_file_name = self.name.clone()+&".dead".to_string();
		new_file_name.make_ascii_lowercase();
		let _ignore_result = std::fs::rename(old_file_name,new_file_name);
	}

	fn xp_cost(&self, skill_level: i16) -> i16
	{
		let cost = skill_level+1;
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

	pub fn practice_woodcraft(&mut self) -> bool
	{
		let cost = self.xp_cost(self.woodcraft);
		if cost > 0
		{
			self.xp -= cost;
			self.woodcraft += 1;
			return true;
		}
		return false;
	}

	pub fn practice_leatherwork(&mut self) -> bool
	{
		let cost = self.xp_cost(self.leatherwork);
		if cost > 0
		{
			self.xp -= cost;
			self.leatherwork += 1;
			return true;
		}
		return false;
	}

	pub fn practice_knowledge(&mut self) -> bool
	{
		let cost = self.xp_cost(self.knowledge);
		if cost > 0
		{
			self.xp -= cost;
			self.knowledge += 1;
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
			result += &item.get_name();
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

	pub fn roll_knowledge(&self) -> i16
	{
		return self.roll_skill(self.intelligence,self.knowledge);
	}

	pub fn roll_perception(&self) -> i16
	{
		return self.roll_skill(self.wisdom,self.perception);
	}

	pub fn roll_leatherwork(&self) -> i16
	{
		return self.roll_skill(self.intelligence,self.leatherwork);
	}

	pub fn roll_leatherwork_or_woodcraft(&self) -> i16
	{
		return self.roll_skill(self.intelligence,max(self.leatherwork,self.woodcraft));
	}

	pub fn roll_woodcraft(&self) -> i16
	{
		return self.roll_skill(self.intelligence,self.woodcraft);
	}

	pub fn get_id(&self) -> usize
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

	pub fn fetch_first_item(&mut self) -> Option<Box<Item> >
	{
		return self.fetch_item_by_position(0);
	}

	pub fn fetch_item_by_position(&mut self, pos: usize) -> Option<Box<Item> >
	{
		if pos >= self.inventory.len()
		{
			return None;
		}
		let mut item = self.inventory.remove(pos);
		let slot_code = item.category_code.clone();
		item.drop_item(self);
		{
			match slot_code
			{
				ItemCategoryCode::Misc => { self.misc_items_slots += 1; },
				ItemCategoryCode::Weapon => { self.is_armed = false; self.unwield(); },
				ItemCategoryCode::Armor => { item.armor_value = self.armor; self.armor = 0; self.is_armored = false; },
			}
		}
		return Some(item);
	}

	pub fn eat_item_by_name(&mut self, key: &String) -> String
	{
		let item = self.fetch_item_by_name(key);
		match item
		{
			Some(mut item) =>
				{
					if item.eat(self)
					{
						return "You eat the ".to_owned()+&item.get_name();
					}
					else
					{
						self.add_item(item,false);
						return "You cannot eat that!".to_string();
					}
				},
			None => { return "Eat what?".to_string(); }
		}	
	}

	pub fn fetch_item_by_name(&mut self, key: &String) -> Option<Box<Item> >
	{
		let mut i = 0;
		let mut lower_case_key = key.clone();
		lower_case_key.make_ascii_lowercase();
		while i < self.inventory.len()
		{
			let mut name = self.inventory[i].name.clone();
			name.make_ascii_lowercase();
    		if name.contains(key)
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
			ItemCategoryCode::Weapon => { return !self.is_armed; },
			ItemCategoryCode::Armor => { return !self.is_armored; },
		}
	}

	pub fn add_item(&mut self, mut item: Box<Item>, take_xp: bool)
	{
		let slot_code = item.category_code.clone();
		item.got_item(self,take_xp);
		{
			match slot_code
			{
				ItemCategoryCode::Misc => { self.misc_items_slots -= 1; },
				ItemCategoryCode::Weapon => { self.is_armed = true; },
				ItemCategoryCode::Armor => { self.is_armored = true; self.armor = item.armor_value; },
			}
		}
		self.inventory.push(item);
	}

	pub fn is_active(&self) -> bool
	{
		return self.wanders || self.aggressive;
	}

	/// Construct mobile with default values
	fn new(name: &String) -> Box<Mobile>
	{
		let article = "a".to_string();
		return Box::new(
			Mobile
			{
				id: uid::new(),
				name: name.clone(),
				name_with_article: article.clone()+" "+&name,
				description: "You see ".to_string()+&article+" "+name+".",
				arrive_prefix: article.clone()+" "+name+" arrives",
				leave_prefix: article.clone()+" "+name+" leaves",
				strength: 10,
				dexterity: 10,
				constitution: 10,
				max_damage: 10,
				intelligence: 10,
				wisdom: 10,
				charisma: 10,
				luck: 0,
				xp: 0,
				combat: 0,
				steal: 0,
				perception: 0,
				leatherwork: 0,
				woodcraft: 0,
				knowledge: 0,
				damage: 0,	
				actions_per_tick: 1,
				actions_used: 0,
				wielded: "fist".to_string(),
				damage_dice: Dice { number: 1, die: 2 },
				inventory: Vec::new(),
				misc_items_slots: 10,
				is_armed: false,
				is_armored: false,
				frequency: 10000,
				armor: 0,
				wanders: false,
				aggressive: false
			});
	}

	pub fn new_character(name: &String) -> Box<Mobile>
	{
		let mut character = Mobile::new(name);
		let die = Dice { number: 3, die: 6 };
		character.name = name.clone();
		character.name_with_article = name.clone();
		character.description = name.clone()+" is here.";
		character.arrive_prefix = name.clone()+" arrives";
		character.leave_prefix = name.clone()+" leaves";
		character.strength = die.roll();
		character.dexterity = die.roll();
		character.constitution = die.roll();
		character.max_damage = character.constitution;
		character.intelligence = die.roll();
		character.wisdom = die.roll();
		character.charisma = die.roll();
		return character;
	}

	pub fn small_woodland_creature() -> Box<Mobile>
	{
		let die = Dice { number: 1, die: 4 };
		let mut mobile = Mobile::new(&"woodland creature".to_string());
		mobile.description = "A small woodland creature plays happily in the forest.".to_string();
		mobile.arrive_prefix = "A woodland creature scurries in".to_string();
		mobile.leave_prefix = "A woodland creature scurries away".to_string();
		mobile.strength = die.roll();
		mobile.dexterity = 18;
		mobile.constitution = 18;
		mobile.max_damage = die.roll();
		mobile.intelligence = die.roll();
		mobile.wisdom = 1;
		mobile.charisma = die.roll();
		mobile.luck = die.roll();
		mobile.perception = 10+die.roll();
		mobile.wielded = "bite".to_string();
		mobile.damage_dice = Dice { number: 1, die: 2 };
		mobile.is_armed = true;
		mobile.frequency = 50;
		let treasure = Item::woodland_trinket();
		if treasure.is_some()
		{	
			mobile.add_item(treasure.unwrap(),false); 
		}	
		return mobile;
	}

	pub fn rodent() -> Box<Mobile>
	{
		let mut mobile = Mobile::new(&"rodent".to_string());
		mobile.description = "A small rodent watches you keenly.".to_string();
		mobile.arrive_prefix = "A small rodent scurries in".to_string();
		mobile.leave_prefix = "A small rodent scurries away".to_string();
		mobile.strength = 1;
		mobile.dexterity = 18;
		mobile.constitution = 18;
		mobile.max_damage = 3;
		mobile.intelligence = 2;
		mobile.wisdom = 1;
		mobile.charisma = 3;
		mobile.perception = 10;
		mobile.wielded = "bite".to_string();
		mobile.damage_dice = Dice { number: 1, die: 1 };
		mobile.is_armed = true;
		mobile.frequency = 50;
		mobile.add_item(Item::healthy_nuts_and_seeds(),false);
		return mobile;
	}

	pub fn rabbit() -> Box<Mobile>
	{
		let mut mobile = Mobile::new(&"rabbit".to_string());
		mobile.description = "A rabbit watches you carefully.".to_string();
		mobile.arrive_prefix = "A rabbit hops".to_string();
		mobile.leave_prefix = "A rabbit hops in from".to_string();
		mobile.strength = 1;
		mobile.dexterity = 18;
		mobile.constitution = 3;
		mobile.max_damage = 3;
		mobile.intelligence = 2;
		mobile.wisdom = 1;
		mobile.charisma = 3;
		mobile.luck = 1;
		mobile.perception = 10;
		mobile.wielded = "bite".to_string();
		mobile.damage_dice = Dice { number: 1, die: 1 };
		mobile.is_armed = true;
		mobile.frequency = 50;
		mobile.add_item(Item::rabbit_foot(),false);
		return mobile;
	}

	pub fn beggar() -> Box<Mobile>	
	{
		let mut mobile = Mobile::new(&"beggar".to_string());
		mobile.description = "A beggar is asking for alms.".to_string();
		mobile.luck = -10;
		mobile.perception = 1;
		mobile.wielded = "fist".to_string();
		mobile.frequency = 50;
		mobile.add_item(Item::green_penny(),false);
		return mobile;
	}

	pub fn goblin() -> Box<Mobile>	
	{
		let mut mobile = Mobile::new(&"goblin".to_string());
		mobile.description = "A goblin is here, looking menacing.".to_string();
		mobile.wisdom = 3;
		mobile.perception = 1;
		mobile.frequency = 100;
		mobile.wanders = true;
		mobile.aggressive = true;
		let weapon = Item::pointed_stick();
		mobile.add_item(weapon,false);
		let treasure = Item::minor_treasure();
		if treasure.is_some()
		{
			mobile.add_item(treasure.unwrap(),false);
		}
		return mobile;
	}

	pub fn head_hunter() -> Box<Mobile>	
	{
		let mut mobile = Mobile::new(&"head hunter".to_string());
		mobile.description = "A savage head hunter is looking for trophies.".to_string();
		mobile.perception = 1;
		mobile.frequency = 100;
		mobile.wanders = true;
		mobile.aggressive = true;
		let weapon = Item::stone_knife();
		mobile.add_item(weapon,false);
		let item = Item::shrunken_head();
		mobile.add_item(item,false);
		return mobile;
	}

	pub fn foppish_dandy() -> Box<Mobile>	
	{
		let mut mobile = Mobile::new(&"foppish dandy".to_string());
		mobile.name_with_article = "a dandy".to_string();
		mobile.description = "A foppish dandy looks at you disdainfully.".to_string();
		mobile.wisdom = 8;
		mobile.charisma = 13;
		mobile.frequency = 50;
		let weapon = Item::sword();
		mobile.add_item(weapon,false);
		return mobile;
	}

	pub fn lumber_jack() -> Box<Mobile>	
	{
		let mut mobile = Mobile::new_character(&"lumberjack".to_string());
		mobile.name_with_article = "the ".to_string()+&mobile.name;
		mobile.description = "A strong lumberjack is looking for a tall tree.".to_string();
		mobile.arrive_prefix = "A lumberjack strides in from".to_string();
		mobile.leave_prefix = "A lumberjack strides away".to_string();
		mobile.strength = 16;
		mobile.wanders = true;
		let weapon = Item::axe();
		mobile.add_item(weapon,false);
		return mobile;
	}

	// Standard difficulty levels
	pub fn trivial_task() -> i16 { return 5*5; }
	pub fn easy_task() -> i16 { return 5*10; }
	pub fn routine_task() -> i16 { return 5*15; }
	pub fn skilled_task() -> i16 { return 5*20; }
	pub fn very_skilled_task() -> i16 { return 5*25; }
	pub fn heroic_task() -> i16 { return 5*30; }

}

#[cfg(test)]
mod mobile_unit_test
{
	use super::*;
	use crate::items::*;

	#[test]
	fn damage_test()
	{
		let mut mobile = Mobile::new(&"goober".to_string());
		mobile.armor = 2;
		mobile.damage = 0;
		mobile.do_damage(1);
		assert_eq!(mobile.damage,0);
		mobile.do_damage(2);
		assert_eq!(mobile.damage,0);
		mobile.do_damage(3);
		assert_eq!(mobile.damage,1);
		assert_eq!(mobile.armor,1);
		mobile.do_damage(1);
		assert_eq!(mobile.damage,1);
		assert_eq!(mobile.armor,1);
		mobile.do_damage(2);
		assert_eq!(mobile.damage,2);
		assert_eq!(mobile.armor,0);
		mobile.do_damage(1);
		assert_eq!(mobile.damage,3);
		assert_eq!(mobile.armor,0);
	}

	#[test]
	fn test_fetch_first_item()
	{
		let mut mobile = Mobile::new_character(&"Jim".to_string());
		let result1 = mobile.fetch_first_item();
		assert!(result1.is_none());
	}

	#[test]
	fn test_slots()
	{
		let mut foot = Item::rabbit_foot();
		let mut mobile = Mobile::new_character(&"Jim".to_string());
		while mobile.has_room_for_item(&foot)
		{
			mobile.add_item(foot,true);
			foot = Item::rabbit_foot();
		}
		mobile.fetch_item_by_position(0);
		assert!(mobile.has_room_for_item(&foot));
	}

	#[test]
	fn save_load_mobile()
	{
		let c1 = Mobile::new_character(&"Test".to_string());
		c1.save_to_file();
		let mut c2 = Mobile::new_character(&"Test".to_string());
		assert!(c2.load_from_file());
		assert_eq!(c1.strength,c2.strength);
		assert_eq!(c1.intelligence,c2.intelligence);
		assert_eq!(c1.wisdom,c2.wisdom);
		assert_eq!(c1.constitution,c2.constitution);
		assert_eq!(c1.charisma,c2.charisma);
		assert_eq!(c1.dexterity,c2.dexterity);
		assert_eq!(c1.combat,c2.combat);
		assert_eq!(c1.steal,c2.steal);
		assert_eq!(c1.knowledge,c2.knowledge);
		assert_eq!(c1.perception,c2.perception);
		assert_eq!(c1.leatherwork,c2.leatherwork);
		assert_eq!(c1.woodcraft,c2.woodcraft);
		let mut c3 = Mobile::new_character(&"Lord Tom".to_string());
		assert!(!c3.load_from_file());
	}
}
