// use syn::Expr;

// use crate::{io::IO, ir2::{ExprPat, HBlock, HExpr, HNode, HPattern, ScopePat}, r_ast::{RExpr, RExprBlock, RExprIf}, utils::{DebugStr, Tagged}};

// impl From<RExpr<IO>> for Box<dyn HNode<I=ScopePat, O=dyn HPattern>> {
//     fn from(value: RExpr<IO>) -> Self {
//         match value {
//             RExpr::If(s) => Box::new(HExpr::from(s)),
//             RExpr::Block(s) => Box::new(HBlock::from(s)),
//             RExpr::Raw(s) => Box::new(HExpr::from(s)),
//         }
//     }
// }

// impl From<Tagged<DebugStr<Expr>, IO>> for HExpr {
//     fn from(Tagged(DebugStr(expr), IO { input_scope: scope, output_scope }): Tagged<DebugStr<Expr>, IO>) -> Self {
//         Self { expr, scope }
//     }
// }

// impl From<RExprIf<IO>> for HExpr {
//     fn from(value: RExprIf<IO>) -> Self {
//         todo!()
//     }
// }

// impl<O: HPattern> From<RExprBlock<IO>> for HBlock<O> {
//     fn from(RExprBlock { stmt, box return_expr }: RExprBlock<IO>) -> Self {
//         let stmt = stmt.into();
//         let eval = return_expr.into();
//         Self { stmt, eval }
//     }
// }

use std::ops::{ControlFlow, FromResidual, Try};

use syn::Expr;

use crate::{
    io::IO,
    ir2::{HBind, HExpr, HExprRaw, HReturn, HScope, Hor},
    r_ast::{RExpr, RExprBlock, RExprIf, RStmt, RStmtLet, RStmtReturn},
    utils::{DebugStr, Tagged},
};

// pub trait From2<T, I> {
//     fn from2(value: T, input: I) -> Self;
// }

/// Represents a H node which can evaluate to self or return
pub trait HFrom<T>: Sized {
    fn hfrom(value: T, input: HScope) -> Hor<Self>;
}

pub trait HInto<O: HFrom<Self>>: Sized {
    fn hinto(self, input: HScope) -> Hor<O>;
}

impl<T, O: HFrom<T>> HInto<O> for T {
    fn hinto(self, input: HScope) -> Hor<O> {
        O::hfrom(self, input)
    }
}

impl<T> FromResidual<HReturn> for Hor<T> {
    fn from_residual(residual: HReturn) -> Self {
        Hor::Return(residual)
    }
}

impl<T> Try for Hor<T> {
    type Output = T;
    type Residual = HReturn;

    fn from_output(output: Self::Output) -> Self {
        Hor::Inner(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Hor::Inner(inner) => ControlFlow::Continue(inner),
            Hor::Return(hreturn) => ControlFlow::Break(hreturn),
        }
    }
}

impl HFrom<RExpr<IO>> for HExpr {
    fn hfrom(value: RExpr<IO>, input: HScope) -> Hor<HExpr> {
        match value {
            RExpr::If(s) => s.hinto(input),
            RExpr::Block(s) => s.hinto(input),
            RExpr::Raw(Tagged(
                DebugStr(expr),
                IO {
                    input_scope,
                    output_scope,
                },
            )) => Hor::Inner(HExpr::Raw(HExprRaw { input, expr })),
        }
    }
}

impl HFrom<RExprIf<IO>> for HExpr {
    fn hfrom(RExprIf { box cond_expr, box then_expr, box else_expr }: RExprIf<IO>, input: HScope) -> Hor<Self> {
        Hor::Inner(HEx)
    }
}

impl HFrom<RExprBlock<IO>> for HExpr {
    fn hfrom(RExprBlock { stmt, box expr }: RExprBlock<IO>, input: HScope) -> Hor<HExpr> {
        let stmt = stmt.hinto(input)?;
        expr.hinto(stmt)
    }
}

impl HFrom<RStmt<IO>> for HScope {
    fn hfrom(value: RStmt<IO>, input: HScope) -> Hor<HScope> {
        match value {
            RStmt::Let(s) => s.hinto(input),
            RStmt::Return(s) => s.hinto(input),
        }
    }
}

impl HFrom<Tagged<RStmtLet<IO>, IO>> for HScope {
    fn hfrom(
        Tagged(
            RStmtLet { ident, box value },
            IO {
                input_scope,
                output_scope,
            },
        ): Tagged<RStmtLet<IO>, IO>,
        input: HScope,
    ) -> Hor<Self> {
        let value = value.hinto(input)?;
        Hor::Inner(Self::Bind(HBind {
            input: Box::new(value),
            id: ident,
        }))
    }
}

impl<T> HFrom<RStmtReturn<IO>> for T {
    fn hfrom(RStmtReturn { box value }: RStmtReturn<IO>, input: HScope) -> Hor<Self> {
        let value = value.hinto(input)?;
        Hor::Return(HReturn { input: value })
    }
} 