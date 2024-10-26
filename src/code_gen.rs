// // use std::{cell::RefCell, rc::Rc};

// // use hydroflow_plus::ir::HfPlusNode;
// // use quote::{quote, ToTokens};
// // use syn::{parse_quote, Expr, Ident, Stmt};

// // use crate::{
// //     io::{Scope, IO}, ir::HEntryPoint, r_ast::*, utils::{ident, DebugStr, Tagged}
// // };

// // // TODO: new ast? expr trait (GAT expr type), scope trait

// // pub trait IntoHf<'a> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a>;
// // }

// // /// Returns (return_value, scope)
// // impl<'a> IntoHf<'a> for RExpr<IO> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
// //         match self {
// //             RExpr::If(s) => s.into_hf(input),
// //             RExpr::Block(s) => s.into_hf(input),
// //             RExpr::Raw(s) => s.into_hf(input),
// //         }
// //     }
// // }

// // impl<'a> IntoHf<'a> for RExprIf<IO> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
// //         let Self {
// //             box cond_expr,
// //             box then_expr,
// //             box else_expr,
// //         } = self;
// //         let cond_node = Rc::new(RefCell::new(cond_expr.into_hf(input)));
// //         // TODO: Replace with partition
// //         // filtered for cond
// //         let then_cond_node = HfPlusNode::Filter {
// //             input: Box::new(HfPlusNode::Tee {
// //                 inner: cond_node.clone(),
// //             }),
// //             f: cond_to_option_f().into(),
// //         };
// //         // filtered for !cond
// //         let else_cond_node = HfPlusNode::Filter {
// //             input: Box::new(HfPlusNode::Map {
// //                 input: Box::new(HfPlusNode::Tee {
// //                     inner: cond_node.clone(),
// //                 }),
// //                 f: map_expr_f(quote! {::std::ops::Not::not}).into(),
// //             }),
// //             f: cond_to_option_f().into(),
// //         };

// //         let then_node = then_expr.into_hf(Box::new(then_cond_node));
// //         let else_node = else_expr.into_hf(Box::new(else_cond_node));

// //         HfPlusNode::Union(Box::new(then_node), Box::new(else_node))
// //     }
// // }

// // impl<'a> IntoHf<'a> for RExprBlock<IO> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
// //         let Self {
// //             stmt,
// //             box return_expr,
// //         } = self;
// //         let stmt_node = stmt.into_hf(input).into();
// //         return_expr.into_hf(stmt_node)
// //     }
// // }

// // impl<'a> IntoHf<'a> for Tagged<DebugStr<Expr>, IO> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
// //         let Tagged(
// //             DebugStr(expr),
// //             IO {
// //                 input_scope: inputs,
// //                 output_scope: outputs,
// //             },
// //         ) = self; // TODO: do we need outputs here?

// //         let f = evaluate_expr_f(expr, &inputs).into();
// //         HfPlusNode::Map { input, f }
// //     }
// // }

// // impl<'a> IntoHf<'a> for RStmt<IO> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
// //         match self {
// //             RStmt::Let(s) => s.into_hf(input),
// //             RStmt::Return(s) => s.into_hf(input),
// //         }
// //     }
// // }

// // impl<'a> IntoHf<'a> for Tagged<RStmtLet<IO>, IO> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
// //         let Tagged(
// //             RStmtLet { ident, box value },
// //             IO {
// //                 input_scope,
// //                 output_scope,
// //             },
// //         ) = self;

// //         HfPlusNode::Map {
// //             input: value.into_hf(input).into(),
// //             f: bind_expr_f(ident, &input_scope, &output_scope).into(),
// //         }
// //     }
// // }

// // impl<'a> IntoHf<'a> for RStmtReturn<IO> {
// //     fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
// //         // TODO: fix this please
// //         *input
// //     }
// // }

// // // In this section, we construct closures for nodes
// // // Note that scopes are tuples of variable names,
// // // and expressions output (return_value, scope)

// // /// Evalaluates an expression into a value, using some scope
// // /// :: scope -> (return_value, scope)
// // fn evaluate_expr_f(expr: Expr, scope: &[Ident]) -> Expr {
// //     parse_quote! {
// //         |#(#scope),*| (#expr, (#(#scope,)*))
// //     }
// // }

// // /// Evaluates a boolean for a filter_map
// // /// :: (condition, scope) -> Option<scope>
// // fn cond_to_option_f() -> Expr {
// //     parse_quote! {
// //         |(condition, scope)| condition.then_some(scope)
// //     }
// // }

// // /// Evaluates a map function over an expression input
// // /// :: (input_value, scope) -> (return_value, scope)
// // fn map_expr_f(map_func: impl ToTokens) -> Expr {
// //     parse_quote! {
// //         |(input_value, scope)| ((#map_func)(input_value), scope)
// //     }
// // }

// // /// Binds an expr return value to a variable named ident in scope
// // /// :: (input_value, input_scope) -> output_scope
// // fn bind_expr_f(ident: Ident, input_scope: &[Ident], output_scope: &[Ident]) -> Expr {
// //     parse_quote! {
// //         |(#ident, (#(#input_scope,)*))| (#(#output_scope,)*)
// //     }
// // }

// //
// // let x = i + 1;
// // if (x < 5) {
// //     let z = 1+1;
// //     let y = a.await;
// //     y + z
// // } else {
// //     let z = a.await;
// //     return z;
// // }
// // x + 2
// //
// // ->
// //
// // input
// // -> map(|i| { // raw stmt
// //     let x = i + 1;
// //     (x)
// // })
// // -> map(|(x))| { // raw expr
// //     let _temp1 = x < 5;
// //     (_temp1, in)
// // })
// // -> demux(|(_temp1, in)| {
// //     _temp1 => then
// //     !_temp1 => else
// // })
// //
// // then = map(|(x))

// // NEW

// use std::{cell::RefCell, rc::Rc};

// use hydroflow_plus::ir::HfPlusNode;
// use quote::{quote, ToTokens};

// use crate::{
//     io::Scope,
//     ir::{HBlockEnd, HEntryPoint, HExprBind, HExprBranch, HExprConsumer, HExprMap, HExprRaw, HLink, HReturn},
//     utils::ident,
// };

// /// Links a hydroflow input into an H node.
// pub trait HfGen<'a> {
//     fn gen(input: Box<HfPlusNode<'a>>, h_node: Self) -> HfPlusNode<'a>;
// }

// // TODO: value is a special keyword atm

// /// Constructs a quoted closure which maps from input to output (with optional body)
// macro_rules! hf (
//     {$i:expr => $o:expr, $($body:tt)*} => {
//         {
//             let inputs = $i;
//             let outputs = $o;
//             let expr: ::syn::Expr = ::syn::parse_quote! {
//                 |#inputs| {
//                     $($body)*
//                     #outputs
//                 }
//             };
//             let debug_expr: ::hydroflow_plus::ir::DebugExpr = expr.into();
//             debug_expr
//         }
//     };
//     {$i:expr => $o:expr} => {
//         hf! {$i => $o,}
//     }
// );

// /// Tuple which can be tokenized
// struct TokTup<T, U>(pub T, pub U);

// impl<T: ToTokens, U: ToTokens> ToTokens for TokTup<T, U> {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         let Self(a, b) = self;
//         tokens.extend(quote! {
//             (#a, #b)
//         });
//     }
// }

// impl<'a> HfGen<'a> for HEntryPoint {
//     fn gen(input: Box<HfPlusNode<'a>>, Self { next }: Self) -> HfPlusNode<'a> {
//         let scope = TokTup(ident("value"), Scope::empty());
//         let f = hf!(&scope.0 => &scope);
//         let node = HfPlusNode::Map { input, f };
//         HfGen::gen(Box::new(node), next)
//     }
// }

// impl<'a> HfGen<'a> for HExprConsumer {
//     fn gen(input: Box<HfPlusNode<'a>>, h_node: Self) -> HfPlusNode<'a> {
//         match h_node {
//             HExprConsumer::Map(s) => HfGen::gen(input, s),
//             HExprConsumer::Bind(s) => HfGen::gen(input, s),
//             HExprConsumer::Branch(s) => HfGen::gen(input, s),
//             HExprConsumer::Return(s) => HfGen::gen(input, s),
//         }
//     }
// }

// impl<'a> HfGen<'a> for HExprMap {
//     fn gen(
//         input: Box<HfPlusNode<'a>>,
//         Self {
//             map_expr,
//             scope,
//             box next,
//         }: Self,
//     ) -> HfPlusNode<'a> {
//         let f = hf!(&scope => &scope, #map_expr);
//         let node = HfPlusNode::Map { input, f };
//         HfGen::gen(Box::new(node), next)
//     }
// }

// impl<'a> HfGen<'a> for HExprBind {
//     fn gen(
//         input: Box<HfPlusNode<'a>>,
//         Self {
//             definition,
//             scope,
//             next,
//         }: Self,
//     ) -> HfPlusNode<'a> {
//         let output_scope = scope.with(definition.clone());
//         let f = hf!(TokTup(ident("value"), scope) => output_scope, let #definition = value;);
//         let node = HfPlusNode::Map { input, f };
//         HfGen::gen(Box::new(node), next)
//     }
// }

// impl<'a> HfGen<'a> for HExprBranch {
//     fn gen(
//         box input: Box<HfPlusNode<'a>>,
//         Self {
//             next_true,
//             next_false,
//         }: Self,
//     ) -> HfPlusNode<'a> {
//         // TODO: this cannot handle returns?
//         let scope = TokTup(ident("predicate"), ident("scope"));

//         let f_true = hf!(&scope => &scope.1, predicate.then_some(scope));
//         let f_false = hf!(&scope => &scope.1, predicate.not().then_some(scope));

//         let input_true = Rc::new(RefCell::new(input));
//         let input_false = input_true.clone();

//         HfPlusNode::Union(
//             Box::new(HfGen::gen(
//                 Box::new(HfPlusNode::FilterMap {
//                     f: f_true,
//                     input: Box::new(HfPlusNode::Tee { inner: input_true }),
//                 }),
//                 next_true,
//             )),
//             Box::new(HfGen::gen(
//                 Box::new(HfPlusNode::FilterMap {
//                     f: f_false,
//                     input: Box::new(HfPlusNode::Tee { inner: input_false }),
//                 }),
//                 next_false,
//             )),
//         )
//     }
// }

// impl<'a> HfGen<'a> for HReturn {
//     fn gen(input: Box<HfPlusNode<'a>>, HReturn: Self) -> HfPlusNode<'a> {
//         let scope = TokTup(ident("value"), ident("_"));
//         let f = hf!(&scope => &scope.0);
//         HfPlusNode::Map { input, f }
//     }
// }

// impl<'a> HfGen<'a> for HLink {
//     fn gen(input: Box<HfPlusNode<'a>>, h_node: Self) -> HfPlusNode<'a> {
//         match h_node {
//             HLink::BlockEnd(s) => HfGen::gen(input, s),
//             HLink::Expr(box s) => HfGen::gen(input, s),
//         }
//     }
// }

// impl<'a> HfGen<'a> for HBlockEnd {
//     fn gen(input: Box<HfPlusNode<'a>>, Self { old_scope, new_scope, box next }: Self) -> HfPlusNode<'a> {
//         let f = hf!(old_scope => new_scope);
//         let node = HfPlusNode::Map { input, f };
//         HfGen::gen(Box::new(node), next)
//     }
// }

// impl<'a> HfGen<'a> for HExprRaw {
//     fn gen(input: Box<HfPlusNode<'a>>, Self { expr, scope, next }: Self) -> HfPlusNode<'a> {
//         let expr_scope = TokTup(scope, ident("value"));
//         let f = hf!(&expr_scope.0 => &expr_scope, let value = #expr;);
//         let node = HfPlusNode::Map { input, f };
//         HfGen::gen(Box::new(node), next)
//     }
// }

use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use by_address::ByAddress;
use hydroflow_plus::ir::{DebugExpr, HfPlusNode};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Expr};

use crate::{
    io::Scope, ir2::{
        ExprPat, HBind, HExpr, HExprRaw, HExprShared, HExprUnion, HFilter, HInput, HNode, HOutput,
        HPattern, HReturn, HScope, ScopePat,
    }, utils::{ident, Tagged, TupleFunctor}
};

// /// Constructs a quoted closure which maps from input to output (with optional body)
// macro_rules! hfunc (
//     {$i:expr => $o:expr, $($body:tt)*} => {
//         {
//             let inputs = $i;
//             let outputs = $o;
//             let expr: ::syn::Expr = ::syn::parse_quote! {
//                 |#inputs| {
//                     $($body)*
//                     #outputs
//                 }
//             };
//             let debug_expr: ::hydroflow_plus::ir::DebugExpr = expr.into();
//             debug_expr
//         }
//     };
//     {$i:expr => $o:expr} => {
//         hf! {$i => $o,}
//     }
// );

/// Generates hydroflow+ node from hnode, which consumes the specified input.
pub fn generate<'a>(h_node: HOutput, input: HfPlusNode<'a>) -> HfPlusNode<'a> {
    let mut memos = HfMemos::new();
    memos.put(Rc::new(HInput), input);
    *HOutput::gen(h_node, memos).1
}

trait HfGen<'a>: HNode {
    /// Generates Hydroflow+ from h_node. memos must contain inputs.
    fn gen(h_node: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>);
}

// trait HfGenFuncBody<'a, O: HPattern>: HfGen<'a> + HNode<O = O> {
//     fn gen_func_body<U, I: HPattern>(
//         h_node: U,
//         hf_input: Rc<HfPlusNode<'a>>,
//         input_scope: I,
//         output_scope: O,
//         body: TokenStream,
//     ) -> (Box<HfPlusNode<'a>>, DebugExpr)
//     where
//         U: HfGen<'a> + HNode<O = I>;
// }

// trait HfGenFunc<'a, O: HPattern>: HfGenFuncBody<'a, O> + HfGen<'a> + HNode<O = O> {
//     fn gen_func<U, I: HPattern>(
//         h_node: U,
//         hf_input: Rc<HfPlusNode<'a>>,
//         input_scope: I,
//         output_scope: O,
//     ) -> (Box<HfPlusNode<'a>>, DebugExpr)
//     where
//         U: HfGen<'a> + HNode<O = I>;
// }

// impl<'a, T, O: HPattern> HfGenFuncBody<'a, O> for T
// where
//     T: HfGen<'a> + HNode<O = O>,
// {
//     fn gen_func_body<U, I: HPattern>(
//         h_node: U,
//         hf_input: Rc<HfPlusNode<'a>>,
//         input_pattern: I,
//         output_pattern: O,
//         body: TokenStream,
//     ) -> (Box<HfPlusNode<'a>>, DebugExpr)
//     where
//         U: HfGen<'a> + HNode<O = I>,
//     {
//         let hf_node = U::gen(h_node, hf_input);
//         let f: Expr = parse_quote! {
//             |#input_pattern|
//             #body
//             #output_pattern
//         };
//         (hf_node, f.into())
//     }
// }

// impl<'a, T, O: HPattern> HfGenFunc<'a, O> for T
// where
//     T: HfGenFuncBody<'a, O> + HfGen<'a> + HNode<O = O>,
// {
//     fn gen_func<U, I: HPattern>(
//         h_node: U,
//         hf_input: Rc<HfPlusNode<'a>>,
//         input_scope: I,
//         output_scope: O,
//     ) -> (Box<HfPlusNode<'a>>, DebugExpr)
//     where
//         U: HfGen<'a> + HNode<O = I>,
//     {
//         Self::gen_func_body(
//             h_node,
//             hf_input,
//             input_scope,
//             output_scope,
//             TokenStream::new(),
//         )
//     }
// }

/// Memoized conversion from K (by reference) to a HF+ node.
pub struct HfMemo<'a, K: Clone> {
    map: HashMap<ByAddress<Rc<K>>, Rc<RefCell<HfPlusNode<'a>>>>,
}

pub trait HfMemoize<'a, K: Clone> {
    /// Get a clone of the corresponding value pointer if present.
    fn get(&self, key: &Rc<K>) -> Option<Rc<RefCell<HfPlusNode<'a>>>>;

    /// Memoize a mapping.
    fn put(&mut self, key: Rc<K>, value: HfPlusNode<'a>);
}

impl<'a, K: Clone> HfMemo<'a, K> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<'a, K: Clone> HfMemoize<'a, K> for HfMemo<'a, K> {
    fn get(&self, key: &Rc<K>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.map.get(&ByAddress(key.clone())).map(Clone::clone)
    }

    fn put(&mut self, key: Rc<K>, value: HfPlusNode<'a>) {
        self.map
            .insert(ByAddress(key), Rc::new(RefCell::new(value)));
    }
}

/// Memoized conversions from Rc<K: HNode> to Rc<HfPlusNode>.
/// This allowes for constructing tees off of shared inputs
struct HfMemos<'a> {
    exprs: HfMemo<'a, HExpr>,
    inputs: HfMemo<'a, HInput>,
}

impl<'a> HfMemos<'a> {
    fn new() -> Self {
        Self {
            exprs: HfMemo::new(),
            inputs: HfMemo::new(),
        }
    }
}

impl<'a> HfMemoize<'a, HExpr> for HfMemos<'a> {
    fn get(&self, key: &Rc<HExpr>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.exprs.get(key)
    }

    fn put(&mut self, key: Rc<HExpr>, value: HfPlusNode<'a>) {
        self.exprs.put(key, value);
    }
}

impl<'a> HfMemoize<'a, HInput> for HfMemos<'a> {
    fn get(&self, key: &Rc<HInput>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.inputs.get(key)
    }

    fn put(&mut self, key: Rc<HInput>, value: HfPlusNode<'a>) {
        self.inputs.put(key, value);
    }
}

impl<'a> HfMemos<'a> {
    /// Either gets the memoized value, or generates and memoize it
    fn get_or_gen<K>(self, key: Rc<K>) -> (Self, Rc<RefCell<HfPlusNode<'a>>>)
    where
        K: Clone + HfGen<'a>,
        Self: HfMemoize<'a, K>,
    {
        if let Some(value) = self.get(&key) {
            (self, value)
        } else {
            let ki: K = (*key).clone();
            K::gen(ki, self).map(|box node| Rc::new(RefCell::new(node)))
        }
    }
}

struct HFunc<I: HPattern, O: HPattern> {
    pub ins: I,
    pub outs: O,
    pub body: TokenStream,
}

impl<I: HPattern, O: HPattern> HFunc<I, O> {
    pub fn newb(ins: I, outs: O, body: TokenStream) -> Self {
        Self { ins, outs, body }
    }

    pub fn new(ins: I, outs: O) -> Self {
        Self {
            ins,
            outs,
            body: TokenStream::new(),
        }
    }
}

trait HfGenMap<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: HPattern,
{
    /// Generate a node which maps over h_node.
    fn gen_map<U, I: HPattern>(
        h_node: U,
        memos: HfMemos<'a>,
        func: HFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>;
}

impl<'a, T, O> HfGenMap<'a, O> for T
where
    O: HPattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_map<U, I: HPattern>(
        h_node: U,
        memos: HfMemos<'a>,
        HFunc { ins, outs, body }: HFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>,
    {
        let f_expr: Expr = parse_quote! {
            |#ins|
            {
                #body
                #outs
            }
        };
        U::gen(h_node, memos).map(|hf_node| Box::new(HfPlusNode::Map {
            f: f_expr.into(),
            input: hf_node,
        }))
    }
}

trait HfGenFilterMap<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: HPattern,
{
    /// Generate a node which filter maps over h_node.
    /// The body of func should handle returning None.
    fn gen_filter_map<U, I: HPattern>(
        h_node: U,
        memos: HfMemos<'a>,
        func: HFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>;
}

impl<'a, T, O> HfGenFilterMap<'a, O> for T
where
    O: HPattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_filter_map<U, I: HPattern>(
        h_node: U,
        memos: HfMemos<'a>,
        HFunc { ins, outs, body }: HFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>,
    {
        // TODO: merge function definition into HFunc?
        let f_expr: Expr = parse_quote! {
            |#ins|
            {
                #body
                Some(#outs)
            }
        };
        U::gen(h_node, memos).map(|hf_node| Box::new(HfPlusNode::Map {
            f: f_expr.into(),
            input: hf_node,
        }))
    }
}

trait HfGenTee<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: HPattern,
{
    /// Generate a node which tees over a shared node.
    /// Does this by either
    /// 1) getting the generated node from memos,
    /// or 2) generating it and memoizing it into memos.
    fn gen_tee<U, I>(h_node: Rc<U>, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        I: HPattern,
        U: HfGen<'a> + HNode<O = I> + Clone,
        HfMemos<'a>: HfMemoize<'a, U>;
}

impl<'a, T, O> HfGenTee<'a, O> for T
where
    O: HPattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_tee<U, I>(h_node: Rc<U>, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        I: HPattern,
        U: HfGen<'a> + HNode<O = I> + Clone,
        HfMemos<'a>: HfMemoize<'a, U>,
    {
        memos.get_or_gen(h_node).map(|inner| Box::new(HfPlusNode::Tee { inner }))
    }
}

trait HfGenUnion<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: HPattern,
{
    /// Generate a node merges two input streams.
    fn gen_union<U1, U2, I: HPattern>(
        h_node1: U1,
        h_node2: U2,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U1: HfGen<'a> + HNode<O = I>,
        U2: HfGen<'a> + HNode<O = I>;
}

impl<'a, T, O> HfGenUnion<'a, O> for T
where
    O: HPattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_union<U1, U2, I: HPattern>(
        h_node1: U1,
        h_node2: U2,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U1: HfGen<'a> + HNode<O = I>,
        U2: HfGen<'a> + HNode<O = I>,
    {
        
        let (memos, hf_node1) = U1::gen(h_node1, memos);
        let (memos, hf_node2) = U2::gen(h_node2, memos);
        (memos, Box::new(HfPlusNode::Union(hf_node1, hf_node2)))
    }
}

impl<'a> HfGen<'a> for HExpr {
    fn gen(h_node: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        match h_node {
            HExpr::Raw(s) => HfGen::gen(s, memos),
            HExpr::Union(s) => HfGen::gen(s, memos),
            HExpr::Shared(s) => HfGen::gen(s, memos),
        }
    }
}

impl<'a> HfGen<'a> for Tagged<HExprRaw, Scope> {
    fn gen(
        Self(HExprRaw { input, expr, scope: in_scope }, out_scope): Self,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_map(
            input,
            memos,
            HFunc::newb(
                ScopePat::Destructured(in_scope),
                ExprPat::Destructured(ident("value"), ScopePat::Destructured(out_scope)),
                quote! { let value = #expr; },
            ),
        )
    }
}

impl<'a> HfGen<'a> for HExprUnion {
    fn gen(
        Self(box input1, box input2): Self,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_union(input1, input2, memos)
    }
}

impl<'a> HfGen<'a> for HExprShared {
    fn gen(Self(input): Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_tee(input, memos)
    }
}

impl<'a> HfGen<'a> for HScope {
    fn gen(h_node: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        match h_node {
            HScope::Input(s) => HfGen::gen(s, memos),
            HScope::Bind(s) => HfGen::gen(s, memos),
            HScope::Filter(s) => HfGen::gen(s, memos),
        }
    }
}

impl<'a> HfGen<'a> for HInput {
    fn gen(Self: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        // Todo: UPDATE THIS TO SUPPORT MORE THAN ONE INPUT
        let hf_input = memos.inputs.map.values().next().unwrap().clone();
        // can't use one of my i/o type-checked functions because this is just a raw input
        (memos, Box::new(HfPlusNode::Tee { inner: hf_input }))
    }
}

impl<'a> HfGen<'a> for Tagged<HBind, Scope> {
    fn gen(
        Tagged(HBind { id, box value }, scope): Self,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        // Todo: update this to support shadowing
        Self::gen_map(
            value,
            memos,
            HFunc::new(
                ExprPat::Destructured(id.clone(), ScopePat::Destructured(scope.without(&id))),
                ScopePat::Destructured(scope),
            ),
        )
    }
}

impl<'a> HfGen<'a> for HFilter {
    fn gen(
        Self { box cond, expectation }: Self,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        // Todo: standardize/fix idents
        Self::gen_filter_map(
            cond,
            memos,
            HFunc::newb(
                ExprPat::Destructured(ident("cond"), ScopePat::Ident(ident("scope"))),
                ScopePat::Ident(ident("scope")),
                quote! {
                    if cond != #expectation {
                        return None
                    }
                },
            ),
        )
    }
}

impl<'a> HfGen<'a> for HReturn {
    fn gen(Self { value }: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_map(
            value,
            memos,
            HFunc::new(
                ExprPat::Destructured(ident("value"), ScopePat::Ident(ident("_"))),
                ident("value"),
            ),
        )
    }
}

impl<'a> HfGen<'a> for HOutput {
    fn gen(Self { input, other }: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        match other {
            Some(box input2) => Self::gen_union(input, input2, memos),
            None => HfGen::gen(input, memos),
        }
    }
}
