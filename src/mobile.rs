use crate::object::Object;
use crate::items::Item;
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
	// Actions per tick,
	pub actions_per_tick: i16,
	// Actions used in current tick
	pub actions_used: i16,
	// Skills
	pub combat: i16,
	pub damage_dice: Dice,
	// Inventory
	pub inventory: Vec<Box<Item > >
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
		result += &("combat: ".to_string()+&(self.combat.to_string()));
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
	fn list_inventory(&self) -> String
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
		return self.constitution;
	}

	pub fn new_character(name: String) -> Box<Mobile>
	{
		let die = Dice { number: 3, die: 6 };
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
				constitution: die.roll(),
				intelligence: die.roll(),
				wisdom: die.roll(),
				charisma: die.roll(),
				luck: 0,
				xp: 0,
				combat: 0,
				damage: 0,	
				actions_per_tick: 1,
				actions_used: 0,
				damage_dice: Dice { number: 1, die: 2 },
				inventory: Vec::new()
			});
	}

	pub fn rodent() -> Box<Mobile>
	{
		return Box::new(
			Mobile {
				id: Uuid::new_v4(),
				name: "rodent".to_string(),
				description: "A small rodent watches you keenly.".to_string(),
				arrive_prefix: "A small rodent scurries in".to_string(),
				leave_prefix: "A small rodent scurries away".to_string(),
				strength: 1,
				dexterity: 18,
				constitution: 3,
				intelligence: 2,
				wisdom: 1,
				charisma: 3,
				luck: 0,
				xp: 0,
				combat: 0,
				damage: 0,
				actions_per_tick: 1,
				actions_used: 0,
				damage_dice: Dice { number: 1, die: 1 },
				inventory: Vec::new()
			});
	}
	pub fn rabbit() -> Box<Mobile>
	{
		return Box::new(
			Mobile {
				id: Uuid::new_v4(),
				name: "rabbit".to_string(),
				description: "A rabbit watches you carefully.".to_string(),
				arrive_prefix: "A rabbit hops ".to_string(),
				leave_prefix: "A rabbit hops in from the".to_string(),
				strength: 1,
				dexterity: 18,
				constitution: 3,
				intelligence: 2,
				wisdom: 1,
				charisma: 3,
				luck: 1,
				xp: 0,
				combat: 0,
				damage: 0,
				actions_per_tick: 1,
				actions_used: 0,
				damage_dice: Dice { number: 1, die: 1 },
				inventory: vec![ Item::rabbit_foot() ]
			});
	}

	pub fn beggar() -> Box<Mobile>	
	{
		return Box::new(
			Mobile {
				id: Uuid::new_v4(),
				name: "beggar".to_string(),
				description: "A beggar is asking for alms.".to_string(),
				arrive_prefix: "A beggar arrives".to_string(),
				leave_prefix: "A beggar leaves".to_string(),
				strength: 5,
				dexterity: 10,
				constitution: 10,
				intelligence: 10,
				wisdom: 3,
				charisma: 10,
				combat: 0,
				damage: 0,
				luck: 0,
				xp: 0,
				actions_per_tick: 1,
				actions_used: 0,
				damage_dice: Dice { number: 1, die: 2 },
				inventory: Vec::new()
			});
	}
}
