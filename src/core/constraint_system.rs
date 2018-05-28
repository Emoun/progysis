
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
	marker::PhantomData
};

///
/// Trait alias
///
pub trait ConstraintSystemGraph<A>:
	EdgeWeightedGraph<EdgeWeight=A> +
	BaseGraph<Vertex=u32>
	where
		<Self as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<Self as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<Self as BaseGraph>::EdgeId)>
{}
impl<A,G> ConstraintSystemGraph<A> for G
	where
		G: 	EdgeWeightedGraph<EdgeWeight=A> +
			BaseGraph<Vertex=u32>,
		<Self as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<Self as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<Self as BaseGraph>::EdgeId)>
{}

pub struct ConstraintSystem<G,L,A,N>
	where
		G: ConstraintSystemGraph<A>,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
		L: CompleteLattice,
		N: Analysis<L,A>,
{
	pub graph: G,
	pha1: PhantomData<L>,
	pha2: PhantomData<A>,
	pha3: PhantomData<N>,
}

impl<G,L,A,N> ConstraintSystem<G,L,A,N>
	where
		G: ConstraintSystemGraph<A>,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32,u32,<G as BaseGraph>::EdgeId)>,
		L: CompleteLattice,
		N: Analysis<L,A>,
{
	pub fn new(graph: G) -> Self
	{
		Self{graph, pha1: PhantomData, pha2: PhantomData, pha3: PhantomData}
	}
	
	fn evaluate_flow_variable(&self, fv: u32, values: &HashMap<u32,Element<L>>)
		-> Element<L>
	{
		let dependencies = self.fv_dependencies(fv);
		let mut dependencies_iter = dependencies.iter();
		if let Some(first_edge) = dependencies_iter.next(){
			let mut result = N::transfer(&values[&first_edge.0], self.graph.weight_ref(first_edge.1).unwrap());
			while let Some(e) = dependencies_iter.next() {
				result += N::transfer(&values[&e.0], self.graph.weight_ref(e.1).unwrap());
			}
			result
		}else{
			// flow variable has no dependencies
			// Therefore, just return whatever values the map
			// give	s it
			values[&fv].clone()
		}
	}
	
	/// The flow variables that depend on the given flow variable.
	fn fv_dependentants(&self, fv: u32) -> Vec<(u32,G::EdgeId)>
	{
		use core::AnalysisDirection::*;
		match N::direction(){
			Forward => self.adjacent(fv, true),
			Backward => self.adjacent(fv, false),
			_ => unimplemented!()
		}
	}
	
	/// The flow variables the given flow variable is dependent on.
	fn fv_dependencies(&self, fv: u32) -> Vec<(u32,G::EdgeId)>
	{
		use core::AnalysisDirection::*;
		match N::direction(){
			Forward => self.adjacent(fv, false),
			Backward => self.adjacent(fv, true),
			_ => unimplemented!()
		}
	}
	
	fn adjacent(&self, fv: u32, outgoing: bool) -> Vec<(u32, G::EdgeId)>
	{
		if outgoing {
			self.graph.edges_sourced_in(fv).into_iter().map(|e| (*e.sink(),*e.id())).collect()
		}else{
			self.graph.edges_sinked_in(fv).into_iter().map(|e| (*e.source(),*e.id())).collect()
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
				for (dependant,_) in self.fv_dependentants(fv){
					worklist.insert(dependant);
				}
				initial_values.insert(fv, new_value);
			}
		}
	}
}
