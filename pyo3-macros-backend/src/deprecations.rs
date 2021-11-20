use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};

pub enum Deprecation {
    CallAttribute,
}

impl Deprecation {
    fn ident(&self, span: Span) -> syn::Ident {
        let string = match self {
            Deprecation::CallAttribute => "CALL_ATTRIBUTE",
        };
        syn::Ident::new(string, span)
    }
}

#[derive(Default)]
pub struct Deprecations(Vec<(Deprecation, Span)>);

impl Deprecations {
    pub fn new() -> Self {
        Deprecations(Vec::new())
    }

    pub fn push(&mut self, deprecation: Deprecation, span: Span) {
        self.0.push((deprecation, span))
    }
}

impl ToTokens for Deprecations {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (deprecation, span) in &self.0 {
            let ident = deprecation.ident(*span);
            quote_spanned!(
                *span =>
                let _ = ::pyo3::impl_::deprecations::#ident;
            )
            .to_tokens(tokens)
        }
    }
}
