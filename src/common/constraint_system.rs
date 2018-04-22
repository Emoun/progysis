

use graphene::core::*;
use graphene::common::*;
use ::core::{Element, CompleteLattice};
use ::common::worklist::Worklist;
use std::iter::FromIterator;

use std::collections::HashMap;
use std::marker::PhantomData;

pub trait ConstraintSystemGraph<A,I>:
	EdgeWeightedGraph<EdgeWeight=A> +
	WeightedGraph<Weight=A,WeightRef=I> +
	BaseGraph<Vertex=u32,Edge=I>
	where
		<Self as BaseGraph>::VertexIter: IdIter<u32>,
		<Self as BaseGraph>::EdgeIter: IdIter<(u32,u32,I)>,
		I: Id,
{}
impl<A,I,G> ConstraintSystemGraph<A,I> for G
	where
		G: 	EdgeWeightedGraph<EdgeWeight=A> +
			WeightedGraph<Weight=A,WeightRef=I> +
			BaseGraph<Vertex=u32,Edge=I>,
		<G as BaseGraph>::VertexIter: IdIter<u32>,
		<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,I)>,
		I: Id,
{}

pub struct ConstraintSystem<G,L,A,I>
	where
		G: ConstraintSystemGraph<A,I>,
		<G as BaseGraph>::VertexIter: IdIter<u32>,
		<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,I)>,
		I: Id,
		L: CompleteLattice,
{
	pub graph: G,
	func: fn(&Element<L>, &A) -> Element<L>,
	ph: PhantomData<I>
}

impl<G,L,A,I> ConstraintSystem<G,L,A,I>
	where
		G: ConstraintSystemGraph<A,I>,
		<G as BaseGraph>::VertexIter: IdIter<u32>,
		<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,I)>,
		I: Id,
		L: CompleteLattice,
{
	pub fn new(graph: G, func: fn(&Element<L>, &A) -> Element<L>) -> Self
	{
		Self{graph, func, ph: PhantomData}
	}
	
	fn evaluate_flow_variable(&self, fv: u32, values: &HashMap<u32,Element<L>>)
		-> Element<L>
	{
		let all_edges_iter = self.graph.all_edges().into_iter();
		let mut sourced_in_fv = all_edges_iter.filter(|e| *e.source() == fv);
		
		if let Some(first_edge) = sourced_in_fv.next(){
			let mut result = (self.func)(&values[&first_edge.sink()], self.graph.weight_of(first_edge).unwrap());
			for e in sourced_in_fv {
				result += (self.func)(&values[&e.sink()], self.graph.weight_of(first_edge).unwrap());
			}
			result
		}else{
			// flow variable has no dependencies
			// Therefore, just return whatever values the map
			// give	s it
			values[&fv].clone()
		}
	}
	
	pub fn solve<W>(&self, initial_values: &mut HashMap<u32,Element<L>>)
		where
			W: Worklist
	{
		let mut worklist = W::initialize(self);
		while let Some(fv) = worklist.next(){
			let new_value = self.evaluate_flow_variable(fv, initial_values);
			if new_value != initial_values[&fv] {
				worklist.insert(fv);
				initial_values.insert(fv, new_value);
			}
		}
	}
}
