
use std::vec::Vec;
use core::{
	Worklist, Analysis, CompleteLattice, SubLattice
};
use graphene::core::{
	BaseGraph, EdgeWeightedGraph,
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
	fn insert(&mut self, v: u32, _: bool)
	{
		self.list.push(v);
	}
	
	fn initialize<G,N,L>(g: &G) -> Self
		where
			G: EdgeWeightedGraph<EdgeWeight=N::Action> + BaseGraph<Vertex=u32>,
			<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
			<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
			N: Analysis<G,L>,
			L: CompleteLattice + SubLattice<N::Lattice>
	{
		let mut new = FifoWorklist{list: Vec::new()};
		for v in g.all_vertices().into_iter(){
			new.insert(v,N::FORWARD);
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