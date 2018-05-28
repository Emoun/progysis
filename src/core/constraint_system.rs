
use graphene::core::{
	BaseGraph, EdgeWeightedGraph, Edge,
	trait_aliases::{
		IntoFromIter
	}
};
use ::core::{
	Element, CompleteLattice,Worklist, Analysis,
};

use std::{
	collections::HashMap,
};

///
/// Trait alias
///
pub trait ConstraintSystem:
	EdgeWeightedGraph +
	BaseGraph<Vertex=u32> + Sized
	where
		<Self as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<Self as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<Self as BaseGraph>::EdgeId)>
{
	///
	/// The states set in the initial values are assumed to be the initial states,
	/// and the values are their initial values.
	/// The initial state's function spaces do not have to have entries to every variable.
	/// The other states must not have any entries in the initial state map.
	///
	fn analyze<N,W>(&self, initial_values: &mut HashMap<u32,Element<N::Lattice>>)
		where
			W: Worklist,
			N: Analysis<Action=Self::EdgeWeight>
	{
		let mut worklist = W::initialize::<_,N>(self);
		
		// Initialize all states
		for i in self.all_vertices(){
			if !initial_values.contains_key(&i) {
				initial_values.insert(i, Element::bottom());
			}
		}
		
		while let Some(fv) = worklist.next(){
			let new_value = evaluate_flow_variable::<_,N>(self, fv, initial_values);
			if new_value != initial_values[&fv] {
				for (dependant,_) in fv_dependentants::<_,N>(self, fv){
					worklist.insert(dependant);
				}
				initial_values.insert(fv, new_value);
			}
		}
	}
}

impl<G> ConstraintSystem for G
	where
		G: 	EdgeWeightedGraph +
		BaseGraph<Vertex=u32>,
		<Self as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<Self as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<Self as BaseGraph>::EdgeId)>
{}

// Helper functions

/// The flow variables that depend on the given flow variable.
fn fv_dependentants<G,N>(g: &G, fv: u32) -> Vec<(u32,G::EdgeId)>
	where
		G: ConstraintSystem,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
		N: Analysis<Action=G::EdgeWeight>,
{
	use core::AnalysisDirection::*;
	match N::direction(){
		Forward => adjacent(g, fv, true),
		Backward => adjacent(g, fv, false),
		_ => unimplemented!()
	}
}

/// The flow variables the given flow variable is dependent on.
fn fv_dependencies<G,N>(g: &G, fv: u32) -> Vec<(u32,G::EdgeId)>
	where
		G: ConstraintSystem,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
		N: Analysis<Action=G::EdgeWeight>,
{
	use core::AnalysisDirection::*;
	match N::direction(){
		Forward => adjacent(g, fv, false),
		Backward => adjacent(g, fv, true),
		_ => unimplemented!()
	}
}

fn adjacent<G>(g: &G, fv: u32, outgoing: bool) -> Vec<(u32, G::EdgeId)>
	where
		G: ConstraintSystem,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
{
	if outgoing {
		g.edges_sourced_in(fv).into_iter().map(|e| (*e.sink(),*e.id())).collect()
	}else{
		g.edges_sinked_in(fv).into_iter().map(|e| (*e.source(),*e.id())).collect()
	}
}

fn evaluate_flow_variable<G,N>(g: &G, fv: u32, values: &HashMap<u32,Element<N::Lattice>>)
	 -> Element<N::Lattice>
	where
		G: ConstraintSystem,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
		N: Analysis<Action=G::EdgeWeight>,

{
	let dependencies = fv_dependencies::<_,N>(g, fv);
	let mut dependencies_iter = dependencies.iter();
	if let Some(first_edge) = dependencies_iter.next(){
		let mut result = N::transfer(&values[&first_edge.0], g.weight_ref(first_edge.1).unwrap());
		while let Some(e) = dependencies_iter.next() {
			result += N::transfer(&values[&e.0], g.weight_ref(e.1).unwrap());
		}
		result
	}else{
		// flow variable has no dependencies
		// Therefore, just return whatever values the map
		// give	s it
		values[&fv].clone()
	}
}



