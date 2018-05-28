use super::*;

use ::common::lattices::{
	Sign, SignPowerSet, StringSignTFSpace
};
use progysis::{
	core::{
		ConstraintSystem
	},
	common::worklist::FifoWorklist
};
use graphene::{
	core::{
		BaseGraph, EdgeWeightedGraph
	},
	common::AdjListGraph
};
use std::{
	collections::HashMap,
	marker::PhantomData
};




pub struct U32Analysis {}

impl Analysis for U32Analysis
{
	type Lattice = u32;
	type Action = u32;
	
	fn transfer(e: &Element<u32>, action: &u32) -> Element<u32>
	{
		Element::new(e.inner + action)
	}
	
	fn direction() -> AnalysisDirection
	{
		AnalysisDirection::Forward
	}
}

#[test]
fn solve_test()
{
	let mut map: HashMap<u32,_> = HashMap::new();
	map.insert(0, Element::new(1));
	
	let mut program = AdjListGraph::empty_graph();
	program.add_vertex(0).unwrap();
	program.add_vertex(1).unwrap();
	program.add_vertex(2).unwrap();
	
	program.add_edge_weighted((0, 1), 1).unwrap();
	program.add_edge_weighted((1, 2), 2).unwrap();
	
	let cs = ConstraintSystem::<_, U32Analysis>::new(program);
	
	cs.solve::<FifoWorklist>(&mut map);
	
	assert_eq!(1, map[&0].inner);
	assert_eq!(2, map[&1].inner);
	assert_eq!(4, map[&2].inner);
}

enum Action{
	DeclareX,
	DeclareY,
	YIsMinus1,
	XIs0,
	Skip,
	IncX,
	ReadY,
}

struct SignAnalysis<'a>
{
	pha: PhantomData<&'a u8>
}

impl<'a> Analysis for SignAnalysis<'a>
{
	type Lattice = StringSignTFSpace<'a>;
	type Action = Action;
	
	fn transfer(init: &Element<StringSignTFSpace<'a>>, acc: &Action) -> Element<StringSignTFSpace<'a>>
	{
		use self::Action::*;
		use self::Sign::*;
		let mut result = init.clone();
		match *acc {
			DeclareX => {
				result["x"] = Element::from_iter(
					vec![Plus, Minus, Zero]
				);
			},
			IncX => {
				let x = if result.has_key("x"){ result["x"].clone()}else{Element::bottom()};
				result["x"] =
					if x >= Element::singleton(Minus){
						if x >= Element::singleton(Zero){
							x + Element::singleton(Plus)
						}else{
							x + Element::singleton(Zero)
						}
					}else if x >= Element::singleton(Zero){
						Element::singleton(Plus)
					}else{
						x
					}
				;
			},
			YIsMinus1 => {
				result["y"] =Element::singleton(Minus);
			},
			XIs0 => {
				result["x"] = Element::singleton(Zero);
			},
			ReadY | DeclareY => {
				result["y"] = Element::from_iter(
					vec![Plus, Minus, Zero]
				);
			},
			_ => (),
		}
		result
	}
	
	fn direction() -> AnalysisDirection
	{
		AnalysisDirection::Forward
	}
}

#[test]
fn solve_tf_space()
{
	use self::Sign::*;
	let mut g = AdjListGraph::<u32, Action>::empty_graph();
	for i in 0..8{
		g.add_vertex(i).unwrap();
	}
	g.add_edge_weighted((0,3),Action::DeclareX).unwrap();
	g.add_edge_weighted((3,2),Action::DeclareY).unwrap();
	g.add_edge_weighted((2,4),Action::YIsMinus1).unwrap();
	g.add_edge_weighted((4,5),Action::XIs0).unwrap();
	g.add_edge_weighted((6,7),Action::IncX).unwrap();
	g.add_edge_weighted((7,5),Action::ReadY).unwrap();
	g.add_edge_weighted((5,6),Action::Skip).unwrap();
	g.add_edge_weighted((5,1),Action::Skip).unwrap();
	
	let cs = ConstraintSystem::<_, SignAnalysis>::new(g);
	let mut initial = HashMap::new();
	initial.insert(0, Element::bottom());
	
	cs.solve::<FifoWorklist>(&mut initial);
	
	let top = Element::from_iter(vec![Plus,Minus,Zero]);
	let plus_zero = Element::from_iter(vec![Plus, Zero]);
	let minus = Element::singleton(Minus);
	let plus = Element::singleton(Plus);
	
	assert_eq!(false, initial[&0].has_key("x"));	assert_eq!(false, initial[&0].has_key("y"));
	assert_eq!(plus_zero, initial[&1]["x"]);		assert_eq!(top, initial[&1]["y"]);
	assert_eq!(top, initial[&2]["x"]);				assert_eq!(top, initial[&2]["y"]);
	assert_eq!(top, initial[&3]["x"]);				assert_eq!(false, initial[&3].has_key("y"));
	assert_eq!(top, initial[&4]["x"]);				assert_eq!(minus, initial[&4]["y"]);
	assert_eq!(plus_zero, initial[&5]["x"]);		assert_eq!(top, initial[&5]["y"]);
	assert_eq!(plus_zero, initial[&6]["x"]);		assert_eq!(top, initial[&6]["y"]);
	assert_eq!(plus, initial[&7]["x"]);				assert_eq!(top, initial[&7]["y"]);
}


























