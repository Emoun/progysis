

use graphene::core::*;
use graphene::common::*;
use ::core::{Element, CompleteLattice};
use ::common::worklist::Worklist;
use ::common::MonotoneFunction;

use std::collections::HashMap;

custom_graph!{
	pub struct ConstraintSystem<L>
	as AdjListGraph<u32,MonotoneFunction<L>>
	where L: CompleteLattice
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
		let all_edges_iter = self.all_edges().into_iter();
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
		while let Some(fv) = worklist.next(){
			let new_value = self.evaluate_flow_variable(fv, initial_values);
			if new_value != initial_values[&fv] {
				worklist.insert(fv);
				initial_values.insert(fv, new_value);
			}
		}
	}
}
