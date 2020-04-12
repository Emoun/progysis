
use crate::core::{
	Analysis, SubLattice, Bottom
};
use graphene::core::{Graph, Directed};
use std::hash::Hash;

pub trait Worklist<G>: Iterator<Item=G::Vertex>
	where
		G: Graph<Directedness=Directed>,
		G::Vertex: Hash,
{
	fn insert(&mut self, v: G::Vertex);
	
	fn initialize<N,L>(program: &G) -> Self
		where
			N: Analysis<G,L>,
			L: Bottom + SubLattice<N::Lattice>
	;
}