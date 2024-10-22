// use hydroflow_plus::ir::HfPlusNode;
// use quote::{quote, ToTokens};
// use syn::{Expr, Ident};

// use crate::{io::Scope, utils::{ident}};

// /// Return of a value to the outer hf+ program.
// /// :: (value, scope) -> value
// pub struct HReturn;

// /// Evaluation of an expression into a value.
// /// :: scope -> (value, scope)
// pub struct HExprRaw {
//     pub expr: Expr,
//     pub scope: Scope,
//     pub next: HExprConsumer,
// }

// /// Consumer of an expression.
// /// :: (value, scope) -> _
// pub enum HExprConsumer {
//     Map(HExprMap),
//     Bind(HExprBind),
//     Branch(HExprBranch),
//     BlockEnd(HBlockEnd),
//     Return(HReturn),
// }

// /// Mapping a function over an expression.
// /// :: (value, scope) -> (value, scope)
// pub struct HExprMap {
//     pub map_expr: Expr,
//     pub scope: Scope,
//     pub next: Box<HExprConsumer>,
// }

// /// Bind an expression value to a name in scope.
// /// :: (value, scope) -> scope U value
// pub struct HExprBind {
//     pub definition: Ident,
//     /// Does not contain ident
//     pub scope: Scope,
//     pub next: Box<HExprRaw>,
// }

// /// Branches based on a boolean expression value.
// /// :: (true, scope)  -> scope
// /// :: (false, scope) -> scope
// pub struct HExprBranch {
//     pub next_true: Box<HExprRaw>,
//     pub next_false: Box<HExprRaw>,
// }

// /// The end of a block.
// /// :: (value, old_scope) -> (value, new_scope)
// pub struct HBlockEnd {
//     pub old_scope: Scope,
//     pub new_scope: Scope,
//     pub next: Box<HExprConsumer>,
// }

// /// Entrypoint, turns input into expression value.
// /// :: value -> (value, [])
// pub struct HEntryPoint {
//     pub next: HExprConsumer
// }



// /*

// let x = 5;
// let y = if x > 5 {
//     return x + 1;
// } else {
//     x + 2
// }
// x + y


// =>

// PROBLEM: 

// expr(5) -> bind(x) -> expr(x > 5) -> tee()

// true? -> expr(x+1) -> return
// false? -> expr(x+2) -> ret_expr

// union() -> bind(y) -> expr(x+y)
// */
