// Basic types of locations for map generation
use crate::object::Object;
use uuid::Uuid;
use crate::dice::*;

// A mobile object or creature
pub struct Mobile
{
	pub id: Uuid,
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
	// Hit points of damage sustained
	pub damage: i16,
	// Actions per tick,
	pub actions_per_tick: i16,
	// Actions used in current tick
	pub actions_used: i16,
	// Skills
	pub combat: i16,
	pub damage_dice: Dice
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
		result += &(", dmg: ".to_string()+&(self.damage.to_string())+&"/".to_string()+&(self.max_hit_points().to_string()));
		result += "\n";
		result += &("combat: ".to_string()+&(self.combat.to_string()));
		return result;
	}

	fn description(&self) -> String
	{
		return self.description.clone();
	}

	fn get_id(&self) -> Uuid
	{
		return self.id;
	}

	fn get_name(&self) -> String
	{
		return self.name.clone();
	}
}

impl Mobile
{

	pub fn use_action(&mut self) -> bool
	{
		self.actions_used += 1;
		return self.actions_used <= self.actions_per_tick;
	}

	pub fn tick(&mut self)
	{
		self.actions_used = 0;
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
				combat: 0,
				damage: 0,	
				actions_per_tick: 1,
				actions_used: 0,
				damage_dice: Dice { number: 1, die: 1 }
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
				combat: 0,
				damage: 0,
				actions_per_tick: 1,
				actions_used: 0,
				damage_dice: Dice { number: 1, die: 1 }
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
				actions_per_tick: 1,
				actions_used: 0,
				damage_dice: Dice { number: 1, die: 2 }
			});
	}

	pub fn contest(a_modifier: i16, b_modifier: i16, a_skill: &mut i16, b_skill: &mut i16) -> bool
	{
		let die = Dice { number: 1, die: 100 };
		let a_total = a_modifier+*a_skill+die.roll();
		let b_total = b_modifier+*b_skill+die.roll();
		if a_total >= b_total
		{
			if a_modifier < b_total
			{
				*a_skill += 1;
			}
			return true;
		}
		else
		{
			if b_modifier < a_total
			{
				*b_skill += 1;
			}
			return false;
		}
	}
}
