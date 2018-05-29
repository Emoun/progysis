
use core::{
	CompleteLattice, Element, Direction
};

pub trait Analysis
{
	type Lattice: CompleteLattice;
	type Action;
	
	const DIRECTION: Direction;
	
	fn transfer(&Element<Self::Lattice>, &Self::Action) -> Element<Self::Lattice>;
}
