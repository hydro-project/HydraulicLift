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
    ir2::{
        HBind, HExpr, HExprRaw, HExprShared, HExprUnion, HFilter, HInput, HOutput, HReturn, HScope,
    },
    r_ast::{RExpr, RExprBlock, RExprIf, RExprRaw, RStmt, RStmtLet, RStmtReturn},
    utils::{DebugStr, Tagged},
};

/// Transforms an RExpr tree into a HOutput node
impl From<RExpr<IO>> for HOutput {
    fn from(expr: RExpr<IO>) -> Self {
        let rail = expr.hinto(HScope::Input(HInput));

        match rail {
            HRail::Inner(inner) => HOutput::new(HReturn { input: inner }),
            HRail::Output(output) => output,
            HRail::Both(inner, output) => output.union(HOutput::new(HReturn { input: inner })),
        }
    }
}

/// Represents a H node which can evaluate to self or return
trait HFrom<T>: Sized {
    fn hfrom(value: T, input: HScope) -> HRail<Self>;
}

trait HInto<O: HFrom<Self>>: Sized {
    fn hinto(self, input: HScope) -> HRail<O>;
}

impl<T, O: HFrom<T>> HInto<O> for T {
    fn hinto(self, input: HScope) -> HRail<O> {
        O::hfrom(self, input)
    }
}

/// Tracks current node alongside early returns
enum HRail<T> {
    Inner(T),
    Output(HOutput),
    Both(T, HOutput),
}

impl<T> HRail<T> {
    fn empty(output: HOutput) -> Self {
        Self::Output(output)
    }

    fn union_output(self, other: HOutput) -> Self {
        match self {
            HRail::Inner(inner) => HRail::Both(inner, other),
            HRail::Output(output) => HRail::Output(output.union(other)),
            HRail::Both(inner, output) => HRail::Both(inner, output.union(other)),
        }
    }
}

/// Monad
impl<T> HRail<T> {
    /// Monad pure
    pub fn pure(inner: T) -> Self {
        Self::Inner(inner)
    }

    /// Monad bind
    pub fn and_then<U, F>(self, f: F) -> HRail<U>
    where
        F: FnOnce(T) -> HRail<U>,
    {
        match self {
            HRail::Inner(inner) => f(inner),
            HRail::Output(output) => HRail::Output(output),
            HRail::Both(inner, output) => f(inner).union_output(output),
        }
    }

    /// Functor map
    pub fn map<F, U>(self, f: F) -> HRail<U>
    where
        F: FnOnce(T) -> U,
    {
        self.and_then(|inner| HRail::pure(f(inner)))
    }
}

pub trait Unionable {
    fn union(self, other: Self) -> Self;
}

impl Unionable for HExpr {
    fn union(self, other: Self) -> Self {
        Self::Union(HExprUnion(Box::new(self), Box::new(other)))
    }
}

impl Unionable for HOutput {
    fn union(self, Self { input, other }: Self) -> Self {
        let new = self.with(input);
        match other {
            Some(box rest) => new.union(rest),
            None => new,
        }
    }
}

impl<T> Unionable for Option<T>
where
    T: Unionable,
{
    fn union(self, other: Self) -> Self {
        match (self, other) {
            (None, None) => None,
            (None, Some(t)) => Some(t),
            (Some(t), None) => Some(t),
            (Some(t1), Some(t2)) => Some(t1.union(t2)),
        }
    }
}

impl<T> Unionable for HRail<T>
where
    T: Unionable,
{
    fn union(self, other: Self) -> Self {
        match (self, other) {
            (HRail::Inner(i1), HRail::Inner(i2)) => Self::Inner(i1.union(i2)),
            (HRail::Inner(i), HRail::Output(o)) => Self::Both(i, o),
            (HRail::Inner(i1), HRail::Both(i2, o)) => Self::Both(i1.union(i2), o),
            (HRail::Output(o), HRail::Inner(i)) => Self::Both(i, o),
            (HRail::Output(o1), HRail::Output(o2)) => Self::Output(o1.union(o2)),
            (HRail::Output(o1), HRail::Both(i, o2)) => Self::Both(i, o1.union(o2)),
            (HRail::Both(i1, o), HRail::Inner(i2)) => Self::Both(i1.union(i2), o),
            (HRail::Both(i, o1), HRail::Output(o2)) => Self::Both(i, o1.union(o2)),
            (HRail::Both(i1, o1), HRail::Both(i2, o2)) => Self::Both(i1.union(i2), o1.union(o2)),
        }
    }
}

impl<T, U: HFrom<T>> HFrom<Tagged<T, IO>> for Tagged<U, IO> {
    fn hfrom(Tagged(inner, io): Tagged<T, IO>, input: HScope) -> HRail<Self> {
        inner.hinto(input).map(|inner| Tagged(inner, io))
    }
}

impl HFrom<RExpr<IO>> for HExpr {
    fn hfrom(value: RExpr<IO>, input: HScope) -> HRail<HExpr> {
        match value {
            RExpr::If(s) => s.hinto(input),
            RExpr::Block(s) => s.hinto(input),
            RExpr::Raw(s) => s.hinto(input).map(HExpr::Raw),
        }
    }
}

impl HFrom<RExprRaw> for HExprRaw {
    fn hfrom(RExprRaw(DebugStr(expr)): RExprRaw, input: HScope) -> HRail<Self> {
        HRail::pure(Self { input, expr })
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
        cond_expr.hinto(input).and_then(|cond| {
            let cond = Rc::<HExpr>::new(cond);
            let then_cond = HScope::Filter(HFilter {
                cond: Box::new(HExpr::Shared(HExprShared(cond.clone()))),
                expr: parse_quote!(true),
            });
            let else_cond = HScope::Filter(HFilter {
                cond: Box::new(HExpr::Shared(HExprShared(cond.clone()))),
                expr: parse_quote!(false),
            });
            then_expr.hinto(then_cond).union(else_expr.hinto(else_cond))
        })
    }
}

impl HFrom<RExprBlock<IO>> for HExpr {
    fn hfrom(RExprBlock { stmt, box expr }: RExprBlock<IO>, input: HScope) -> HRail<HExpr> {
        stmt.hinto(input).and_then(|stmt| expr.hinto(stmt))
    }
}

impl HFrom<RStmt<IO>> for HScope {
    fn hfrom(value: RStmt<IO>, input: HScope) -> HRail<HScope> {
        match value {
            RStmt::Let(s) => s.hinto(input).map(HScope::Bind),
            RStmt::Return(s) => s.hinto(input),
        }
    }
}

impl HFrom<RStmtLet<IO>> for HBind {
    fn hfrom(RStmtLet { id, box value }: RStmtLet<IO>, input: HScope) -> HRail<Self> {
        value.hinto(input).map(|value| HBind {
            input: Box::new(value),
            id,
        })
    }
}

impl<T> HFrom<RStmtReturn<IO>> for T {
    fn hfrom(RStmtReturn { box value }: RStmtReturn<IO>, input: HScope) -> HRail<Self> {
        value
            .hinto(input)
            .and_then(|value| HRail::empty(HOutput::new(HReturn { input: value })))
    }
}
