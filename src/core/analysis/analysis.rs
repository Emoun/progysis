
use core::{
	CompleteLattice, SubLattice, Worklist, Bottom
};
use graphene::{
	core::{
		Graph, Edge
	}
};
use std::{
	collections::{
		HashMap
	},
	hash::Hash
};

pub trait Analysis<'a,G,L>
	where
		Self: Sized,
		G: Graph<'a>,
		G::Vertex: Hash,
		L: Bottom + SubLattice<Self::Lattice>
{
	type Lattice: CompleteLattice;
	
	const FORWARD: bool;
	
	fn transfer(dependency: &L, target: &L, action: &G::EdgeWeight)
		-> Self::Lattice
	;
	
	fn analyze<W>(g: &'a G, initial_values: &mut HashMap<G::Vertex,L>)
		where
			W: Worklist<'a,G>
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
			if !(*initial_values[&fv].sub_lattice_ref() >= new_value)   {
				if let Some(t) = initial_values.get_mut(&fv) {
					*t.sub_lattice_ref_mut() += new_value;
				}else {
					unreachable!("All flow variables should have been initialized above")
				}
				for v in fv_dependentants::<Self,_,_>(g, fv){
					worklist.insert(v.0);
				}
			}
		}
	}
}

// Helper functions

/// The flow variables that depend on the given flow variable.
fn fv_dependentants<'a,N,L,G>(g: &'a G, fv: G::Vertex) -> Vec<(G::Vertex, &'a G::EdgeWeight)>
	where
		G: Graph<'a>,
		G::Vertex: Hash,
		N: Analysis<'a,G,L>,
		L: Bottom + SubLattice<N::Lattice>
{
	let result = if N::FORWARD {
		fv_dependencies(g, fv, false)
	}else{
		fv_dependencies(g, fv, true)
	};
	result.into_iter().collect()
}

/// The flow variables the given flow variable is dependent on.
fn fv_dependencies<'a,G>(g: &'a G, fv: G::Vertex, forward: bool) -> Vec<(G::Vertex, &'a G::EdgeWeight)>
	where
		G: Graph<'a>,
		G::Vertex: Hash,
{
	
	if forward {
		g.edges_sinked_in(fv).into_iter().map(
			|e| (e.source(), e.2)).collect()
	}else{
		g.edges_sourced_in(fv).into_iter().map(
			|e| (e.sink(), e.2)).collect()
	}
}

fn evaluate_flow_variable<'a,N,Nl,L,G>(g: &'a G, fv: G::Vertex, values: &HashMap<G::Vertex,L>)
							   -> N::Lattice
	where
		G: Graph<'a>,
		G::Vertex: Hash,
		N: Analysis<'a,G,L,Lattice=Nl>,
		Nl: CompleteLattice, // Used to circumvent this problem: https://stackoverflow.com/questions/50660911
		L: Bottom + SubLattice<N::Lattice>

{
	let dependencies = fv_dependencies(g, fv, N::FORWARD);
	let mut dependencies_iter = dependencies.iter();
	if let Some(first_edge) = dependencies_iter.next(){
		let mut result = N::transfer(&values[&(first_edge.0)], &values[&fv], first_edge.1);
		while let Some(e) = dependencies_iter.next() {
			result += N::transfer(&values[&(e.0)], &values[&fv], e.1);
		}
		result
	}else{
		// flow variable has no dependencies
		// Therefore, just return whatever values the map
		// gives it
		values[&fv].sub_lattice_ref().clone()
	}
}