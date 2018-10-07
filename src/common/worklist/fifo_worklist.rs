
use std::{
	vec::Vec,
	hash::Hash,
};
use core::{
	Worklist, Analysis, SubLattice, Bottom
};
use graphene::core::{
	Graph,
};

pub struct FifoWorklist<'a,G>
	where
		G: Graph<'a>,
		G::Vertex: Hash,
{
	list: Vec<G::Vertex>
}

impl<'a,G> Worklist<'a,G> for FifoWorklist<'a,G>
	where
		G: Graph<'a>,
		G::Vertex: Hash,
{
	fn insert(&mut self, v: G::Vertex)
	{
		self.list.push(v);
	}
	
	fn initialize<N,L>(g: &G) -> Self
		where
			N: Analysis<'a,G,L>,
			L: Bottom + SubLattice<N::Lattice>
	{
		let mut new = FifoWorklist{list: Vec::new()};
		for v in g.all_vertices().into_iter(){
			new.insert(v);
		}
		new
	}
}

impl<'a,G> Iterator for FifoWorklist<'a,G>
	where
		G: Graph<'a>,
		G::Vertex: Hash
{
	type Item = G::Vertex;
	
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.list.len() != 0 {
			Some(self.list.remove(0))
		}else{
			None
		}
	}
}