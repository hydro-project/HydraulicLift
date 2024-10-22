// use hydroflow_plus::ir::DebugExpr;
// use quote::ToTokens;
// use syn::{parse_quote, Expr, Ident};

// use crate::io::Scope;

// /// Hydroflow function, input and output of dataflow sub-tree.
// trait HF<I, O> {}

// /// Generate a closure 
// trait HfGen<I, O> {
//     fn hf(input_pattern: I, body: Expr, output_pattern: O) -> DebugExpr;
// }

// impl<T, I, O> HfGen<I, O> for T
// where
//     T: HF<I, O>,
//     I: ToTokens,
//     O: ToTokens,
// {
//     fn hf(input_pattern: I, body: Expr, output_pattern: O) -> DebugExpr {
//         let expr: Expr = parse_quote! {
//             |#input_pattern| {
//                 #body
//                 #output_pattern
//             }
//         };
//         expr.into()
//     }
// }

// /// Input to the whole tree
// /// :: (value, scope=[])
// struct HInput;

// /// Highest layer output, return value.
// /// :: (value, scope) -> value
// struct HReturn {
//     input: HExpr
// }

// /// :: scope -> (value, scope)
// struct HExpr {
//     input: HScope,
//     expr: Expr,
//     scope: Scope
// }

// // :: (value, scope) -> scope
// struct HBind {
//     scope: Scope,
//     definition: Ident,
//     input: Box<HExpr>
// }

// // :: scope -> scope
// struct HBlockEnd {
//     scope: Scope,
//     input: Box<HScope>
// }

// // :: _ -> scope
// enum HScope {
//     Bind(HBind),
//     BlockEnd(HBlockEnd)
// }

// // :: _ -> value | (value, scope)
// enum HExprReturn {
//     Block(HBlock),
//     Return(HReturn)
// }

// /// :: scope -> value | (value, scope)
// enum HBlock {
//     input: HScope
// }


// /*
// {
//     let x = input;
//     x + 1
// }

//     HReturn(HExpr(Bind(x, HInput)))

// {
//     return input;
//     x + 1
// }

//     HReturn(HInput)

// {
//     let x = input;
//     if x > 1 {
//         return x;
//     }
//     x + 1
// }
//     Bind(x, HInput)

// */

// NEW
// scopes contain their inputs