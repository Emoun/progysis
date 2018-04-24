

use graphene::core::*;
use ::core::{Element, CompleteLattice};
use ::common::worklist::Worklist;

use std::collections::HashMap;

///
/// Trait alias
///
pub trait ConstraintSystemGraph<A>:
	EdgeWeightedGraph<EdgeWeight=A> +
	BaseGraph<Vertex=u32>
	where
		<Self as BaseGraph>::VertexIter: IdIter<u32>,
		<Self as BaseGraph>::EdgeIter: IdIter<(u32,u32,<Self as BaseGraph>::Edge)>
{}
impl<A,G> ConstraintSystemGraph<A> for G
	where
		G: 	EdgeWeightedGraph<EdgeWeight=A> +
			BaseGraph<Vertex=u32>,
		<Self as BaseGraph>::VertexIter: IdIter<u32>,
		<Self as BaseGraph>::EdgeIter: IdIter<(u32,u32,<Self as BaseGraph>::Edge)>
{}

pub struct ConstraintSystem<G,L,A>
	where
		G: ConstraintSystemGraph<A>,
		<G as BaseGraph>::VertexIter: IdIter<u32>,
		<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,<G as BaseGraph>::Edge)>,
		L: CompleteLattice,
{
	pub graph: G,
	func: fn(&Element<L>, &A) -> Element<L>
}

impl<G,L,A> ConstraintSystem<G,L,A>
	where
		G: ConstraintSystemGraph<A>,
		<G as BaseGraph>::VertexIter: IdIter<u32>,
		<G as BaseGraph>::EdgeIter: IdIter<(u32,u32,<G as BaseGraph>::Edge)>,
		L: CompleteLattice,
{
	pub fn new(graph: G, func: fn(&Element<L>, &A) -> Element<L>) -> Self
	{
		Self{graph, func}
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
	
	///
	/// The states set in the initial values are assumed to be the initial states,
	/// and the values are their initial values.
	/// The initial state's function spaces do not have to have entries to every variable.
	/// The other states must not have any entries in the initial state map.
	///
	///
	///
	pub fn solve<W>(&self, initial_values: &mut HashMap<u32,Element<L>>)
		where
			W: Worklist
	{
		let mut worklist = W::initialize(self);
		
		// Initialize all states
		for i in self.graph.all_vertices(){
			if !initial_values.contains_key(&i) {
				initial_values.insert(i, Element::bottom());
			}
		}
		
		while let Some(fv) = worklist.next(){
			let new_value = self.evaluate_flow_variable(fv, initial_values);
			if new_value != initial_values[&fv] {
				worklist.insert(fv);
				initial_values.insert(fv, new_value);
			}
		}
	}
}
