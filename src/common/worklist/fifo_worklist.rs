use super::*;

use std::vec::Vec;
use common::ConstraintSystem;
use core::CompleteLattice;
use graphene::core::BaseGraph;

pub struct FifoWorklist
{
	list: Vec<u32>
}

impl Worklist for FifoWorklist
{
	fn insert(&mut self, v: u32)
	{
		self.list.push(v);
	}
	
	fn initialize<L>(cs: &ConstraintSystem<L>) -> Self
		where L: CompleteLattice
	{
		let mut new = FifoWorklist{list: Vec::new()};
		for v in cs.all_vertices().into_iter(){
			new.insert(v);
		}
		new
	}
}

impl Iterator for FifoWorklist
{
	type Item = u32;
	
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.list.len() != 0 {
			Some(self.list.remove(0))
		}else{
			None
		}
	}
}