use super::*;

use ::common::lattices::{Sign, SignPowerSet};
use progysis::common::ConstraintSystem;
use graphene::core::{BaseEdge,BaseGraph};
use std::collections::HashMap;

pub type U32ConstraintSystem<'a> = ConstraintSystem<'a,u32>;

#[test]
fn simple_test()
{
	let mut map = HashMap::new();
	map.insert(0, Element::new(1));
	map.insert(1, Element::new(10));
	map.insert(2, Element::new(20));
	
	let mut cs = U32ConstraintSystem::new();
	
	let add_one: fn(&Element<u32>) -> Element<u32> =
		|&Element{inner}| Element::new(inner + 1);
	let add_two: fn(&Element<u32>) -> Element<u32> =
		|&Element{inner}| Element::new(inner + 2);
	cs.add_vertex(0).unwrap();
	cs.add_vertex(1).unwrap();
	cs.add_vertex(2).unwrap();
	
	assert_eq!(Element::new(1), cs.evaluate_flow_variable(0, &map));
	assert_eq!(Element::new(10), cs.evaluate_flow_variable(1, &map));
	assert_eq!(Element::new(20), cs.evaluate_flow_variable(2, &map));
	
	cs.add_edge(BaseEdge::new(1,0,add_one)).unwrap();
	cs.add_edge(BaseEdge::new(2,1,add_two)).unwrap();
	
	assert_eq!(Element::new(1), cs.evaluate_flow_variable(0, &map));
	assert_eq!(Element::new(2), cs.evaluate_flow_variable(1, &map));
	assert_eq!(Element::new(12), cs.evaluate_flow_variable(2, &map));
}




























