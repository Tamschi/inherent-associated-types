#![warn(clippy::pedantic)]

use std::io::Write;

use bytemuck::cast_slice_mut;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote_spanned};
use syn::{
	Ident, ImplItem, ImplItemType, ItemImpl, Type, TypeGroup, TypeModifiers,
	parse::{Parse, ParseStream},
	parse_macro_input, parse_quote_spanned,
	token::{Crate, Eq, Group},
};

/// Use on `impl Type` with **only** `pub type` items.\
/// `impl` generics ok.\
/// Use `#[associate(crate = …)]` if aliased.
#[proc_macro_attribute]
pub fn associate(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
	let root = if attr.is_empty() {
		quote_spanned! {Span::mixed_site()=> inherent_associated_types}
	} else {
		parse_macro_input!(attr as CustomRootAttr).to_token_stream()
	};

	let ItemImpl {
		attrs,
		modifiers: syn::ImplModifiers {
			defaultness,
			polarity,
			..
		},
		unsafety,
		impl_token,
		generics,
		trait_,
		self_ty,
		brace_token,
		items,
	} = parse_macro_input!(item as ItemImpl);
	assert!(trait_.is_none(), "Only inherent impls are supported.");

	let mut output = TokenStream::new();

	for item in items {
		match item {
			ImplItem::Type(ImplItemType {
				attrs: inner_attrs,
				vis,
				modifiers: TypeModifiers {
					defaultness: inner_defaultness,
					..
				},
				type_token,
				ident,
				generics: inner_generics,
				eq_token,
				ty,
				semi_token,
			}) => {
				assert!(
					inner_generics.lt_token.is_none(),
					"Generic associated types are currently not supported."
				);
				assert_eq!(
					vis.to_token_stream().to_string(),
					"pub",
					"Only public associated types are supported."
				);
				let associated = associated(&root, &ident, eq_token.span);
				let mut body = TokenStream::new();
				brace_token.surround(&mut body, |body| {
					quote_spanned! {Span::mixed_site()=>
						#inner_defaultness #type_token Associated #eq_token #ty #semi_token
					}
					.to_tokens(body);
				});
				quote_spanned! {Span::mixed_site()=>
					#(#attrs)* #(#inner_attrs)*
					#[automatically_derived]
					// Acts as spacer for attached docs.
					///
					#[doc = #root::__::concat!(
						"<sub><code>[Associated](",
						#root::__::stringify!(#root),
						"::__::Associated!)!(",
						#root::__::stringify!(#self_ty),
						"\\[",
						#root::__::stringify!(#ident),
						"\\])",
						"</code></sub>")]
					#defaultness #unsafety #impl_token #generics #polarity #root::__::#associated for #self_ty #body
				}
			}
			.to_tokens(&mut output),
			_ => panic!("Only type items are supported for now."),
		}
	}
	output.into()
}

struct CustomRootAttr {
	root: TokenStream,
}

impl Parse for CustomRootAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			root: {
				let _: Crate = input.parse()?;
				let _: Eq = input.parse()?;
				input.parse()?
			},
		})
	}
}

impl ToTokens for CustomRootAttr {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.root.to_tokens(tokens);
	}
}

fn associated(root: &TokenStream, ident: &Ident, symbol_span: Span) -> TokenStream {
	let ident_str = &*ident.to_string();
	let ident_span = ident.span().resolved_at(Span::mixed_site());
	let symbol_span = symbol_span.resolved_at(Span::mixed_site());
	match ident_str.len() {
		0 => quote_spanned!(ident.span()=> Associated<>),
		n if n < std::mem::size_of::<u128>() => {
			let mut args = [0_u128; 1];
			cast_slice_mut::<_, u8>(&mut args)
				.write_all(ident_str.as_bytes())
				.expect("fits");
			let [a0] = args.map(u128::from_le);
			let a0 = quote_spanned!(ident_span=> {#root::__::obstruct_rustdoc(#a0)});
			quote_spanned!(ident_span=> Associated<#a0>)
		}
		n if n <= 2 * std::mem::size_of::<u128>() => {
			let mut args = [0_u128; 2];
			cast_slice_mut::<_, u8>(&mut args)
				.write_all(ident_str.as_bytes())
				.expect("fits");
			let [a0, a1] = args.map(u128::from_le);
			let a0 = quote_spanned!(ident_span=> {#root::__::obstruct_rustdoc(#a0)});
			let a1 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a1)});
			quote_spanned!(ident_span=> Associated<#a0, #a1>)
		}
		n if n <= 3 * std::mem::size_of::<u128>() => {
			let mut args = [0_u128; 3];
			cast_slice_mut::<_, u8>(&mut args)
				.write_all(ident_str.as_bytes())
				.expect("fits");
			let [a0, a1, a2] = args.map(u128::from_le);
			let a0 = quote_spanned!(ident_span=> {#root::__::obstruct_rustdoc(#a0)});
			let a1 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a1)});
			let a2 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a2)});
			quote_spanned!(ident_span=> Associated<#a0, #a1, #a2>)
		}
		n if n <= 5 * std::mem::size_of::<u128>() => {
			let mut args = [0_u128; 5];
			cast_slice_mut::<_, u8>(&mut args)
				.write_all(ident_str.as_bytes())
				.expect("fits");
			let [a0, a1, a2, a3, a4] = args.map(u128::from_le);
			let a0 = quote_spanned!(ident_span=> {#root::__::obstruct_rustdoc(#a0)});
			let a1 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a1)});
			let a2 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a2)});
			let a3 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a3)});
			let a4 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a4)});
			quote_spanned!(ident_span=> Associated<#a0, #a1, #a2, #a3, #a4>)
		}
		n if n <= 4 * std::mem::size_of::<u128>() => {
			let mut args = [0_u128; 4];
			cast_slice_mut::<_, u8>(&mut args)
				.write_all(ident_str.as_bytes())
				.expect("fits");
			let [a0, a1, a2, a3] = args.map(u128::from_le);
			let a0 = quote_spanned!(ident_span=> {#root::__::obstruct_rustdoc(#a0)});
			let a1 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a1)});
			let a2 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a2)});
			let a3 = quote_spanned!(symbol_span=> {#root::__::obstruct_rustdoc(#a3)});
			quote_spanned!(ident_span=> Associate4<#a0, #a1, #a2, #a3>)
		}
		n => panic!("Identifiers of length {} aren't supported yet.", n),
	}
}

#[proc_macro]
pub fn associated_impl(input: TokenStream1) -> TokenStream1 {
	parse_macro_input!(input as Associated)
		.to_token_stream()
		.into()
}

struct Associated {
	root: TokenTree,
	host: Type,
	ident: Ident,
}

impl Parse for Associated {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			root: input.parse()?,
			host: input.parse()?,
			ident: input.parse()?,
		})
	}
}

impl ToTokens for Associated {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Associated { root, host, ident } = self;
		TypeGroup {
			attrs: vec![],
			group_token: Group {
				span: Span::mixed_site(),
			},
			elem: Box::new({
				let associated = associated(
					&root.into_token_stream(),
					ident,
					ident.span(), // The span doesn't matter here, I think.
				);
				parse_quote_spanned! {Span::mixed_site()=>
					<#host as #root::__::#associated>::Associated
				}
			}),
		}
		.to_tokens(tokens)
	}
}
