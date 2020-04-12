
use crate::core::CompleteLattice;

pub trait SubLattice<L>
	where L: CompleteLattice
{
	fn sub_lattice(self) -> L;
	
	fn sub_lattice_ref(&self) -> &L;
	
	fn sub_lattice_ref_mut(&mut self) -> &mut L;
}

impl<L> SubLattice<L> for L
	where L: CompleteLattice
{
	fn sub_lattice(self) -> L
	{
		self
	}
	
	fn sub_lattice_ref(&self) -> &L
	{
		self
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut L
	{
		self
	}
}