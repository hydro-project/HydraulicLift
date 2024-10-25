// // use hydroflow_plus::ir::DebugExpr;
// // use quote::ToTokens;
// // use syn::{parse_quote, Expr, Ident};

// // use crate::io::Scope;

// // /// Hydroflow function, input and output of dataflow sub-tree.
// // trait HF<I, O> {}

// // /// Generate a closure
// // trait HfGen<I, O> {
// //     fn hf(input_pattern: I, body: Expr, output_pattern: O) -> DebugExpr;
// // }

// // impl<T, I, O> HfGen<I, O> for T
// // where
// //     T: HF<I, O>,
// //     I: ToTokens,
// //     O: ToTokens,
// // {
// //     fn hf(input_pattern: I, body: Expr, output_pattern: O) -> DebugExpr {
// //         let expr: Expr = parse_quote! {
// //             |#input_pattern| {
// //                 #body
// //                 #output_pattern
// //             }
// //         };
// //         expr.into()
// //     }
// // }

// // /// Input to the whole tree
// // /// :: (value, scope=[])
// // struct HInput;

// // /// Highest layer output, return value.
// // /// :: (value, scope) -> value
// // struct HReturn {
// //     input: HExpr
// // }

// // /// :: scope -> (value, scope)
// // struct HExpr {
// //     input: HScope,
// //     expr: Expr,
// //     scope: Scope
// // }

// // // :: (value, scope) -> scope
// // struct HBind {
// //     scope: Scope,
// //     definition: Ident,
// //     input: Box<HExpr>
// // }

// // // :: scope -> scope
// // struct HBlockEnd {
// //     scope: Scope,
// //     input: Box<HScope>
// // }

// // // :: _ -> scope
// // enum HScope {
// //     Bind(HBind),
// //     BlockEnd(HBlockEnd)
// // }

// // // :: _ -> value | (value, scope)
// // enum HExprReturn {
// //     Block(HBlock),
// //     Return(HReturn)
// // }

// // /// :: scope -> value | (value, scope)
// // enum HBlock {
// //     input: HScope
// // }

// // /*
// // {
// //     let x = input;
// //     x + 1
// // }

// //     HReturn(HExpr(Bind(x, HInput)))

// // {
// //     return input;
// //     x + 1
// // }

// //     HReturn(HInput)

// // {
// //     let x = input;
// //     if x > 1 {
// //         return x;
// //     }
// //     x + 1
// // }
// //     Bind(x, HInput)

// // */
// // NEW
// // scopes contain their inputs

// // NEW
// // nodes are just processing, they are externally linked

// use quote::ToTokens;
// use syn::{parse_quote, Expr, Ident};

// use crate::{io::Scope, utils::ident};

// // matches: scope | (a, b, c)
// pub enum ScopePat {
//     Ident(Ident),
//     Destructured(Scope)
// }
// // matches: expr | (value, scope) | (a, (b, c, d))
// pub enum ExprPat{
//     Ident(Ident),
//     Destructured(Ident, ScopePat)
// }

// pub trait HPattern {

// }

// impl HPattern for ScopePat {

// }

// impl HPattern for Ident {

// }

// impl HPattern for ExprPat {

// }

// pub trait HNode {
//     type I: HPattern;
//     type O: HPattern;
// }

// pub struct HExpr {
//     pub expr: Expr,
//     pub scope: Scope,
// }

// impl HNode for HExpr {
//     type I = ScopePat;
//     type O = ExprPat;
// }

// pub struct HBlock<O> {
//     pub stmt: Box<dyn HNode<I = ScopePat, O = ScopePat>>,
//     pub eval: Box<dyn HNode<I = ScopePat, O = O>>,
// }

// impl<O: HPattern> HNode for HBlock<O> {
//     type I = ScopePat;
//     type O = O;
// }

// pub struct HBind {
//     pub definition: Ident,
//     pub expr: Box<dyn HNode<I = ScopePat, O = ExprPat>>,
// }

// impl HNode for HBind {
//     type I = ScopePat;
//     type O = ScopePat;
// }

// pub struct HBranch<OT: HPattern, OF: HPattern> {
//     pub cond: Box<dyn HNode<I = ScopePat, O = ExprPat>>,
//     pub branch_t: Box<dyn HNode<I = ScopePat, O = OT>>,
//     pub branch_f: Box<dyn HNode<I = ScopePat, O = OF>>,
// }

// impl<O: HPattern> HNode for HBranch<O, O> {
//     type I = ScopePat;
//     type O = O;
// }

// fn test() {
//     let out = {
//         let x = 1;
//         {
//             let y = 2;
//             x + y
//         }
//     };
//     // input is a scope (will be wrapped around input value)

//     let whole = HBlock {
//         stmt: Box::new(HBind {
//             definition: ident("x"),
//             expr: Box::new(HExpr {
//                 expr: parse_quote!(1),
//                 scope: Scope::empty(),
//             }),
//         }),
//         eval: Box::new(HBlock {
//             stmt: Box::new(HBind {
//                 definition: ident("y"),
//                 expr: Box::new(HExpr {
//                     expr: parse_quote!(2),
//                     scope: Scope::empty().with(ident("x")),
//                 }),
//             }),
//             eval: Box::new(HExpr {
//                 expr: parse_quote!(x + y),
//                 scope: Scope::empty().with(ident("x")).with(ident("y")),
//             }),
//         }),
//     };

//     let output = {
//         let x = 1;
//         if x > 2 {
//             let y = 2;
//             x + y
//         } else {
//             let z = 3;
//             x + z
//         }
//     };

//     let whole = HBlock {
//         stmt: Box::new(HBind {
//             definition: ident("x"),
//             expr: Box::new(HExpr {
//                 expr: parse_quote!(1),
//                 scope: Scope::empty(),
//             }),
//         }),
//         eval: Box::new(HBranch {
//             cond: Box::new(HExpr {
//                 expr: parse_quote!(x > 2),
//                 scope: Scope::empty().with(ident("x")),
//             }),
//             branch_t: Box::new(HBlock {
//                 stmt: Box::new(HBind {
//                     definition: ident("y"),
//                     expr: Box::new(HExpr {
//                         expr: parse_quote!(2),
//                         scope: Scope::empty().with(ident("x")),
//                     }),
//                 }),
//                 eval: Box::new(HExpr {
//                     expr: parse_quote!(x+y),
//                     scope: Scope::empty().with(ident("x")).with(ident("y")),
//                 }),
//             }),
//             branch_f: Box::new(HBlock {
//                 stmt: Box::new(HBind {
//                     definition: ident("z"),
//                     expr: Box::new(HExpr {
//                         expr: parse_quote!(3),
//                         scope: Scope::empty().with(ident("x")),
//                     }),
//                 }),
//                 eval: Box::new(HExpr {
//                     expr: parse_quote!(x+z),
//                     scope: Scope::empty().with(ident("x")).with(ident("z")),
//                 }),
//             }),
//         }),
//     };
// }

// NEW
// input of a tree is always a scope!
// nodes contain their inputs
// nodes are simply pipelines
// branches contain branch & merge

use std::{ops::Add, rc::Rc};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Expr, Ident};

use crate::{
    io::{Scope, IO},
    utils::{ident, DebugStr, Tagged},
};

// :: value
#[derive(Debug, Clone)]
pub struct HOutput {
    pub input: HReturn,
    pub other: Option<Box<HOutput>>,
}

impl HNode for HOutput {
    type O = Ident;
}

impl HOutput {
    pub fn new(input: HReturn) -> Self {
        Self { input, other: None }
    }

    pub fn with(self, input: HReturn) -> Self {
        Self {
            input,
            other: Some(Box::new(self)),
        }
    }
}

pub trait HPattern: ToTokens {}

/// Specifies the pattern which can match the output of the node.
pub trait HNode {
    type O: HPattern;
}

/// matches: scope | (a, b, c)
pub enum ScopePat {
    Ident(Ident),
    Destructured(Scope),
}
/// matches: expr | (value, scope) | (a, (b, c, d))
pub enum ExprPat {
    Ident(Ident),
    Destructured(Ident, ScopePat),
}

impl ToTokens for ScopePat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ScopePat::Ident(ident) => ident.to_tokens(tokens),
            ScopePat::Destructured(scope) => scope.to_tokens(tokens),
        };
    }
}

impl ToTokens for ExprPat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExprPat::Ident(ident) => ident.to_tokens(tokens),
            ExprPat::Destructured(ident, scope_pat) => tokens.extend(quote! {(#ident, #scope_pat)}),
        }
    }
}

impl HPattern for ScopePat {}

impl HPattern for Ident {}

impl HPattern for ExprPat {}

impl<T, O, M> HNode for Tagged<T, M>
where
    O: HPattern,
    T: HNode<O = O>,
{
    type O = O;
}

/// :: value
#[derive(Debug, Clone)]
pub struct HReturn {
    pub value: HExpr,
}

impl HNode for HReturn {
    type O = Ident;
}

/// :: (value, scope)
#[derive(Debug, Clone)]
pub enum HExpr {
    Raw(Tagged<HExprRaw, IO>),
    // A merge point
    Union(HExprUnion),
    /// A branch point
    Shared(HExprShared),
}

impl HNode for HExpr {
    type O = ExprPat;
}

#[derive(Debug, Clone)]
pub struct HExprUnion(pub Box<HExpr>, pub Box<HExpr>);

impl HNode for HExprUnion {
    type O = ExprPat;
}

#[derive(Debug, Clone)]
pub struct HExprShared(pub Rc<HExpr>);

impl HNode for HExprShared {
    type O = ExprPat;
}

#[derive(Debug, Clone)]
pub struct HExprRaw {
    pub expr: DebugStr<Expr>,
    pub input: HScope,
}

impl HNode for HExprRaw {
    type O = ExprPat;
}

/// :: scope
#[derive(Debug, Clone)]
pub enum HScope {
    Input(HInput),
    Bind(Tagged<HBind, IO>),
    Filter(HFilter),
}

impl HNode for HScope {
    type O = ScopePat;
}

#[derive(Debug, Clone)]
pub struct HBind {
    pub id: Ident,
    pub value: Box<HExpr>,
}

impl HNode for HBind {
    type O = ScopePat;
}

/// Filters for cond == expectation
#[derive(Debug, Clone)]
pub struct HFilter {
    pub expectation: bool,
    pub cond: Box<HExpr>,
}

impl HNode for HFilter {
    type O = ScopePat;
}

#[derive(Debug, Clone)]
pub struct HInput;

impl HNode for HInput {
    type O = ScopePat;
}

// fn test() {
//     let input = 1;
//     let out = {
//         let x = input;
//         {
//             let y = 2;
//             x + y
//         }
//     };

//     let whole = HExpr::Raw(HExprRaw {
//         input: HScope::Bind(HBind {
//             input: Box::new(HExpr::Raw(HExprRaw {
//                 input: HScope::Bind(HBind {
//                     input: Box::new(HExpr::Raw(HExprRaw {
//                         input: HScope::Input(HInput),
//                         expr: parse_quote!(input),
//                     })),
//                     id: ident("x"),
//                 }),
//                 expr: parse_quote!(2),
//             })),
//             id: ident("y"),
//         }),
//         expr: parse_quote!(x + y),
//     });

//     let out = {
//         let x = 1;
//         if x < 2 {
//             let y = 3;
//             x + y
//         } else {
//             let z = 4;
//             x + z
//         }
//     };

// }
