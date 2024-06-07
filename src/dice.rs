use rand::Rng;

pub struct Dice
{
	pub number: i16,
	pub die: i16,
}

impl Dice
{
	pub fn roll(&self) -> i16
	{
		let mut rng = rand::thread_rng();
		let mut result = 0;
		let mut count = 0;
		while count < self.number
		{
			count += 1;
			result += rng.gen_range(1..self.die+1);
		}
		return result;
	}
}


#[cfg(test)]
mod dice_unit_test
{
	use super::*;

	#[test]
	fn range_test()
	{
		let die = Dice { number: 3, die: 6 };
		for i in 1..100
		{
			assert!(die.roll() <= 18);
			assert!(die.roll() >= 3);
		}
	}
}

