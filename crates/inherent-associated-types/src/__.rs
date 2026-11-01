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

pub const fn obstruct_rustdoc(str: u128) -> u128 {
	str
}
