
use core::{
	ConstraintSystem, ConstraintSystemGraph,CompleteLattice, Analysis
};
use graphene::core::{
	BaseGraph,
	trait_aliases::{
		IntoFromIter
	}
};


pub trait Worklist: Iterator<Item=u32>
{
	fn insert(&mut self, v: u32);
	
	fn initialize<G,L,A,N>(cs: &ConstraintSystem<G,L,A,N>) -> Self
		where
			G: ConstraintSystemGraph<A>,
			<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
			<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
			L: CompleteLattice,
			N: Analysis<L,A>,
	;
}