use std::marker::PhantomData;

use inherent_associated_types::{Associated, associate};

pub enum Host {}
pub struct GenericHost<T>(PhantomData<T>);
pub struct Associated;

#[associate(crate = inherent_associated_types)]
impl Host {
	/// And there's documentation here.
	pub type Associated = Associated;
}

#[associate]
impl<T> GenericHost<T> {
	/// There's documentation here.
	pub type Associatedaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa =
		Host;
}

const _: Associated!(GenericHost<()>[Associatedaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa][Associated]) =
	Associated;
