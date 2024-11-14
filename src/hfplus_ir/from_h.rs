use std::{cell::RefCell, rc::Rc};

use hydroflow_plus::{
    ir::{HfPlusLeaf, HfPlusNode},
    location::LocationId,
};
use quote::quote;

use crate::{
    h_ir::{ir::*, node::HNode},
    utils::{
        functional::{SequenceState, State},
        idents::ident,
        pattern::{ExprPat, ScopePat},
        scope::Scope,
        tagged::TagOut,
    },
};

use super::{
    func::{FilterMapFunc, MapAsyncFunc, MapFunc},
    gen::*,
    memo::{HfMemoize, HfMemos},
};

/// Generates hydroflow+ node from hnode, which consumes the specified input.
pub fn generate_hf<'a>(
    (h_sinks, h_node): (Vec<HCycleSink>, HOutput),
    input: HfPlusNode<'a>,
) -> (Vec<HfPlusLeaf<'a>>, HfPlusNode<'a>) {
    let memos = HfMemos::new().with(Rc::new(HInput), Rc::new(RefCell::new(input)));

    // let hf_node = *HOutput::gen(h_node).run(memos).1;
    // let hf_leaves = h_sinks.into_iter().scan(initial_state, f);

    HOutput::gen(h_node)
        .and_then(|box hf_node| {
            h_sinks
                .into_iter()
                .map(gen_leaf)
                .sequence()
                .map(|hf_sinks| (hf_sinks, hf_node))
        })
        .run(memos)
        .1
}

fn gen_leaf<'a>(HCycleSink { scope, ident }: HCycleSink) -> State<'a, HfMemos<'a>, HfPlusLeaf<'a>> {
    HfGen::gen(scope).map(|hf_scope| HfPlusLeaf::CycleSink {
        input: hf_scope,
        ident,
        location_kind: LocationId::Process(0),
    })
}

impl<'a> HfGen<'a> for HExpr {
    fn gen(h_node: Self) -> HFS<'a> {
        match h_node {
            HExpr::Raw(s) => HfGen::gen(s),
            HExpr::Await(s) => HfGen::gen(s),
            HExpr::Union(s) => HfGen::gen(s),
            HExpr::Shared(s) => HfGen::gen(s),
        }
    }
}

impl<'a> HfGen<'a> for TagOut<HExprRaw, Scope> {
    fn gen(
        Self(
            HExprRaw {
                input,
                expr,
                scope_def: in_scope,
            },
            out_scope,
        ): Self,
    ) -> HFS<'a> {
        let val_id = ident("value");
        Self::gen_map(
            input,
            MapFunc::newb(
                ScopePat::DestructuredDef(in_scope),
                ExprPat::Destructured(val_id.clone(), ScopePat::Destructured(out_scope)),
                quote! { let #val_id = #expr; },
            ),
        )
    }
}

impl<'a> HfGen<'a> for HExprAwait {
    fn gen(Self(box h_expr): Self) -> HFS<'a> {
        let scope_id = ident("scope");
        let fut_id = ident("future");
        let val_id = ident("value");
        Self::gen_map_async(
            h_expr,
            MapAsyncFunc::newb(
                ExprPat::Destructured(fut_id.clone(), ScopePat::Ident(scope_id.clone())),
                ExprPat::Destructured(val_id.clone(), ScopePat::Ident(scope_id)),
                quote! {
                    let #val_id = #fut_id.await;
                },
            ),
        )
    }
}

impl<'a, T> HfGen<'a> for HUnion<T> 
    where T: 'a + HNode + HfGen<'a>{
    fn gen(Self(box h_in1, box h_in2): Self) -> HFS<'a> {
        Self::gen_union(h_in1, h_in2)
    }
}

impl<'a> HfGen<'a> for HExprShared {
    fn gen(Self(h_input): Self) -> HFS<'a> {
        Self::gen_tee(h_input)
    }
}

impl<'a> HfGen<'a> for HScope {
    fn gen(h_node: Self) -> HFS<'a> {
        match h_node {
            Self::Expr(s) => HfGen::gen(s),
            Self::Input(s) => HfGen::gen(s),
            Self::Bind(s) => HfGen::gen(s),
            Self::Filter(s) => HfGen::gen(s),
            Self::Union(s) => HfGen::gen(s),
            Self::CycleSource(s) => HfGen::gen(s),
        }
    }
}

impl<'a> HfGen<'a> for HDropExpr {
    fn gen(Self { box expr }: Self) -> HFS<'a> {
        let scope_id = ident("scope");
        Self::gen_map(
            expr,
            MapFunc::new(
                ExprPat::Destructured(ident("_"), ScopePat::Ident(scope_id.clone())),
                ScopePat::Ident(scope_id),
            ),
        )
    }
}

impl<'a> HfGen<'a> for TagOut<HInput, Scope> {
    fn gen(TagOut(h_node, outs): Self) -> HFS<'a> {
        // Check if input is used or discarded
        let ins = if outs.is_empty() {
            ScopePat::Ident(ident("_"))
        } else {
            ScopePat::Destructured(outs.clone())
        };
        Self::gen_map(h_node, MapFunc::new(ins, ScopePat::Destructured(outs)))
    }
}

impl<'a> HfGen<'a> for HCycleSource {
    fn gen(Self(ident): Self) -> HFS<'a> {
        HFS::pure(Box::new(HfPlusNode::CycleSource { ident, location_kind: LocationId::Process(0)}))
    }
}

impl<'a> HfGen<'a> for HInput {
    fn gen(h_node: Self) -> HFS<'a> {
        HFS::memo(
            |_| panic!("Did not memoize hydroflow input"),
            Rc::new(h_node),
        )
    }
}

impl<'a> HfGen<'a> for TagOut<HBind, Scope> {
    fn gen(TagOut(HBind { id, box value }, scope): Self) -> HFS<'a> {
        // Todo: update this to support shadowing
        Self::gen_map(
            value,
            MapFunc::new(
                ExprPat::Destructured(
                    id.clone(),
                    ScopePat::Destructured(scope.clone().without(&id)),
                ),
                ScopePat::Destructured(scope),
            ),
        )
    }
}

impl<'a> HfGen<'a> for HFilter {
    fn gen(
        Self {
            box cond,
            expectation,
        }: Self,
    ) -> HFS<'a> {
        let cond_id = ident("cond");
        let scope_id = ident("scope");
        // Todo: standardize/fix idents
        Self::gen_filter_map(
            cond,
            FilterMapFunc::newb(
                ExprPat::Destructured(cond_id.clone(), ScopePat::Ident(scope_id.clone())),
                ScopePat::Ident(scope_id),
                quote! {
                    if #cond_id != #expectation {
                        return None
                    }
                },
            ),
        )
    }
}

impl<'a> HfGen<'a> for HReturn {
    fn gen(Self { value }: Self) -> HFS<'a> {
        Self::gen_map(
            value,
            MapFunc::new(
                ExprPat::Destructured(ident("value"), ScopePat::Ident(ident("_"))),
                ident("value"),
            ),
        )
    }
}

impl<'a> HfGen<'a> for HOutput {
    fn gen(value: Self) -> HFS<'a> {
        match value {
            HOutput::Return(s) => HfGen::gen(s),
            HOutput::Union(s) => HfGen::gen(s),
        }
    }
}
