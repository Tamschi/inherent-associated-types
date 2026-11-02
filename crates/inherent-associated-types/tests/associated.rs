use std::marker::PhantomData;

use ::inherent_associated_types::{Associated, associate};
mod inherent_associated_types {}

pub enum Host {}
pub struct GenericHost<T>(PhantomData<T>);
pub struct Associated;

use ::inherent_associated_types as iat;
#[iat::associate(crate = iat)]
impl Host {
	pub type Associated = Associated;
}

#[associate]
impl<T> GenericHost<T> {
	pub type Associated______________________________________________________________________ =
		Host;
}

const _: Associated!(GenericHost<()>[Associated______________________________________________________________________][Associated]) =
	Associated;
const _: Associated!(
	Associated!(GenericHost<()>[Associated______________________________________________________________________])
		[Associated]
) = Associated;
