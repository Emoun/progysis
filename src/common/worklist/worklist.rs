
use common::{ConstraintSystem, ConstraintSystemGraph};
use core::CompleteLattice;
use graphene::core::*;


pub trait Worklist: Iterator<Item=u32>
{
	fn insert(&mut self, v: u32);
	
	fn initialize<G,L,A,I>(cs: &ConstraintSystem<G,L,A,I>) -> Self
		where
			G: ConstraintSystemGraph<A,I>,
			<G as BaseGraph>::VertexIter: IdIter<u32>,
			<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,I)>,
			I: Id,
			L: CompleteLattice;
}