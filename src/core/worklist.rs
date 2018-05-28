
use core::{
	ConstraintSystem, Analysis
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
	
	fn initialize<G,N>(cs: &G) -> Self
		where
			G: ConstraintSystem,
			<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
			<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
			N: Analysis,
	;
}