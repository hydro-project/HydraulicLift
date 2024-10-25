// use std::rc::Rc;

// use syn::Expr;

// use crate::{
//     io::{Scope, IO},
//     r_ast::*,
//     utils::{ident, DebugStr, Tagged},
// };

// // tag<T> :: T<()> -> needed_output -> (T<IO>, needed_input)

// // TODO: tag<T> :: T<()> -> output_scope -> (T<IO>, input_scope)
// impl RExpr<()> {
//     pub fn tag(self) -> RExpr<IO>  {
//         match self {
//             RExpr::If(s) => RExpr::If(s.tag()),
//             RExpr::Block(s) => RExpr::Block(s.tag()),
//             RExpr::Raw(s) => RExpr::Raw(s.tag()),
//         }
//     }
// }

// impl RExprBlock<()> {
//     pub fn tag(self) -> RExprBlock<IO> {
//         let Self { stmt, box expr } = self;
//         RExprBlock {
//             stmt: stmt.tag(),
//             expr: Box::new(expr.tag()),
//         }
//     }
// }

// impl RExprIf<()> {
//     pub fn tag(self) -> RExprIf<IO> {
//         let Self {
//             box cond_expr,
//             box then_expr,
//             box else_expr,
//         } = self;
//         RExprIf {
//             cond_expr: Box::new(cond_expr.tag()),
//             then_expr: Box::new(then_expr.tag()),
//             else_expr: Box::new(else_expr.tag()),
//         }
//     }
// }

// impl Tagged<RExprRaw, ()> {
//     fn tag(self) -> Tagged<RExprRaw, IO> {
//         let Self(inner, ()) = self;
//         Tagged(
//             inner,
//             IO {
//                 ins: Scope::empty(),
//                 outs: Scope::empty(),
//             },
//         )
//         //TODO: actually implement this
//     }
// }

// impl RStmt<()> {
//     pub fn tag(self) -> RStmt<IO> {
//         match self {
//             RStmt::Let(s) => RStmt::Let(s.tag()),
//             RStmt::Return(s) => RStmt::Return(s.tag()),
//         }
//     }
// }

// impl Tagged<RStmtLet<()>, ()> {
//     pub fn tag(self) -> Tagged<RStmtLet<IO>, IO> {
//         let Tagged(
//             RStmtLet {
//                 id: ident,
//                 box value,
//             },
//             (),
//         ) = self;
//         let ident_clone = ident.clone();
//         Tagged(
//             RStmtLet {
//                 id: ident,
//                 value: Box::new(value.tag()),
//             },
//             IO {
//                 ins: Scope::empty(),
//                 outs: Scope::empty().with(ident_clone),
//             },
//         )
//     }
// }

// impl RStmtReturn<()> {
//     pub fn tag(self) -> RStmtReturn<IO> {
//         let RStmtReturn { box value } = self;
//         RStmtReturn {
//             value: Box::new(value.tag()),
//         }
//     }
// }

// // use std::collections::BTreeSet;

// // use syn::Ident;

// // use crate::{io::IO, utils::Tagged};
// // use crate::r_ast::RStmt;

// // type Variables = BTreeSet<Ident>;

// // tag :: needed_below -> obj<()> -> (obj<IO>, needed_above)

// // impl<T> Tagged<T, ()> {
// //     fn tagged(&self, needed_below: Variables) -> (Tagged<T, IO>, Variables) {

// //     }
// // }

// // // // tag :: needed_below -> obj<()> -> Tagged<obj>

// // // impl Tagged<RStmt<IO>, IO> {
// // //     fn tag(rstmt: RStmt<()>) -> Self {
// // //         rstmt.
// // //     }
// // // }

// // // use std::collections::{BTreeSet, HashSet};

// // // use syn::Ident;

// // // use crate::r_ast::*;
// // // use crate::io::IO;

// // // /// Pair of expression and inputs needed by expression
// // // type Tagged<T> = (T, HashSet<Ident>);

// // // /// defer_tagging!(value { Enum: Variant, Variant, ... })
// // // /// will match value to the variants, and return the result of converting the value in the variant
// // // macro_rules! defer_tagging {
// // //     ($item:ident { $enum:ident :: $( $variant:ident ),* }) => {
// // //         match $item {
// // //             $($enum :: $variant (inner) => {
// // //                 let (new_value, variables) = inner.into();
// // //                 ($enum :: $variant (new_value), variables)
// // //             }),*
// // //         }
// // //     };
// // // }

// // // /// Takes in an unlabeled AST nodes
// // // /// Returns the same node with IO labels, and passes the required inputs up

// // // impl From<RExpr<()>> for Tagged<RExpr<IO>> {
// // //     fn from(value: RExpr<()>) -> Self {
// // //         defer_tagging!(value { RExpr :: If, Block, Raw})
// // //     }
// // // }

// // // impl From<RExprIf<()>> for Tagged<RExprIf<IO>> {
// // //     fn from(RExprIf { condition, then_block, else_block }: RExprIf<()>) -> Self {

// // //     }
// // // }

// // // impl From<RExprBlock<()>> for Tagged<RExprBlock<IO>> {
// // //     fn from(value: RExprBlock<()>) -> Self {

// // //     }
// // // }

// // // impl From<RStmt<()>> for Tagged<RStmt<IO>> {
// // //     fn from(value: RStmt<()>) -> Self {
// // //         defer_tagging!(value {RStmt :: LetAwait, Return, Expr, Raw})
// // //     }
// // // }

// // // impl From<RStmtLetAwait<()>> for Tagged<RStmtLetAwait<IO>> {
// // //     fn from(RStmtLetAwait { definition, box future }: RStmtLetAwait<()>) -> Self {
// // //         /// PROBLEM: WE WANT TO REMOVE DEFINITION FROM THE NEEDED INPUTS OF OTHER STATEMENTS!!!
// // //         /// THIS ABSTRACTION IS WRONG
// // //         let (tagged_future, inputs) = future.into();
// // //         let tagged_stmt = RStmtLetAwait {
// // //             definition,
// // //             future: Box::new(tagged_future),
// // //         };
// // //         inputs.remove()
// // //         (tagged_stmt, inputs)
// // //     }
// // // }

// // // impl From<RReturn<()>> for Tagged<RReturn<IO>> {
// // //     fn from(value: RReturn<()>) -> Self {
// // //         value.into()
// // //     }
// // // }

// // // impl From<Raw<syn::Expr, ()>> for Tagged<Raw<syn::Expr, IO>> {
// // //     fn from(value: Raw<syn::Expr, ()>) -> Self {

// // //     }
// // // }

// // // impl From<Raw<syn::Stmt, ()>> for Tagged<Raw<syn::Stmt, IO>> {
// // //     fn from(value: Raw<syn::Stmt, ()>) -> Self {

// // //     }
// // // }

// tag<T> :: T<()> -> needed_output -> (T<IO>, needed_input)

use syn::visit::Visit;

use crate::{
    io::{Scope, IO}, r_ast::*, transform::Unionable, utils::{Tagged, TupleFunctor}
};

impl From<RExpr> for RExpr<IO> {
    fn from(untagged: RExpr) -> Self {
        Tag::tag(untagged, Scope::empty()).1
    }
}

trait Tag {
    type Out;

    fn tag(untagged: Self, output: Scope) -> (Scope, Self::Out);
}

impl Tag for RExpr {
    type Out = RExpr<IO>;

    fn tag(untagged: Self, output: Scope) -> (Scope, RExpr<IO>) {
        match untagged {
            RExpr::If(s) => Tag::tag(s, output).map(RExpr::If),
            RExpr::Block(s) => Tag::tag(s, output).map(RExpr::Block),
            RExpr::Raw(s) => Tag::tag(s, output).map(RExpr::Raw),
        }
    }
}

impl Tag for RExprIf {
    type Out = RExprIf<IO>;

    fn tag(Self { box cond_expr, box then_expr, box else_expr }: Self, output: Scope) -> (Scope, RExprIf<IO>) {
        let (else_input, else_expr) = Tag::tag(else_expr, output.clone());
        let (then_input, then_expr) = Tag::tag(then_expr, output.clone());
        // This probably doesnt work loll then_input and else_input are expecting different things.
        // Maybe they need to be Tagged?
        Tag::tag(cond_expr, then_input.union(else_input)).map(|cond_expr| RExprIf::new(cond_expr, then_expr, else_expr))
    }
}

impl Tag for RExprBlock {
    type Out = RExprBlock<IO>;

    fn tag(Self { stmt, box expr }: Self, output: Scope) -> (Scope, RExprBlock<IO>) {
        // TODO: this doesnt include nested scopes!!!
        // let x = 5;
        // {
        //    let x = 3;
        // }
        // x
        // Solution: Maybe disallow re-using identifiers anywhere?
        let (expr_input, expr) = Tag::tag(expr, output);
        let (stmt_input, stmt) = Tag::tag(stmt, expr_input);
        (stmt_input, RExprBlock::new(stmt, expr))
    }
}

impl Tag for RExprRaw {
    type Out = RExprRaw;

    fn tag(expr: Self, output: Scope) -> (Scope, RExprRaw) {
        let mut visitor = ScopeVisitor {
            current_scope: output,
        };
        visitor.visit_expr(&expr.0);
        let ScopeVisitor {
            current_scope: input,
        } = visitor;
        (input, expr)
    }
}

impl Tag for RStmt {
    type Out = RStmt<IO>;

    fn tag(untagged: Self, output: Scope) -> (Scope, RStmt<IO>) {
        match untagged {
            RStmt::Let(s) => Tag::tag(s, output).map(RStmt::Let),
            RStmt::Return(s) => Tag::tag(s, output).map(RStmt::Return),
        }
    }
}

impl Tag for RStmtLet {
    type Out = RStmtLet<IO>;

    fn tag(Self { id, box value }: Self, output: Scope) -> (Scope, RStmtLet<IO>) {
        Tag::tag(value, output.without(&id)).map(|value| RStmtLet::new(id, value))
    }
}

impl Tag for RStmtReturn {
    type Out = RStmtReturn<IO>;

    fn tag(Self { box value }: Self, _: Scope) -> (Scope, RStmtReturn<IO>) {
        Tag::tag(value, Scope::empty()).map(|value| RStmtReturn::new(value))
    }
}

impl<U1, U2> Tag for Tagged<U1>
where
    U1: Tag<Out = U2>,
{
    type Out = Tagged<U2, IO>;

    fn tag(Self(inner, ()): Self, output: Scope) -> (Scope, Tagged<U2, IO>) {
        let (input, inner) = Tag::tag(inner, output.clone());
        (
            input.clone(),
            Tagged(
                inner,
                IO {
                    ins: input,
                    outs: output,
                },
            ),
        )
    }
}

struct ScopeVisitor {
    current_scope: Scope,
}

impl<'ast> Visit<'ast> for ScopeVisitor {
    fn visit_ident(&mut self, ident: &'ast proc_macro2::Ident) {
        self.current_scope = self.current_scope.with(ident.clone())
    }

    fn visit_pat_ident(&mut self, syn::PatIdent { ident, .. }: &'ast syn::PatIdent) {
        // TODO: support mutability
        self.current_scope = self.current_scope.with(ident.clone())
    }
}

// impl RExpr {
//     pub fn tag(self, output: Scope) -> (RExpr<IO>, Scope) {
//         match self {
//             Self::If(s) => s.tag(output).map(RExpr::If),
//             Self::Block(s) => s.tag(output).map(RExpr::Block),
//             Self::Raw(s) => s.tag(output).map(RExpr::Raw),
//         }
//     }
// }

// impl RExprIf {
//     pub fn tag(self, output: Scope) -> (RExprIf<IO>, Scope) {

//     }
// }

// impl RExprBlock {
//     pub fn tag(self, output: Scope) -> (RExprBlock<IO>, Scope) {

//     }
// }

// impl RExprRaw {
//     pub fn tag(self, output: Scope) -> (RExprRaw<IO>, Scope) {

//     }
// }

// impl RStmt {
//     pub fn tag(self, output: Scope) -> (RStmt<IO>, Scope) {

//     }
// }

// impl RStmtLet {
//     pub fn tag(self, output: Scope) -> (RStmtLet<IO>, Scope) {

//     }
// }

// impl RStmtReturn {
//     pub fn tag(self, output: Scope) -> (RStmtReturn<IO>, Scope) {

//     }
// }

// impl<T> Tagged<T> {
//     pub fn tag(self, output: Scope) -> (Tagged<T, IO>, Scope) {
//         let Self(inner, ()) = self;
//         inner.
//     }
// }
