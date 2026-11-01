pub use core::{concat, stringify};

pub use inherent_associated_types_proc_macro_definitions::associated_impl;

pub use crate::Associated;

pub trait Associated0 {
	type Associated;
}

pub trait Associated1<const _0: u128> {
	type Associated;
}

pub trait Associated2<const _0: u128, const _1: u128> {
	type Associated;
}

pub trait Associated3<const _0: u128, const _1: u128, const _2: u128> {
	type Associated;
}

pub trait Associated4<const _0: u128, const _1: u128, const _2: u128, const _3: u128> {
	type Associated;
}

pub const fn obstruct_rustdoc(str: u128) -> u128 {
	str
}
