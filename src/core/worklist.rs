
use core::{ConstraintSystem, ConstraintSystemGraph,CompleteLattice};
use graphene::core::*;


pub trait Worklist: Iterator<Item=u32>
{
	fn insert(&mut self, v: u32);
	
	fn initialize<G,L,A>(cs: &ConstraintSystem<G,L,A>) -> Self
		where
			G: ConstraintSystemGraph<A>,
			<G as BaseGraph>::VertexIter: IdIter<u32>,
			<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,<G as BaseGraph>::Edge)>,
			L: CompleteLattice;
}