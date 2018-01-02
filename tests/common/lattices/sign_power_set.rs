use super::*;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::BitOr;
use std::iter::FromIterator;
use std::cmp::Ordering;
use progysis::common::lattices::{HashPowerSet};
use ::common::lattices::sign_power_set::Sign::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sign{
	Plus,
	Zero,
	Minus
}

type SignPowerSet = HashPowerSet<Sign>;


#[test]
fn comparison_test(){
	let empty = SignPowerSet::bottom();
	let plus = SignPowerSet::singleton(Plus);
	let zero = SignPowerSet::singleton(Zero);
	let minus = SignPowerSet::singleton(Minus);
	let plus_minus = SignPowerSet::from(vec![Plus, Minus]);
	let plus_zero = SignPowerSet::from(vec![Plus, Zero]);
	let minus_zero = SignPowerSet::from(vec![Minus, Zero]);
	let plus_minus_zero = SignPowerSet::from(vec![Plus, Minus, Zero]);
	
	// They are all equal to themselves
	let all = vec![empty.clone(), plus.clone(), zero.clone(), minus.clone(), plus_minus.clone(), plus_zero.clone(), minus_zero.clone(), plus_minus_zero.clone()];
	for e in all.clone() {
		assert!(e == e, "{:?}",e);
	}
	
	// They are all mutually unequal
	for (i,e1) in all.clone().into_iter().enumerate(){
		for (j,e2) in all.clone().into_iter().enumerate(){
			if i != j {
				assert!(e1 != e2, "{:?} == {:?}",e1, e2);
			}
		}
	}
	
	// The empty element is less than all else
	let first = all[0].clone();
	for (i,e) in all.clone().into_iter().enumerate(){
		if i>0 {
			assert!(first < e, "{:?} >= {:?}",first, e);
		}
	}
	
	// plusMinuZero element is larger than all else
	let last = all[all.len()-1].clone();
	for (i,e) in all.clone().into_iter().enumerate(){
		if i < (all.len()-1) {
			assert!(last > e, "{:?} =< {:?}", last, e);
		}
	}
	
	assert!(plus < plus_minus);
	assert!(plus < plus_zero);
	assert!(minus < plus_minus);
	assert!(minus < minus_zero);
	assert!(zero < plus_zero);
	assert!(zero < minus_zero);
	assert!(!(plus < minus_zero));
	assert!(!(minus < plus_zero));
	assert!(!(zero < plus_minus));
}

#[test]
fn addition_test(){
	let empty = SignPowerSet::bottom();
	let plus = SignPowerSet::singleton(Plus);
	let zero = SignPowerSet::singleton(Zero);
	let minus = SignPowerSet::singleton(Minus);
	let plus_minus = SignPowerSet::from(vec![Plus, Minus]);
	let plus_zero = SignPowerSet::from(vec![Plus, Zero]);
	let minus_zero = SignPowerSet::from(vec![Minus, Zero]);
	let plus_minus_zero = SignPowerSet::from(vec![Plus, Minus, Zero]);
	let all = vec![empty.clone(), plus.clone(), zero.clone(), minus.clone(), plus_minus.clone(), plus_zero.clone(), minus_zero.clone(), plus_minus_zero.clone()];
	
	// Adding an element to itself does not change it.
	for e in all.clone() {
		assert!(e.clone() + e.clone() == e.clone(), "{:?}",e);
	}
	
	// Adding the empty element to any other element, results in the other element
	let first = all[0].clone();
	for (i,e) in all.clone().into_iter().enumerate(){
		if i>0 {
			assert!(first.clone() + e.clone() == e, "{:?} + {:?} != {:?}",first, e, e);
		}
	}
	
	// Adding the top element to any other element, results in the top element
	let last = all[all.len()-1].clone();
	for (i,e) in all.clone().into_iter().enumerate(){
		if i < (all.len()-1) {
			assert!(last.clone() + e.clone() == last.clone(), "{:?} + {:?} != {:?}",last, e, last);
		}
	}
	
	assert_eq!(plus.clone() + minus.clone(),  plus_minus.clone());
	assert_eq!(plus.clone() + zero.clone(),  plus_zero.clone());
	assert_eq!(plus.clone() + plus_minus.clone(),  plus_minus.clone());
	assert_eq!(plus.clone() + plus_zero.clone(),  plus_zero.clone());
	assert_eq!(plus.clone() + minus_zero.clone(),  plus_minus_zero.clone());
	
	assert_eq!(minus.clone() + plus.clone(),  plus_minus.clone());
	assert_eq!(minus.clone() + zero.clone(),  minus_zero.clone());
	assert_eq!(minus.clone() + plus_minus.clone(),  plus_minus.clone());
	assert_eq!(minus.clone() + plus_zero.clone(),  plus_minus_zero.clone());
	assert_eq!(minus.clone() + minus_zero.clone(),  minus_zero.clone());
	
	assert_eq!(zero.clone() + plus.clone(),  plus_zero.clone());
	assert_eq!(zero.clone() + minus.clone(),  minus_zero.clone());
	assert_eq!(zero.clone() + plus_minus.clone(),  plus_minus_zero.clone());
	assert_eq!(zero.clone() + plus_zero.clone(),  plus_zero.clone());
	assert_eq!(zero.clone() + minus_zero.clone(),  minus_zero.clone());
}