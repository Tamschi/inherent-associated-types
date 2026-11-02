#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![no_std]

//! Emulates&nbsp;[`inherent_associated_types`](https://github.com/rust-lang/rust/issues/8995)&nbsp;in&nbsp;stable&nbsp;Rust
//! (in&nbsp;a&nbsp;way&nbsp;that&nbsp;looks&nbsp;clean&nbsp;in&nbsp;both&nbsp;code&nbsp;and&nbsp;documentation).
//!
//! ```rust
//! use inherent_associated_types::{Associated, associate};
//!
//! struct Host;
//! struct A<T>(T);
//! struct B;
//!
//! #[associate]
//! impl Host {
//! 	pub type HostA = A<()>;
//! 	pub type HostB = B;
//! }
//!
//! #[associate]
//! impl<T> A<T> {
//! 	/// You can document and use attributes.
//! 	pub type AA = A<T>;
//! 	pub type AB = B;
//! }
//!
//! const _: Associated!(Host[HostA][AA][AB]) = B;
//! const _: Associated!(Associated!(Host[HostA])[AB]) = B;
//! ```
//!
//! ## Features
//!
//! - Clean generated documentation! For example, from the above:
//!
//!   <fieldset>
//!   <section class=impl>
//!   <h3 class=code-header>impl&lt;T> Associated&lt;AA> for <span class=struct>A</span>&lt;T></h3>
//!   <div class=docblock><p>You can document and use attributes.<br>
//!   <sub><code><a>Associated</a>!(A < T >[AA])</code></sub>
//!   </p></div></section>
//!   <div class=impl-items><section class="associatedtype trait-impl">
//!   <h4 class=code-header>type <span class=associatedtype>Associated</span> = <span class=struct>A</span>&lt;T></h4>
//!   </section></div>
//!   </fieldset>
//!
//!   which is listed as "<a>Associated&lt;AA></a>" in the sidebar.
//!
//! - 100% hygienic bar (iff not specified) crate root used in the attribute macro.
//!
//! ## Limitations
//!
//! - Associated types must be (as) public (as their host).
//! - Each associated type identifier is limited to about 80 Latin or 26 CJK characters.
//! - [As in the RFC](https://github.com/rust-lang/rfcs/blob/3bdcd22feea6848a7920fa06d17ad4c6ada9db89/text/0195-associated-items.md#inherent-associated-items),
//!   the (presence of an) inherent associated type can't <!-- sensibly --> be used as bound.
//!
//! ## Known issues
//!
//! - The errors in response to invalid macro input are a little unpolished,\
//!   and currently they stop the macro outright instead of recovering where possible.
//! - The generated documentation contains extra spaces if the host type has generics.
//! - The error reported for missing associated types shatters the illusion (and can be a bit long).\
//!   Similarly, identifiers longer than 16 bytes in UTF-8 cause spacers to appear in the documentation.
//! - Generic associated types (GATs) are currently not supported.
//!
//! <!-- This was going to be a meme crate, but it accidentally went really well. Please enjoy! -->
//! <!-- No AI was involved in this project. -->

#[doc(hidden)]
pub mod __;

// #[doc(inline)] // Not *technically* necessary, I think.
/// Use on `impl Type` with **only** `pub type` items.\
/// `impl` generics ok. Use `#[associate(crate = …)]` if aliased.
///
/// Types registered through this macro can be resolved through [`Associated!`](`crate::Associated!`).\
/// (A small usage example with link to that macro is attached in the host's documentation.)
pub use inherent_associated_types_proc_macro_definitions::associate;

/// Type macro: <code>[Associated]!(Host\[Associated\])</code>.\
/// Supports <code>[Associated]!(Host\[A\]\[B\]…)</code> shorthand.
///
/// Use this macro to resolve types registered through [`#[associate]`](`associate`).
#[macro_export]
macro_rules! Associated {
	($ty:ty[$ident:ident]$([$rest:ident])+) => (
		$crate::Associated!($crate::__::associated_impl!($crate $ty $ident)$([$rest])+)
	);
	($ty:ty[$ident:ident]) => (
		$crate::__::associated_impl!($crate $ty $ident)
	);
}
