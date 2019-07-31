extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Token, Attribute, Visibility, Field, Ident, Generics, parse, parenthesized};

struct ItemNewType {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub type_token: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub inner: Field,
    pub semi: Option<Token![;]>
}

impl parse::Parse for ItemNewType {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let type_token = input.parse()?;
        let ident = input.parse()?;
        let generics = input.parse()?;
        let inner = {
            let content;
            parenthesized!(content in input);
            content.call(Field::parse_unnamed)?
        };
        let where_clause = input.parse()?;
        let semi = input.parse()?;

        Ok(ItemNewType {
            attrs,
            vis,
            type_token,
            ident,
            generics: Generics {
                where_clause,
                .. generics
            },
            inner,
            semi
        })
    }
}

/// Creates a new wrapper type. This type is transparent and implements [`Wrapper`](trait.Wrapper.html)
/// 
/// # Examples
/// 
/// ```ignore
/// use trapper::newtype;
/// 
/// newtype!(type BasicNumber(i32));
/// newtype!(pub type WithVisibility(i32));
/// newtype!(pub type WithLifetimes<'a>(std::io::StderrLock<'a>));
/// newtype!(pub type WithTypeParameters<T>(T));
/// newtype!(pub type WithBoth<'a, T>(&'a T));
/// newtype!(pub type WithClause<'a, T>(&'a T) where T: Default);
/// newtype! {
///     /// a summary
///     pub type WithAttributes(i32);
/// }
/// # fn main() { }
/// ```
#[proc_macro]
pub fn newtype(item: TokenStream) -> TokenStream {
    let ItemNewType { 
        attrs: attributes,
        vis,
        ident: name,
        inner,
        generics,
        ..
    } = syn::parse_macro_input!(item as ItemNewType);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let def = quote! {
        #(#attributes
        )*
        #[repr(transparent)]
        #vis struct #name #generics (#inner) #where_clause;
        unsafe impl #impl_generics trapper::Wrapper for #name #type_generics #where_clause {
            type Inner = #inner;

            fn wrap(inner: Self::Inner) -> Self { Self(inner) }
            fn unwrap(self) -> Self::Inner { self.0 }
        }
    };

    TokenStream::from(def)
}
