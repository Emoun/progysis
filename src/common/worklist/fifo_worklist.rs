
use std::vec::Vec;
use core::{
	ConstraintSystem, ConstraintSystemGraph, CompleteLattice, Worklist, Analysis
};
use graphene::core::{
	BaseGraph,
	trait_aliases::{
		IntoFromIter
	}
};

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
	
	fn initialize<G,L,A,N>(cs: &ConstraintSystem<G,L,A,N>) -> Self
		where
			G: ConstraintSystemGraph<A>,
			<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
			<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
			L: CompleteLattice,
			N: Analysis<L,A>,
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