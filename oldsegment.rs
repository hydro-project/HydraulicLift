use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, Ident, Stmt};

#[derive(Clone)]
pub enum RawSegment {
    Await(Vec<Stmt>),
    End(Vec<Stmt>),
}

impl RawSegment {
    pub fn vec(&self) -> &Vec<Stmt> {
        match self {
            Self::Await(v) => v,
            Self::End(v) => v
        }
    }

    pub fn into_vec(self) -> Vec<Stmt> {
        match self {
            Self::Await(v) => v,
            Self::End(v) => v
        }
    }

    pub fn into_block(self) -> Block {
        Block {
            brace_token: Default::default(),
            stmts: self.into_vec(),
        }
    }
    
    pub fn into_segment(self, inputs: Vec<Ident>, outputs: Vec<Ident>) -> Segment {
        if matches!(self, RawSegment::End(_)) {
            Segment::end_from_raw(self, inputs)
        } else {
            Segment::inner_from_raw(self, inputs, outputs)
        }
    }
}

impl ToTokens for RawSegment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vec = self.vec();
        tokens.extend(quote! {
            #(#vec)*
        });
    }
}

/// A raw segment paired with it's inputs&outputs
pub enum Segment {
    Inner(Vec<Ident>, RawSegment, Vec<Ident>),
    End(Vec<Ident>, RawSegment),
}

impl Segment {
    pub fn inner_from_raw(raw: RawSegment, inputs: Vec<Ident>, outputs: Vec<Ident>) -> Segment {
        Segment::Inner(inputs, raw, outputs)
    }

    pub fn end_from_raw(raw: RawSegment, inputs: Vec<Ident>) -> Segment {
        Segment::End(inputs, raw)
    }
}

impl ToTokens for Segment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Inner(inputs, raw, outputs) => quote! {
                poll_future(
                    async |#(#inputs),*| {
                        #raw
                        (#(#outputs),*)
                    }
                ); // ; should be ->
            },
            Self::End(inputs, raw) => quote! {
                map(|#(#inputs),*| {
                    #raw
                });
            },
        });
    }
}
