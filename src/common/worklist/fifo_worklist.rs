use super::*;

use std::vec::Vec;
use common::{ConstraintSystem, ConstraintSystemGraph};
use core::CompleteLattice;
use graphene::core::*;

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
	
	fn initialize<G,L,A,I>(cs: &ConstraintSystem<G,L,A,I>) -> Self
		where
			G: ConstraintSystemGraph<A,I>,
			<G as BaseGraph>::VertexIter: IdIter<u32>,
			<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,I)>,
			I: Id,
			L: CompleteLattice,
	{
		let mut new = FifoWorklist{list: Vec::new()};
		for v in cs.graph.all_vertices().into_iter(){
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