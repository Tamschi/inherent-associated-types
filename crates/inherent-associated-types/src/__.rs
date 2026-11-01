pub use core::{concat, stringify};

pub use inherent_associated_types_proc_macro_definitions::associated_impl;

pub use crate::Associated;

pub trait Associate0 {
	type Associated;
}

pub trait Associate<const _0: u128> {
	type Associated;
}

pub trait Associate2<const _0: u128, const _1: u128> {
	type Associated;
}

pub trait Associate3<const _0: u128, const _1: u128, const _2: u128> {
	type Associated;
}

pub trait Associate4<const _0: u128, const _1: u128, const _2: u128, const _3: u128> {
	type Associated;
}

/// Rustdoc erases all function bodies, so it can't compute
/// the actual value of the `const` generic argument. As such
/// it instead seems to grab whatever the span location of the
/// curly braces points at, which is easily manipulated to be
/// the identifier of the associated type in the macro input.
pub const fn obstruct_rustdoc(str: u128) -> u128 {
	str
}
