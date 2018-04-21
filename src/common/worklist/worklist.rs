
use common::ConstraintSystem;
use core::CompleteLattice;
use graphene::core::*;


pub trait Worklist: Iterator<Item=u32>
{
	fn insert(&mut self, v: u32);
	
	fn initialize<G,L,A>(cs: &ConstraintSystem<G,L,A>) -> Self
		where
			G: BaseGraph<Vertex=u32, Weight=A>,
			<G as BaseGraph>::VertexIter: VertexIter<u32>,
			<G as BaseGraph>::EdgeIter: EdgeIter<u32,A>,
			L: CompleteLattice,
			A: Weight;
}