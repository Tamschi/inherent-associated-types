#![no_std]

#[doc(hidden)]
pub mod __;

pub use inherent_associated_types_proc_macro_definitions::associate;

/// Type macro: <code>[Associated]!(Host\[Associated\])</code>.\
/// Supports <code>[Associated]!(Host\[A\]\[B\]…)</code> shorthand.
#[macro_export]
macro_rules! Associated {
	($ty:ty[$ident:ident]$([$rest:ident])+) => (
		$crate::Associated!($crate::__::associated_impl!($crate $ty $ident)$([$rest])+)
	);
	($ty:ty[$ident:ident]) => (
		$crate::__::associated_impl!($crate $ty $ident)
	);
}
