// // use syn::Expr;

// // use crate::{io::IO, ir2::{ExprPat, HBlock, HExpr, HNode, HPattern, ScopePat}, r_ast::{RExpr, RExprBlock, RExprIf}, utils::{DebugStr, Tagged}};

// // impl From<RExpr<IO>> for Box<dyn HNode<I=ScopePat, O=dyn HPattern>> {
// //     fn from(value: RExpr<IO>) -> Self {
// //         match value {
// //             RExpr::If(s) => Box::new(HExpr::from(s)),
// //             RExpr::Block(s) => Box::new(HBlock::from(s)),
// //             RExpr::Raw(s) => Box::new(HExpr::from(s)),
// //         }
// //     }
// // }

// // impl From<Tagged<DebugStr<Expr>, IO>> for HExpr {
// //     fn from(Tagged(DebugStr(expr), IO { input_scope: scope, output_scope }): Tagged<DebugStr<Expr>, IO>) -> Self {
// //         Self { expr, scope }
// //     }
// // }

// // impl From<RExprIf<IO>> for HExpr {
// //     fn from(value: RExprIf<IO>) -> Self {
// //         todo!()
// //     }
// // }

// // impl<O: HPattern> From<RExprBlock<IO>> for HBlock<O> {
// //     fn from(RExprBlock { stmt, box return_expr }: RExprBlock<IO>) -> Self {
// //         let stmt = stmt.into();
// //         let eval = return_expr.into();
// //         Self { stmt, eval }
// //     }
// // }

// use std::ops::{ControlFlow, FromResidual, Try};

// use syn::Expr;

// use crate::{
//     io::IO,
//     ir2::{HBind, HExpr, HExprRaw, HReturn, HScope, HRail},
//     r_ast::{RExpr, RExprBlock, RExprIf, RStmt, RStmtLet, RStmtReturn},
//     utils::{DebugStr, Tagged},
// };

// // pub trait From2<T, I> {
// //     fn from2(value: T, input: I) -> Self;
// // }

// /// Represents a H node which can evaluate to self or return
// pub trait HFrom<T>: Sized {
//     fn hfrom(value: T, input: HScope) -> HRail<Self>;
// }

// pub trait HInto<O: HFrom<Self>>: Sized {
//     fn hinto(self, input: HScope) -> HRail<O>;
// }

// impl<T, O: HFrom<T>> HInto<O> for T {
//     fn hinto(self, input: HScope) -> HRail<O> {
//         O::hfrom(self, input)
//     }
// }

// impl<T> FromResidual<HReturn> for HRail<T> {
//     fn from_residual(residual: HReturn) -> Self {
//         HRail::Return(residual)
//     }
// }

// impl<T> Try for HRail<T> {
//     type Output = T;
//     type Residual = HReturn;

//     fn from_output(output: Self::Output) -> Self {
//         HRail::Inner(output)
//     }

//     fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
//         match self {
//             HRail::Inner(inner) => ControlFlow::Continue(inner),
//             HRail::Return(hreturn) => ControlFlow::Break(hreturn),
//         }
//     }
// }

// impl HFrom<RExpr<IO>> for HExpr {
//     fn hfrom(value: RExpr<IO>, input: HScope) -> HRail<HExpr> {
//         match value {
//             RExpr::If(s) => s.hinto(input),
//             RExpr::Block(s) => s.hinto(input),
//             RExpr::Raw(Tagged(
//                 DebugStr(expr),
//                 IO {
//                     input_scope,
//                     output_scope,
//                 },
//             )) => HRail::Inner(HExpr::Raw(HExprRaw { input, expr })),
//         }
//     }
// }

// impl HFrom<RExprIf<IO>> for HExpr {
//     fn hfrom(RExprIf { box cond_expr, box then_expr, box else_expr }: RExprIf<IO>, input: HScope) -> HRail<Self> {
//         HRail::Inner(HEx)
//     }
// }

// impl HFrom<RExprBlock<IO>> for HExpr {
//     fn hfrom(RExprBlock { stmt, box expr }: RExprBlock<IO>, input: HScope) -> HRail<HExpr> {
//         let stmt = stmt.hinto(input)?;
//         expr.hinto(stmt)
//     }
// }

// impl HFrom<RStmt<IO>> for HScope {
//     fn hfrom(value: RStmt<IO>, input: HScope) -> HRail<HScope> {
//         match value {
//             RStmt::Let(s) => s.hinto(input),
//             RStmt::Return(s) => s.hinto(input),
//         }
//     }
// }

// impl HFrom<Tagged<RStmtLet<IO>, IO>> for HScope {
//     fn hfrom(
//         Tagged(
//             RStmtLet { ident, box value },
//             IO {
//                 input_scope,
//                 output_scope,
//             },
//         ): Tagged<RStmtLet<IO>, IO>,
//         input: HScope,
//     ) -> HRail<Self> {
//         let value = value.hinto(input)?;
//         HRail::Inner(Self::Bind(HBind {
//             input: Box::new(value),
//             id: ident,
//         }))
//     }
// }

// impl<T> HFrom<RStmtReturn<IO>> for T {
//     fn hfrom(RStmtReturn { box value }: RStmtReturn<IO>, input: HScope) -> HRail<Self> {
//         let value = value.hinto(input)?;
//         HRail::Return(HReturn { input: value })
//     }
// }

use std::rc::Rc;

use syn::parse_quote;

use crate::{
    io::IO,
    ir2::{HBind, HExpr, HExprRaw, HExprUnion, HFilter, HInput, HOutput, HReturn, HScope},
    r_ast::{RExpr, RExprBlock, RExprIf, RStmt, RStmtLet, RStmtReturn},
    utils::{DebugStr, Tagged},
};

/// Represents a H node which can evaluate to self or return
pub trait HFrom<T>: Sized {
    fn hfrom(value: T, input: HScope) -> HRail<Self>;
}

pub trait HInto<O: HFrom<Self>>: Sized {
    fn hinto(self, input: HScope) -> HRail<O>;
}

impl<T, O: HFrom<T>> HInto<O> for T {
    fn hinto(self, input: HScope) -> HRail<O> {
        O::hfrom(self, input)
    }
}

/// Tracks current node alongside early returns
pub enum HRail<T> {
    Both(T, HOutput),
    Output(HOutput),
}

impl<T> HRail<T> {
    fn with_output(self, other: HOutput) -> Self {
        match self {
            Self::Output(output) => Self::Output(output.union(other)),
            Self::Both(inner, output) => Self::Both(inner, output.union(other)),
        }
    }
}

/// Monad
impl<T> HRail<T> {
    /// Monad pure
    pub fn pure(inner: T) -> Self {
        Self::Both(inner, HOutput::None)
    }
    /// Monad bind
    pub fn bind<U, F>(self, f: F) -> HRail<U>
    where
        F: FnOnce(T) -> HRail<U>,
    {
        match self {
            Self::Output(output) => HRail::Output(output),
            Self::Both(inner, output) => f(inner).with_output(output),
        }
    }
}

/// Monad+ implementation (only on HExpr because HExpr is itself a Monad+)
impl HRail<HExpr> {
    /// Monad+ mzero (Alternative empty)
    pub fn mzero() -> Self {
        Self::Output(HOutput::None)
    }
    /// Monad+ mplus (Alternative <|>)
    pub fn union(self, other: Self) -> Self {
        match (self, other) {
            (Self::Both(i1, o1), Self::Both(i2, o2)) => Self::Both(
                HExpr::Union(HExprUnion(Box::new(i1), Box::new(i2))), // could be made generic if T is Monad+?
                o1.union(o2),
            ),
            (Self::Both(i, o1), Self::Output(o2)) => Self::Both(i, o1.union(o2)),
            (Self::Output(o1), Self::Both(i, o2)) => Self::Both(i, o1.union(o2)),
            (Self::Output(o1), Self::Output(o2)) => Self::Output(o1.union(o2)),
        }
    }
}

impl HFrom<RExpr<IO>> for HExpr {
    fn hfrom(value: RExpr<IO>, input: HScope) -> HRail<HExpr> {
        match value {
            RExpr::If(s) => s.hinto(input),
            RExpr::Block(s) => s.hinto(input),
            RExpr::Raw(Tagged(
                DebugStr(expr),
                IO {
                    input_scope,
                    output_scope,
                },
            )) => HRail::pure(HExpr::Raw(HExprRaw { input, expr })),
        }
    }
}

impl HFrom<RExprIf<IO>> for HExpr {
    fn hfrom(
        RExprIf {
            box cond_expr,
            box then_expr,
            box else_expr,
        }: RExprIf<IO>,
        input: HScope,
    ) -> HRail<Self> {
        cond_expr.hinto(input).bind(|cond| {
            let cond = Rc::<HExpr>::new(cond);
            let then_cond = HScope::Filter(HFilter {
                cond: Box::new(HExpr::Shared(cond.clone())),
                expr: parse_quote!(true),
            });
            let else_cond = HScope::Filter(HFilter {
                cond: Box::new(HExpr::Shared(cond.clone())),
                expr: parse_quote!(false),
            });
            let then_expr = then_expr.hinto(then_cond);
            let else_expr = else_expr.hinto(else_cond);

            then_expr.union(else_expr)
        })
    }
}

impl HFrom<RExprBlock<IO>> for HExpr {
    fn hfrom(RExprBlock { stmt, box expr }: RExprBlock<IO>, input: HScope) -> HRail<HExpr> {
        stmt.hinto(input).bind(|stmt| expr.hinto(stmt))
    }
}

impl HFrom<RStmt<IO>> for HScope {
    fn hfrom(value: RStmt<IO>, input: HScope) -> HRail<HScope> {
        match value {
            RStmt::Let(s) => s.hinto(input),
            RStmt::Return(s) => s.hinto(input),
        }
    }
}

impl HFrom<Tagged<RStmtLet<IO>, IO>> for HScope {
    fn hfrom(
        Tagged(
            RStmtLet { id, box value },
            IO {
                input_scope,
                output_scope,
            },
        ): Tagged<RStmtLet<IO>, IO>,
        input: HScope,
    ) -> HRail<Self> {
        value.hinto(input).bind(|value| {
            HRail::pure(Self::Bind(HBind {
                input: Box::new(value),
                id,
            }))
        })
    }
}

impl<T> HFrom<RStmtReturn<IO>> for T {
    fn hfrom(RStmtReturn { box value }: RStmtReturn<IO>, input: HScope) -> HRail<Self> {
        value
            .hinto(input)
            .bind(|value| HRail::Output(HOutput::new().with(HReturn { input: value })))
    }
}
