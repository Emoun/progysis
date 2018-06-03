
use core::{
	Analysis, CompleteLattice, SubLattice
};
use graphene::core::{
	BaseGraph, EdgeWeightedGraph,
	trait_aliases::{
		IntoFromIter
	}
};


pub trait Worklist: Iterator<Item=u32>
{
	fn insert(&mut self, v: u32);
	
	fn initialize<G,N,L>(program: &G) -> Self
		where
			G: EdgeWeightedGraph<EdgeWeight=N::Action> + BaseGraph<Vertex=u32>,
			<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
			<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
			N: Analysis<G,L>,
			L: CompleteLattice + SubLattice<N::Lattice>
	;
}