
use core::{
	Analysis, SubLattice, Bottom
};
use graphene::core::{
	Graph,
};
use std::hash::Hash;

pub trait Worklist<'a,G>: Iterator<Item=G::Vertex>
	where
		G: Graph<'a>,
		G::Vertex: Hash,
{
	fn insert(&mut self, v: G::Vertex);
	
	fn initialize<N,L>(program: &G) -> Self
		where
			N: Analysis<'a,G,L>,
			L: Bottom + SubLattice<N::Lattice>
	;
}