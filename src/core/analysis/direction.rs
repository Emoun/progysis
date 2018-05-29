
use self::Direction::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
	Forward,
	Backward,
	Both
}

impl Direction {

	pub fn reverse(self) -> Self
	{
		match self {
			Forward => Backward,
			Backward => Forward,
			Both => Both,
		}
	}

}