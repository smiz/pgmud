// Generate unique identifiers
use uuid::Uuid;

// Interface for all types of objects
pub trait Object
{
	// Get the description of the object from nearby
	fn description(&self) -> String;
	fn get_id(&self) -> Uuid;
	fn get_name(&self) -> String;
	fn complete_description(&self) -> String;
}

