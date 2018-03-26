use super::*;

use std::panic::catch_unwind;
use progysis::common::lattices::HashTFSpace;

pub type StringSignTFSpace<'a> = HashTFSpace<'a, &'a str, SignPowerSet>;

#[test]
fn initialization_test(){
	let mut f = Element::<StringSignTFSpace>::bottom();
	assert!(f.is_bottom());
}

#[test]
fn add_key_test(){
	let mut f =  Element::<StringSignTFSpace>::bottom();
	assert!( catch_unwind(|| &f["v1"]).is_err());
	f.add_key("v1");
	assert_eq!( f["v1"], Element::bottom(), "\nf: {:?}", f);
}

#[test]
fn comparison_test(){
	let mut f1 = Element::<StringSignTFSpace>::bottom();
	let mut f2 = Element::<StringSignTFSpace>::bottom();
	f1.add_key("v1");
	f2.add_key_with("v1", Element::singleton(Sign::Zero));
	assert!( f1 < f2, "{:?} >= {:?}", f1, f2 );
}

#[test]
fn add_test(){
	let mut f1 = Element::<StringSignTFSpace>::bottom();
	let mut f2 = Element::<StringSignTFSpace>::bottom();
	f1.add_key("v1");
	f2.add_key_with("v1", Element::singleton(Sign::Zero));
	let f3 = f1 + f2;
	assert_eq!( f3["v1"], Element::singleton(Sign::Zero), "\nf3: {:?}", f3);
}

#[test]
fn add_assign_test(){
	let mut f1 = Element::<StringSignTFSpace>::bottom();
	let mut f2 = Element::<StringSignTFSpace>::bottom();
	f1.add_key("v1");
	f2.add_key_with("v1", Element::singleton(Sign::Zero));
	let f3 = f1.clone() + f2.clone();
	f1 += f2;
	assert_eq!( f3, f1, "{:?} != {:?}", f3, f1);
}
