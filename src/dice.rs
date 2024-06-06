use rand;

pub struct Dice
{
	pub number: i16,
	pub die: i16,
}

impl Dice
{
	pub fn roll(&self) -> i16
	{
		let mut result = 0;
		let mut count = 0;
		while count < self.number
		{
			count += 1;
			result += 1+rand::random::<i16>()%self.die;
		}
		return result;
	}
}


