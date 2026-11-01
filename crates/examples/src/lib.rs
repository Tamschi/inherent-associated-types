use std::marker::PhantomData;

use inherent_associated_types::{Associated, associate};

pub enum Host {}
pub struct GenericHost<T>(PhantomData<T>);
pub struct Associated;

#[associate(crate = inherent_associated_types)]
impl Host {
	pub type Associated = Associated;
}

#[associate]
impl<T> GenericHost<T> {
	pub type Associatedaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa = Host;
}

const _: Associated!(GenericHost<()>[Associatedaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa][Associated]) =
	Associated;
