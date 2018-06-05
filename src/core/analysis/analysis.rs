
use core::{
	CompleteLattice, SubLattice, Worklist, Bottom
};
use graphene::{
	core::{
		EdgeWeightedGraph, Edge,
	}
};
use std::{
	collections::{
		HashMap
	},
	hash::Hash
};

pub trait Analysis<G,L>
	where
		Self: Sized,
		G: EdgeWeightedGraph,
		G::Vertex: Hash,
		L: Bottom + SubLattice<Self::Lattice>
{
	type Lattice: CompleteLattice;
	
	const FORWARD: bool;
	
	fn transfer(dependency: &L, target: &L, action: &G::EdgeWeight)
		-> Self::Lattice
	;
	
	fn analyze<W>(g: &G, initial_values: &mut HashMap<G::Vertex,L>)
		where
			W: Worklist<G>
	{
		let mut worklist = W::initialize::<Self,_>(g);
		
		// Initialize all states
		for i in g.all_vertices(){
			if !initial_values.contains_key(&i) {
				initial_values.insert(i, L::bottom());
			}
		}
		
		while let Some(fv) = worklist.next(){
			let new_value = evaluate_flow_variable::<Self,_,_,_>(g, fv, initial_values);
			if new_value != *initial_values[&fv].sub_lattice_ref() {
				for v in fv_dependentants::<Self,_,_>(g, fv){
					worklist.insert(v);
				}
				if let Some(t) = initial_values.get_mut(&fv) {
					*t.sub_lattice_ref_mut() = new_value;
				}else {
					unreachable!("All flow variables should have been initialized above")
				}
			}
		}
	}
}

// Helper functions

/// The flow variables that depend on the given flow variable.
fn fv_dependentants<N,L,G>(g: &G, fv: G::Vertex) -> Vec<G::Vertex>
	where
		G: EdgeWeightedGraph,
		G::Vertex: Hash,
		N: Analysis<G,L>,
		L: Bottom + SubLattice<N::Lattice>
{
	let result = if N::FORWARD {
		fv_dependencies(g, fv, false)
	}else{
		fv_dependencies(g, fv, true)
	};
	result.into_iter().map(|(dependant,_)| dependant).collect()
}

/// The flow variables the given flow variable is dependent on.
fn fv_dependencies<G>(g: &G, fv: G::Vertex, forward: bool) -> Vec<(G::Vertex, G::EdgeId)>
	where
		G: EdgeWeightedGraph,
		G::Vertex: Hash,
{
	
	if forward {
		g.edges_sinked_in(fv).into_iter().map(
			|e| (*e.source(),*e.id())).collect()
	}else{
		g.edges_sourced_in(fv).into_iter().map(
			|e| (*e.sink(),*e.id())
		).collect()
	}
}

fn evaluate_flow_variable<N,Nl,L,G>(g: &G, fv: G::Vertex, values: &HashMap<G::Vertex,L>)
							   -> N::Lattice
	where
		G: EdgeWeightedGraph,
		G::Vertex: Hash,
		N: Analysis<G,L,Lattice=Nl>,
		Nl: CompleteLattice, // Used to circumvent this problem: https://stackoverflow.com/questions/50660911
		L: Bottom + SubLattice<N::Lattice>

{
	let dependencies = fv_dependencies(g, fv, N::FORWARD);
	let mut dependencies_iter = dependencies.iter();
	if let Some(first_edge) = dependencies_iter.next(){
		let mut result = N::transfer(&values[&(first_edge.0)], &values[&fv], g.weight_ref(first_edge.1).unwrap());
		while let Some(e) = dependencies_iter.next() {
			result += N::transfer(&values[&(e.0)], &values[&fv], g.weight_ref(e.1).unwrap());
		}
		result
	}else{
		// flow variable has no dependencies
		// Therefore, just return whatever values the map
		// gives it
		values[&fv].sub_lattice_ref().clone()
	}
}