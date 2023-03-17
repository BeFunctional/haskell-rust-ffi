//! Macro for deriving `HaskellSize` instances for structs
//!
//! Implementation is adapted from the `heapsize` example in the `syn` crate.
//! The implementation is not identical, however: `haskell_size` does not take
//! any value as input, but is entirely type-based.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Iter, Data, DeriveInput, Field, Fields,
    GenericParam, Generics,
};

/// Derive `HaskellSize` instance
///
/// NOTE: Only structs are currently supported.
#[proc_macro_derive(HaskellSize)]
pub fn haskell_size_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = &input.ident;

    // Add a bound `T: HaskellSize` to every type parameter T.
    let without_tag: Generics = add_trait_bounds(input.generics);

    // The instance itself must get an additional `Tag` argument
    //
    // NOTE: Things will go badly if one of the user's parameters is also named `Tag`.
    let mut including_tag: Generics = without_tag.clone();
    including_tag
        .params
        .push(GenericParam::Type(parse_quote!(Tag)));

    let (including_tag_impl, _, _) = including_tag.split_for_impl();
    let (_, without_tag_tys, without_tag_where) = without_tag.split_for_impl();

    // Generate an expression to sum up the size of each field.
    let sum = haskell_size_sum(&input.data);

    let expanded = quote! {
        impl #including_tag_impl HaskellSize<Tag> for #name #without_tag_tys #without_tag_where {
            fn haskell_size(tag: PhantomData<Tag>) -> usize {
                #sum
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

/// Add a bound `T: HaskellSize<Tag>` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(HaskellSize<Tag>));
        }
    }
    generics
}

/// Generate an expression to sum up the size of each field.
fn haskell_size_sum(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => haskell_size_fields(fields.named.iter()),
            Fields::Unnamed(fields) => haskell_size_fields(fields.unnamed.iter()),
            Fields::Unit => quote!(0),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

/// Auxiliary to `haskell_size_sum`
fn haskell_size_fields(fields: Iter<Field>) -> TokenStream {
    let recurse = fields.map(|f| {
        let t = &f.ty;
        quote! { <#t> :: haskell_size(tag) }
    });
    quote! {
        0 #(+ #recurse)*
    }
}
