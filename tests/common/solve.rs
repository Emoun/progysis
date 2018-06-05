
use ::common::{
	lattices::{
		Sign, SignPowerSet, StringSignTFSpace
	}
};
use progysis::{
	common::worklist::FifoWorklist,
	core::{
		CompleteLattice, Analysis, SubLattice, PowerSet, TFSpace, U32, U64, Bottom
	}
};
use graphene::{
	core::{
		BaseGraph, EdgeWeightedGraph,
		trait_aliases::IntoFromIter
	},
	common::AdjListGraph
};
use std::{
	collections::HashMap,
	marker::PhantomData,
	ops::{Add,AddAssign},
	hash::Hash
};

pub struct U32Analysis {}

impl<G,L> Analysis<G,L> for U32Analysis
	where
		G: EdgeWeightedGraph<EdgeWeight=u32>,
		G::Vertex: Hash,
		L: Bottom + SubLattice<U32>
{
	type Lattice = U32;
	const FORWARD: bool = true;
	
	fn transfer(e: &L, _: &L, action: &G::EdgeWeight) -> Self::Lattice
	{
		U32(e.sub_lattice_ref().0 + action)
	}
}

#[test]
fn solve_test()
{
	let mut map = HashMap::new();
	map.insert(0, U32(1));
	
	let mut program = AdjListGraph::empty_graph();
	program.add_vertex(0).unwrap();
	program.add_vertex(1).unwrap();
	program.add_vertex(2).unwrap();
	
	program.add_edge_weighted((0, 1), 1).unwrap();
	program.add_edge_weighted((1, 2), 2).unwrap();
	
	U32Analysis::analyze::<FifoWorklist<_>>(&program, &mut map);
	
	assert_eq!(U32(1), map[&0]);
	assert_eq!(U32(2), map[&1]);
	assert_eq!(U32(4), map[&2]);
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

impl<'a,G,L> Analysis<G,L> for SignAnalysis<'a>
	where
		G: EdgeWeightedGraph<EdgeWeight=Action>,
		G::Vertex: Hash,
		L: Bottom + SubLattice<StringSignTFSpace<'a>>
{
	type Lattice = StringSignTFSpace<'a>;
	const FORWARD: bool = true;
	
	
	fn transfer(init: &L, _: &L, acc: &Action) -> StringSignTFSpace<'a>
	{
		use self::Action::*;
		use self::Sign::*;
		let mut result = init.sub_lattice_ref().clone();
		match *acc {
			DeclareX => {
				result["x"] = SignPowerSet::from_iter(
					vec![Plus, Minus, Zero]
				);
			},
			IncX => {
				let x = if result.has_key("x"){ result["x"].clone()}else{SignPowerSet::bottom()};
				result["x"] =
					if x >= SignPowerSet::singleton(Minus){
						if x >= SignPowerSet::singleton(Zero){
							x + SignPowerSet::singleton(Plus)
						}else{
							x + SignPowerSet::singleton(Zero)
						}
					}else if x >= SignPowerSet::singleton(Zero){
						SignPowerSet::singleton(Plus)
					}else{
						x
					}
				;
			},
			YIsMinus1 => {
				result["y"] =SignPowerSet::singleton(Minus);
			},
			XIs0 => {
				result["x"] = SignPowerSet::singleton(Zero);
			},
			ReadY | DeclareY => {
				result["y"] = SignPowerSet::from_iter(
					vec![Plus, Minus, Zero]
				);
			},
			_ => (),
		}
		result
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
	
	let mut initial: HashMap<u32,StringSignTFSpace> = HashMap::new();
	initial.insert(0, StringSignTFSpace::bottom());
	
	SignAnalysis::analyze::<FifoWorklist<_>>(&g, &mut initial);
	
	let top = SignPowerSet::from_iter(vec![Plus,Minus,Zero]);
	let plus_zero = SignPowerSet::from_iter(vec![Plus, Zero]);
	let minus = SignPowerSet::singleton(Minus);
	let plus = SignPowerSet::singleton(Plus);
	
	assert_eq!(false, initial[&0].has_key("x"));	assert_eq!(false, initial[&0].has_key("y"));
	assert_eq!(plus_zero, initial[&1]["x"]);		assert_eq!(top, initial[&1]["y"]);
	assert_eq!(top, initial[&2]["x"]);				assert_eq!(top, initial[&2]["y"]);
	assert_eq!(top, initial[&3]["x"]);				assert_eq!(false, initial[&3].has_key("y"));
	assert_eq!(top, initial[&4]["x"]);				assert_eq!(minus, initial[&4]["y"]);
	assert_eq!(plus_zero, initial[&5]["x"]);		assert_eq!(top, initial[&5]["y"]);
	assert_eq!(plus_zero, initial[&6]["x"]);		assert_eq!(top, initial[&6]["y"]);
	assert_eq!(plus, initial[&7]["x"]);				assert_eq!(top, initial[&7]["y"]);
}

#[derive(Copy, Clone,PartialOrd, PartialEq,Debug)]
struct D32(U64, U32);

impl Bottom for D32
{
	fn bottom() -> Self
	{
		D32(U64::bottom(), U32::bottom())
	}
}

impl SubLattice<U32> for D32
{
	fn sub_lattice(self) -> U32
	{
		self.1
	}
	
	fn sub_lattice_ref(&self) -> &U32
	{
		&self.1
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut U32
	{
		&mut self.1
	}
}

impl SubLattice<U64> for D32
{
	fn sub_lattice(self) -> U64
	{
		self.0
	}
	
	fn sub_lattice_ref(&self) -> &U64
	{
		&self.0
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut U64
	{
		&mut self.0
	}
}

struct U64Analysis{}

impl<G,L> Analysis<G,L> for U64Analysis
	where
		G: EdgeWeightedGraph<EdgeWeight=u32>,
		G::Vertex: Hash,
		L: Bottom + SubLattice<U64> + SubLattice<U32>,
{
	type Lattice = U64;
	
	const FORWARD: bool = true;
	
	fn transfer(dependency: &L, target: &L, a: &G::EdgeWeight)
		-> Self::Lattice
	{
		let dep: &U64 = dependency.sub_lattice_ref();
		let tar: &U32 = target.sub_lattice_ref();
		
		U64(dep.0 + tar.0 as u64 + *a as u64)
	}
}

#[test]
fn solve_test_dependent()
{
	let mut map = HashMap::new();
	map.insert(0, D32(U64(0),U32(1)));
	
	let mut program = AdjListGraph::empty_graph();
	program.add_vertex(0).unwrap();
	program.add_vertex(1).unwrap();
	program.add_vertex(2).unwrap();
	
	program.add_edge_weighted((0, 1), 1).unwrap();
	program.add_edge_weighted((1, 2), 2).unwrap();
	
	U32Analysis::analyze::<FifoWorklist<_>>(&program, &mut map);
	U64Analysis::analyze::<FifoWorklist<_>>(&program, &mut map);
	
	assert_eq!(D32(U64(0),U32(1)), map[&0]);
	assert_eq!(D32(U64(3),U32(2)), map[&1]);
	assert_eq!(D32(U64(9),U32(4)), map[&2]);
	
}