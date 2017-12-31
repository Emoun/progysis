use super::*;

pub trait Evaluable
{
	type Value: CompleteLattice;
	
	fn evaluate(&self) -> Self::Value;
}

