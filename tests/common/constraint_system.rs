use super::*;

use ::common::lattices::{Sign, SignPowerSet};
use progysis::common::{ConstraintSystem, MonotoneFunction};
use progysis::common::worklist::FifoWorklist;
use graphene::core::{BaseEdge,BaseGraph};
use std::collections::HashMap;

pub type U32ConstraintSystem = ConstraintSystem<u32>;

fn add_one(e: &Element<u32>) -> Element<u32>
{
	Element::new(e.inner + 1)
}

fn add_two(e: &Element<u32>) -> Element<u32>
{
	Element::new(e.inner + 2)
}

#[test]
fn simple_test()
{
	let mut map = HashMap::new();
	map.insert(0, Element::new(1));
	map.insert(1, Element::new(10));
	map.insert(2, Element::new(20));
	
	let mut cs = U32ConstraintSystem::new();
	
	cs.add_vertex(0).unwrap();
	cs.add_vertex(1).unwrap();
	cs.add_vertex(2).unwrap();
	
	assert_eq!(Element::new(1), cs.evaluate_flow_variable(0, &map));
	assert_eq!(Element::new(10), cs.evaluate_flow_variable(1, &map));
	assert_eq!(Element::new(20), cs.evaluate_flow_variable(2, &map));
	
	cs.add_edge(BaseEdge::new(1,0,MonotoneFunction::new(add_one))).unwrap();
	cs.add_edge(BaseEdge::new(2,1,MonotoneFunction::new(add_two))).unwrap();
	
	assert_eq!(Element::new(1), cs.evaluate_flow_variable(0, &map));
	assert_eq!(Element::new(2), cs.evaluate_flow_variable(1, &map));
	assert_eq!(Element::new(12), cs.evaluate_flow_variable(2, &map));
}

#[test]
fn solve_test()
{
	let mut map: HashMap<u32,_> = HashMap::new();
	map.insert(0, Element::new(1));
	map.insert(1, Element::bottom());
	map.insert(2, Element::bottom());
	
	let mut cs = U32ConstraintSystem::new();
	cs.add_vertex(0).unwrap();
	cs.add_vertex(1).unwrap();
	cs.add_vertex(2).unwrap();
	
	cs.add_edge(BaseEdge::new(1,0,MonotoneFunction::new(add_one))).unwrap();
	cs.add_edge(BaseEdge::new(2,1,MonotoneFunction::new(add_two))).unwrap();
	
	cs.solve::<FifoWorklist>(&mut map);
	
	assert_eq!(1, map[&0].inner);
	assert_eq!(2, map[&1].inner);
	assert_eq!(4, map[&2].inner);
}




























