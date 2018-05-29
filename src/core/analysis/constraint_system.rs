
use graphene::core::{
	BaseGraph, EdgeWeightedGraph, Edge,
	trait_aliases::{
		IntoFromIter
	}
};
use ::core::{
	Element, CompleteLattice,Worklist, Analysis, Direction
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
				for (v,d) in fv_dependentants::<_,N>(self, fv){
					worklist.insert(v,d);
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
fn fv_dependentants<G,N>(g: &G, fv: u32) -> Vec<(u32, Direction)>
	where
		G: ConstraintSystem,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
		N: Analysis<Action=G::EdgeWeight>,
{
	use self::Direction::*;
	match N::DIRECTION {
		Forward => fv_dependencies(g, fv, Backward),
		Backward => fv_dependencies(g, fv, Forward),
		Both => {
			fv_dependencies(g,fv, Both).into_iter().map(
				|((v,d),e)| ((v,d.reverse()),e)
			).collect()
		},
	}.into_iter().map(|(dependant,_)| dependant).collect()
}

/// The flow variables the given flow variable is dependent on.
fn fv_dependencies<G>(g: &G, fv: u32, direction: Direction) -> Vec<((u32, Direction), G::EdgeId)>
	where
		G: ConstraintSystem,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
{
	use self::Direction::*;
	match direction {
		Forward => g.edges_sinked_in(fv).into_iter().map(
				|e| ((*e.source(),Forward),*e.id())
			).collect(),
		Backward => g.edges_sourced_in(fv).into_iter().map(
				|e| ((*e.sink(),Backward),*e.id())
			).collect(),
		Both => fv_dependencies(g, fv, Forward).into_iter()
			.chain(fv_dependencies(g,fv, Backward).into_iter())
			.collect(),
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
	let dependencies = fv_dependencies(g, fv, N::DIRECTION);
	let mut dependencies_iter = dependencies.iter();
	if let Some(first_edge) = dependencies_iter.next(){
		let mut result = N::transfer(&values[&(first_edge.0).0], g.weight_ref(first_edge.1).unwrap());
		while let Some(e) = dependencies_iter.next() {
			result += N::transfer(&values[&(e.0).0], g.weight_ref(e.1).unwrap());
		}
		result
	}else{
		// flow variable has no dependencies
		// Therefore, just return whatever values the map
		// gives it
		values[&fv].clone()
	}
}



