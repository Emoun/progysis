
use std::vec::Vec;
use core::{
	ConstraintSystem, Worklist, Analysis, Direction
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
	fn insert(&mut self, v: u32, _: Direction)
	{
		self.list.push(v);
	}
	
	fn initialize<G,N>(g: &G) -> Self
		where
			G: ConstraintSystem,
			<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
			<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
			N: Analysis,
	{
		let mut new = FifoWorklist{list: Vec::new()};
		for v in g.all_vertices().into_iter(){
			new.insert(v,N::DIRECTION);
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