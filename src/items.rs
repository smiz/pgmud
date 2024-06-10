use crate::object::Object;

#[derive(Copy,Clone)]
pub enum ItemTypeCode
{
	RABBIT_FOOT,
	FOREST_DEBRIS,
}

pub struct Item
{
	pub description: String,
	pub name: String,
	pub type_code: ItemTypeCode,
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
	pub fn rabbit_foot() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "A soft rabbits foot is here.".to_string(),
				name: "rabbit foot".to_string(),
				type_code: ItemTypeCode::RABBIT_FOOT,
				xp_value: 1
			});
	}
	pub fn forest_debris() -> Box<Item>
	{
		return Box::new(
			Item
			{
				description: "Some twigs and leaves litter the forest floor.".to_string(),
				name: "leaves and twigs".to_string(),
				type_code: ItemTypeCode::FOREST_DEBRIS,
				xp_value: 1
			});
	}
}
