use std::collections::LinkedList;
use std::time::{SystemTime};
use uuid::Uuid;

struct Message
{
	x: i16,
	y: i16,
	uuid: Option<Uuid>,
	uuid_is_target: bool,
	message: String,
	posted_time: SystemTime
}

pub struct MessageList
{
	msg_list: LinkedList<Message>
}

impl MessageList
{
	pub fn new() -> MessageList
	{
		return MessageList { msg_list: LinkedList::new() };
	}

	pub fn broadcast(&mut self, msg: String, x: i16, y: i16)
	{
		let global_msg = Message {
			x: x,
			y: y,
			uuid: None,
			uuid_is_target: false,
			message: msg,
			posted_time: SystemTime::now()
		};
		self.msg_list.push_back(global_msg);
	}

	fn cleanup_old_messages(&mut self)
	{
		while !self.msg_list.is_empty()
		{
			let msg = self.msg_list.front().unwrap();
			let elapsed = msg.posted_time.elapsed().unwrap();
			if elapsed.as_secs() > 1
			{
				self.msg_list.pop_front();
			}
			else
			{
				break;
			}
		} 
	}

	pub fn post_for_all(&mut self, msg: String, x: i16, y: i16)
	{
		// Insert new message
		let global_msg = Message {
			x: x,
			y: y,
			uuid: None,
			uuid_is_target: false,
			message: msg,
			posted_time: SystemTime::now()
		};
		self.msg_list.push_back(global_msg);
		self.cleanup_old_messages();
	}

	pub fn post_no_echo(&mut self, msg: String, x: i16, y: i16, origin: Uuid)
	{
		// Insert new message
		let global_msg = Message {
			x: x,
			y: y,
			uuid: Some(origin),
			uuid_is_target: false,
			message: msg,
			posted_time: SystemTime::now()
		};
		self.msg_list.push_back(global_msg);
		self.cleanup_old_messages();
	}

	pub fn post_for_target(&mut self, msg: String, target: Uuid)
	{
		// Insert new message
		let global_msg = Message {
			x: 0,
			y: 0,
			uuid: Some(target),
			uuid_is_target: true,
			message: msg,
			posted_time: SystemTime::now()
		};
		self.msg_list.push_back(global_msg);
		self.cleanup_old_messages();
	}

	pub fn read_targetted(&mut self, reader: Uuid, after: SystemTime) -> String
	{
		let mut result = String::new();
		// Build the message
		for element in self.msg_list.iter()
		{
			if element.posted_time >= after && element.uuid_is_target && element.uuid.unwrap() == reader
			{
				result += &element.message; result += "\n";
			}
		}
		self.cleanup_old_messages();
		return result;
	}

	pub fn read(&mut self, x: i16, y: i16, reader: Uuid, after: SystemTime) -> String
	{
		let mut result = String::new();
		// Build the message
		for element in self.msg_list.iter()
		{
			if element.posted_time >= after
			{
				if element.uuid_is_target && element.uuid.unwrap() == reader
				{
					result += &element.message; result += "\n";
				}
				else if !element.uuid_is_target && element.x == x && element.y == y
				{
					match element.uuid
					{
						Some(uuid) =>
						{
							if uuid != reader
							{
								result += &element.message; result += "\n";
							}
						},
						None => { result += &element.message; result += "\n"; }
					}
				}
			}
		}
		self.cleanup_old_messages();
		return result;
	}
}

#[cfg(test)]
mod messages_unit_test
{
	use std::time::{SystemTime};
	use super::*;
	use uuid::Uuid;
	use std::io::{self,Write};

	#[test]
	fn broadcast_test()
	{
		let now = SystemTime::now();
		let uuid = Uuid::new_v4();
		let mut msg_list = MessageList::new();
		msg_list.broadcast("test!".to_string(),0,0);
		let mut result = msg_list.read(0,0,uuid,now);
		print!("{}",result);
		io::stdout().flush().unwrap();
		assert_eq!(result,"test!\n".to_string());
		result = msg_list.read(1,0,uuid,now);
		assert!(result.is_empty());
	}

	#[test]
	fn target_test()
	{
		let now = SystemTime::now();
		let uuid = Uuid::new_v4();
		let mut msg_list = MessageList::new();
		msg_list.post_for_target("test!".to_string(),uuid);
		let mut result = msg_list.read(0,0,uuid,now);
		print!("{}",result);
		io::stdout().flush().unwrap();
		assert_eq!(result,"test!\n".to_string());
		result = msg_list.read(0,0,Uuid::new_v4(),now);
		assert!(result.is_empty());
	}

	#[test]
	fn no_echo_test()
	{
		let now = SystemTime::now();
		let uuid = Uuid::new_v4();
		let mut msg_list = MessageList::new();
		msg_list.post_no_echo("test!".to_string(),0,0,uuid);
		let mut result = msg_list.read(0,0,uuid,now);
		assert!(result.is_empty());
		result = msg_list.read(0,0,Uuid::new_v4(),now);
		assert_eq!(result,"test!\n".to_string());
	}
}
