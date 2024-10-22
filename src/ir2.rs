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

// NEW
// nodes are just processing, they are externally linked

use syn::{parse_quote, Expr, Ident};

use crate::{io::Scope, utils::ident};

// matches: x | destructured_x
enum Pattern<T> {
    Ident(Ident),
    Destructure(T),
}

struct TokTup<T, U>(T, U);

// matches: scope | (a, b, c)
type ScopePattern = Pattern<Scope>;
// matches: expr | (value, scope) | (a, (b, c, d))
type ExprPattern = Pattern<TokTup<Ident, ScopePattern>>;

trait HNode<I, O> {}

struct HExpr {
    expr: Expr,
    scope: Scope,
}

impl HNode<ScopePattern, ExprPattern> for HExpr {}

struct HBlock<T: HNode<ScopePattern, U>, U> {
    stmt: HBind,
    eval: T,
}

impl<T: HNode<ScopePattern, U>, U> HNode<ScopePattern, ExprPattern> for HBlock<T, U> {}

struct HBind {
    definition: Ident,
    expr: Box<dyn HNode<ScopePattern, ExprPattern>>,
}

impl HNode<ScopePattern, ExprPattern> for HBind {}

fn test() {
    let out = {
        let x = 1;
        {
            let y = 2;
            x + y
        }
    };
    // input is a scope (will be wrapped around input value)

    let whole = HBlock {
        stmt: HBind {
            definition: ident("x"),
            expr: Box::new(HExpr {
                expr: parse_quote!(1),
                scope: Scope::empty(),
            }),
        },
        eval: HBlock {
            stmt: HBind {
                definition: ident("y"),
                expr: Box::new(HExpr {
                    expr: parse_quote!(2),
                    scope: Scope::empty().with(ident("x")),
                }),
            },
            eval: HExpr { expr: parse_quote!(x+y), scope: Scope::empty().with(ident("x")).with(ident("y")) },
        },
    };
}
