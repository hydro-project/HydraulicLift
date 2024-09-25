use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, Ident, Stmt};

#[derive(Clone)]
pub enum RawSegment {
    Await(Vec<Stmt>),
    End(Vec<Stmt>),
}

impl RawSegment {
    pub fn into_block(self) -> Block {
        Block {
            brace_token: Default::default(),
            stmts: match self {
                Self::Await(v) => v,
                Self::End(v) => v,
            },
        }
    }

    pub fn into_segment(self, inputs: Vec<Ident>, outputs: Vec<Ident>) -> Segment {
        match self {
            Self::Await(v) => Segment::Await(inputs, v, outputs),
            Self::End(v) => Segment::End(inputs, v),
        }
    }
}

pub enum Segment {
    Await(Vec<Ident>, Vec<Stmt>, Vec<Ident>),
    End(Vec<Ident>, Vec<Stmt>),
}

impl ToTokens for Segment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Await(inputs, statements, outputs) => quote! {
                poll_future(
                    async |#(#inputs),*| {
                        #(#statements)*
                        (#(#outputs),*)
                    }
                ); // ; should be ->
            },
            Self::End(inputs, statements) => quote! {
                map(|#(#inputs),*| {
                    #(#statements)*
                });
            },
        });
    }
}
