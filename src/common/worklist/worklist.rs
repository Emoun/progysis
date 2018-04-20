
use common::ConstraintSystem;
use core::CompleteLattice;

pub trait Worklist: Iterator<Item=u32>
{
	fn insert(&mut self, v: u32);
	
	fn initialize<L>(cs: &ConstraintSystem<L>) -> Self
		where L: CompleteLattice;
}