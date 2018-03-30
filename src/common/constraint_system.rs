use super::*;

use graphene::core::*;
use graphene::common::*;
use ::core::{Element, CompleteLattice};
use ::common::worklist::Worklist;
use ::common::MonotoneFunction;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::fmt::{Display};

pub struct ConstraintSystem<L>
	where
		L: CompleteLattice
{
	constraints: AdjListGraph<u32,MonotoneFunction<L>>
}

impl<L> ConstraintSystem<L>
	where
		L: CompleteLattice
{
	pub fn new() -> Self
	{
		Self::empty_graph()
	}
	
	pub fn evaluate_flow_variable(&self, fv: u32, values: &HashMap<u32,Element<L>>)
		-> Element<L>
	{
		let all_edges_iter = self.constraints.all_edges().into_iter();
		let mut sourced_in_fv = all_edges_iter.filter(|e| e.source() == fv);
		
		if let Some(first_edge) = sourced_in_fv.next(){
			let mut result = first_edge.weight().func(&values[&first_edge.sink()]);
			for e in sourced_in_fv {
				result += e.weight().func(&values[&e.sink()]);
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
		while let Some(fV) = worklist.next(){
			let new_value = self.evaluate_flow_variable(fV, initial_values);
			if new_value != initial_values[&fV] {
				worklist.insert(fV);
				initial_values.insert(fV, new_value);
			}
		}
	}
}

impl<L> BaseGraph for ConstraintSystem<L>
	where
		L: CompleteLattice,
{
	type Vertex = u32;
	type Weight = MonotoneFunction<L>;
	type VertexIter = <AdjListGraph<u32,MonotoneFunction<L>> as BaseGraph>::VertexIter;
	type EdgeIter = <AdjListGraph<u32,MonotoneFunction<L>> as BaseGraph>::EdgeIter;
	
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
