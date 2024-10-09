use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{token, Token};

use crate::rast::*;

// TODO: this should be run on the i/o annotated version of the tree, not the H versions

impl ToTokens for RExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RExpr::If(s)    => s.to_tokens(tokens),
            RExpr::Block(s) => s.to_tokens(tokens),
            RExpr::Raw(s)   => s.to_tokens(tokens),
        }
    }
}

impl ToTokens for RExprIf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { condition, then_block, else_block } = self;
        tokens.extend(quote! {
            if #condition #then_block
        });
        if let Some(expr) = else_block {
            expr.to_tokens(tokens);
        }
    }
}

impl ToTokens for RExprBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { statements } = self;
        tokens.extend(quote! {
            {
                #(#statements)*
            }
        });
    }
}

impl ToTokens for RStmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RStmt::LetAwait(s)  => s.to_tokens(tokens),
            RStmt::Return(s)    => s.to_tokens(tokens),
            RStmt::Expr(s)      => s.to_tokens(tokens),
            RStmt::Raw(s)       => s.to_tokens(tokens),
        }
    }
}

impl ToTokens for RStmtLetAwait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { definition, future } = self;
        tokens.extend(quote! {
            let #definition = #future.await;
        });
    }
}

impl ToTokens for RReturn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { value } = self;
        let value_toks = match value {
            Some(expr) => expr.to_token_stream(),
            None => quote! {()},
        };
        tokens.extend(quote! {
            return #value_toks;
        });
    }
}