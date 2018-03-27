use super::*;

use graphene::core::*;
use graphene::common::*;
use ::core::{Element, CompleteLattice};
use std::collections::HashMap;

pub struct ConstraintSystem<'a,L>
	where
		L: 'a + CompleteLattice
{
	constraints: AdjListGraph<u32, fn(&'a Element<L>)->Element<L>>
}

impl<'a,L> ConstraintSystem<'a,L>
	where
		L: CompleteLattice
{
	pub fn new() -> Self
	{
		Self::empty_graph()
	}
	
	pub fn evaluate_flow_variable(&self, fv: u32, values: &'a HashMap<u32,Element<L>>)
		-> Element<L>
	{
		let all_edges_iter = self.constraints.all_edges().into_iter();
		let mut sourced_in_fv = all_edges_iter.filter(|e| e.source() == fv);
		
		if let Some(first_edge) = sourced_in_fv.next(){
			let mut result = first_edge.weight()(&values[&first_edge.sink()]);
			for e in sourced_in_fv {
				result += e.weight()(&values[&e.sink()]);
			}
			result
		}else{
			// flow variable has no dependencies
			// Therefore, just return whatever values the map
			// gives it
			values[&fv].clone()
		}
	}
}

impl<'a,L> BaseGraph for ConstraintSystem<'a,L>
	where
		L: 'a + CompleteLattice
{
	type Vertex = u32;
	type Weight = fn(&'a Element<L>)->Element<L>;
	type VertexIter = <AdjListGraph<u32, fn(&'a Element<L>)->Element<L>> as BaseGraph>::VertexIter;
	type EdgeIter = <AdjListGraph<u32, fn(&'a Element<L>)->Element<L>> as BaseGraph>::EdgeIter;
	
	fn empty_graph() -> Self
	{
		ConstraintSystem{constraints: AdjListGraph::empty_graph()}
	}
	fn all_vertices(&self) -> Self::VertexIter
	{
		self.constraints.all_vertices()
	}
	fn all_edges(&self) -> Self::EdgeIter
	{
		self.constraints.all_edges()
	}
	fn add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>
	{
		self.constraints.add_vertex(v)
	}
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>
	{
		unimplemented!();
	}
	fn add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>)
		-> Result<(), ()>
	{
		self.constraints.add_edge(e)
	}
	fn remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>)
		-> Result<(), ()>
	{
		unimplemented!();
	}
}

