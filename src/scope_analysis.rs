use syn::Expr;

use crate::{io::{Scope, IO}, r_ast::*, utils::{ident, DebugStr, Tagged}};


impl RExpr<()> {
    pub fn tag(self) -> RExpr<IO> {
        match self {
            RExpr::If(s) => RExpr::If(s.tag()),
            RExpr::Block(s) => RExpr::Block(s.tag()),
            RExpr::Raw(s) => RExpr::Raw(s.tag()),
        }
    }
}

impl RExprBlock<()> {
    pub fn tag(self) -> RExprBlock<IO> {
        let Self { stmt, box return_expr } = self;
        RExprBlock { stmt: stmt.tag(), return_expr: Box::new(return_expr.tag()) }
    }
}

impl RExprIf<()> {
    pub fn tag(self) -> RExprIf<IO> {
        let Self { box cond_expr, box then_expr, box else_expr } = self;
        RExprIf { cond_expr: Box::new(cond_expr.tag()), then_expr: Box::new(then_expr.tag()), else_expr: Box::new(else_expr.tag()) }
    }
}

impl Tagged<DebugStr<Expr>, ()> {
    pub fn tag(self) -> Tagged<DebugStr<Expr>, IO> {
        let Self(inner, ()) = self;
        Tagged(inner, IO { input_scope: Scope::empty(), output_scope: Scope::empty() }) 
        //TODO: actually implement this
    }
}

impl RStmt<()> {
    pub fn tag(self) -> RStmt<IO> {
        match self {
            RStmt::Let(s) => RStmt::Let(s.tag()),
            RStmt::Return(s) => RStmt::Return(s.tag()),
        }
    }
}

impl Tagged<RStmtLet<()>, ()> {
    pub fn tag(self) -> Tagged<RStmtLet<IO>, IO> {
        let Tagged(RStmtLet { ident, box value }, ()) = self;
        let ident_clone = ident.clone();
        Tagged(RStmtLet { ident, value: Box::new(value.tag()) }, IO { input_scope: Scope::empty(), output_scope: Scope::empty().with(ident_clone) })
    }
}

impl RStmtReturn<()> {
    pub fn tag(self) -> RStmtReturn<IO> {
        let RStmtReturn { box value } = self;
        RStmtReturn { value: Box::new(value.tag()) }
    }
}



// use std::collections::BTreeSet;

// use syn::Ident;

// use crate::{io::IO, utils::Tagged};
// use crate::r_ast::RStmt;

// type Variables = BTreeSet<Ident>;


// tag :: needed_below -> obj<()> -> (obj<IO>, needed_above)

// impl<T> Tagged<T, ()> {
//     fn tagged(&self, needed_below: Variables) -> (Tagged<T, IO>, Variables) {

//     }
// }

// // // tag :: needed_below -> obj<()> -> Tagged<obj>

// // impl Tagged<RStmt<IO>, IO> {
// //     fn tag(rstmt: RStmt<()>) -> Self {
// //         rstmt.
// //     }
// // }

// // use std::collections::{BTreeSet, HashSet};

// // use syn::Ident;

// // use crate::r_ast::*;
// // use crate::io::IO;

// // /// Pair of expression and inputs needed by expression
// // type Tagged<T> = (T, HashSet<Ident>);

// // /// defer_tagging!(value { Enum: Variant, Variant, ... })
// // /// will match value to the variants, and return the result of converting the value in the variant
// // macro_rules! defer_tagging {
// //     ($item:ident { $enum:ident :: $( $variant:ident ),* }) => {
// //         match $item {
// //             $($enum :: $variant (inner) => {
// //                 let (new_value, variables) = inner.into();
// //                 ($enum :: $variant (new_value), variables)
// //             }),*
// //         }
// //     };
// // }

// // /// Takes in an unlabeled AST nodes
// // /// Returns the same node with IO labels, and passes the required inputs up

// // impl From<RExpr<()>> for Tagged<RExpr<IO>> {
// //     fn from(value: RExpr<()>) -> Self {
// //         defer_tagging!(value { RExpr :: If, Block, Raw})
// //     }
// // }

// // impl From<RExprIf<()>> for Tagged<RExprIf<IO>> {
// //     fn from(RExprIf { condition, then_block, else_block }: RExprIf<()>) -> Self {
        
// //     }
// // }

// // impl From<RExprBlock<()>> for Tagged<RExprBlock<IO>> {
// //     fn from(value: RExprBlock<()>) -> Self {

// //     }
// // }

// // impl From<RStmt<()>> for Tagged<RStmt<IO>> {
// //     fn from(value: RStmt<()>) -> Self {
// //         defer_tagging!(value {RStmt :: LetAwait, Return, Expr, Raw})
// //     }
// // }

// // impl From<RStmtLetAwait<()>> for Tagged<RStmtLetAwait<IO>> {
// //     fn from(RStmtLetAwait { definition, box future }: RStmtLetAwait<()>) -> Self {
// //         /// PROBLEM: WE WANT TO REMOVE DEFINITION FROM THE NEEDED INPUTS OF OTHER STATEMENTS!!!
// //         /// THIS ABSTRACTION IS WRONG
// //         let (tagged_future, inputs) = future.into();
// //         let tagged_stmt = RStmtLetAwait {
// //             definition,
// //             future: Box::new(tagged_future),
// //         };
// //         inputs.remove()
// //         (tagged_stmt, inputs)
// //     }
// // }

// // impl From<RReturn<()>> for Tagged<RReturn<IO>> {
// //     fn from(value: RReturn<()>) -> Self {
// //         value.into()
// //     }
// // }

// // impl From<Raw<syn::Expr, ()>> for Tagged<Raw<syn::Expr, IO>> {
// //     fn from(value: Raw<syn::Expr, ()>) -> Self {
        
// //     }
// // }

// // impl From<Raw<syn::Stmt, ()>> for Tagged<Raw<syn::Stmt, IO>> {
// //     fn from(value: Raw<syn::Stmt, ()>) -> Self {
        
// //     }
// // }