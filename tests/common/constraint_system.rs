use super::*;

use ::common::lattices::{Sign, SignPowerSet};
use progysis::common::{ConstraintSystem, MonotoneFunction};
use progysis::common::worklist::FifoWorklist;
use graphene::core::{BaseGraph, EdgeWeightedGraph};
use graphene::common::AdjListGraph;
use std::collections::HashMap;


fn add(e: &Element<u32>, action: &u32) -> Element<u32>
{
	Element::new(e.inner + action)
}

#[test]
fn solve_test()
{
	let mut map: HashMap<u32,_> = HashMap::new();
	map.insert(0, Element::new(1));
	map.insert(1, Element::bottom());
	map.insert(2, Element::bottom());
	
	let mut cs_graph = AdjListGraph::empty_graph();
	cs_graph.add_vertex(0).unwrap();
	cs_graph.add_vertex(1).unwrap();
	cs_graph.add_vertex(2).unwrap();
	
	cs_graph.add_edge_weighted((1, 0),1).unwrap();
	cs_graph.add_edge_weighted((2, 1), 2).unwrap();
	
	let cs = ConstraintSystem::new(cs_graph, add);
	
	cs.solve::<FifoWorklist>(&mut map);
	
	assert_eq!(1, map[&0].inner);
	assert_eq!(2, map[&1].inner);
	assert_eq!(4, map[&2].inner);
}




























