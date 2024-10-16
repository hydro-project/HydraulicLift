use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{asta::*, debugutil::DebugStr};

// TODO: this should be run on the H versions, not the R versions

impl ToTokens for AExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AExpr::If(s)            => s.to_tokens(tokens),
            AExpr::Block(s)         => s.to_tokens(tokens),
            AExpr::Raw(DebugStr(s)) => s.to_tokens(tokens),
        }
    }
}

impl ToTokens for AExprIf {
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

impl ToTokens for AExprBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { statements } = self;
        tokens.extend(quote! {
            {
                #(#statements)*
            }
        });
    }
}

impl ToTokens for AStmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AStmt::LetAwait(s)      => s.to_tokens(tokens),
            AStmt::Return(s)        => s.to_tokens(tokens),
            AStmt::Expr(s)          => s.to_tokens(tokens),
            AStmt::Raw(DebugStr(s)) => s.to_tokens(tokens),
        }
    }
}

impl ToTokens for AStmtLetAwait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { definition: DebugStr(definition), future } = self;
        tokens.extend(quote! {
            let #definition = #future.await;
        });
    }
}

impl ToTokens for AReturn {
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