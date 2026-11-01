#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use std::io::Write;

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
/// `impl` generics ok. Use `#[associate(crate = …)]` if aliased.
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

fn associated(root: &TokenStream, ident: &Ident, spacer_span: Span) -> TokenStream {
	let ident_str = &*ident.to_string();
	let ident_span = ident.span().resolved_at(Span::mixed_site());
	let spacer_span = spacer_span.resolved_at(Span::mixed_site());
	const PART_SIZE: usize = size_of::<u128>();
	let mut key = vec![0_u8; ident_str.len().div_ceil(PART_SIZE) * PART_SIZE];
	if key.len() / PART_SIZE > 5 {
		panic!(
			"Identifiers of length {} (> {}) aren't supported yet.",
			key.len(),
			PART_SIZE * 5
		)
	}
	key.as_mut_slice()
		.write_all(ident_str.as_bytes())
		.expect("fits");
	let (key, []) = key.as_chunks() else {
		unreachable!()
	};
	let mut key = key.into_iter().copied().map(u128::from_le_bytes); // For consistency, and because little endian often produces smaller numbers here.
	let mut output = quote_spanned! {ident_span=> Associated<};
	if let Some(a0) = key.next() {
		quote_spanned! {ident_span=> {#root::__::obstruct_rustdoc(#a0)}}.to_tokens(&mut output);
	}
	for a in key {
		quote_spanned! {spacer_span=> , {#root::__::obstruct_rustdoc(#a)}}.to_tokens(&mut output);
	}
	quote_spanned! {ident_span=> >}.to_tokens(&mut output);
	output
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
