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
	let plusMinus = SignPowerSet::from(vec![Plus, Minus]);
	let plusZero = SignPowerSet::from(vec![Plus, Zero]);
	let minusZero = SignPowerSet::from(vec![Minus, Zero]);
	let plusMinusZero = SignPowerSet::from(vec![Plus, Minus, Zero]);
	
	// They are all equal to themselves
	let all = vec![empty.clone(),plus.clone(),zero.clone(),minus.clone(),plusMinus.clone(),plusZero.clone(),minusZero.clone(),plusMinusZero.clone()];
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
	
	assert!(plus < plusMinus);
	assert!(plus < plusZero);
	assert!(minus < plusMinus);
	assert!(minus < minusZero);
	assert!(zero < plusZero);
	assert!(zero < minusZero);
	assert!(!(plus < minusZero));
	assert!(!(minus < plusZero));
	assert!(!(zero < plusMinus));
}