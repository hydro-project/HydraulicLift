use std::{cell::RefCell, hash::Hash, rc::Rc};

use hydroflow_plus::ir::HfPlusNode;
use syn::Expr;

use crate::{
    h_ir::node::HNode,
    utils::{
        functional::State,
        pattern::Pattern,
    },
};

use super::{
    func::{FilterMapFunc, MapFunc},
    memo::{HfMemoize, HfMemos},
};

/// State monad which passes memoization structure through.
/// Really we would want two layers of memoization monad transformers, but no HKT so this is unreasonable.
pub type HFState<'a> = State<'a, HfMemos<'a>, Box<HfPlusNode<'a>>>;

impl<'a> HFState<'a> {
    /// Memoizes the result of appling f to t as a state monad.
    /// Returns a tee to the underlying node.
    pub fn memo<H, F>(gen: F, h_node: Rc<H>) -> Self
    where
        H: 'a + Hash + Eq + Clone,
        F: 'a + FnOnce(H) -> Self,
        HfMemos<'a>: HfMemoize<'a, H>,
    {
        State::get()
            .and_then(|memos: HfMemos<'a>| match memos.get(&h_node) {
                Some(hf_node) => State::pure(hf_node),
                None => gen((*h_node).clone()).and_then(|box hf_node| {
                    let hf_node = Rc::new(RefCell::new(hf_node));
                    let memos = memos.with(h_node, hf_node.clone());
                    State::put(memos).map_const(hf_node)
                }),
            })
            .map(|hf_node| Box::new(HfPlusNode::Tee { inner: hf_node }))
    }
}

/// Implement this for an HNode to convert it to hydroflow+.
/// Additional traits will automatically be implemented,
/// allowing type-safe (with respect to node output patterns)
/// hydroflow+ nodes to be generated.
pub trait HfGen<'a>: HNode {
    /// Generates Hydroflow+ from h_node.
    /// State contains memoization information to avoid repeating generation of shared inputs.
    fn gen(h_node: Self) -> HFState<'a>;
}

// The rest of this file contains generator traits which will allow
// specific HfPlusNodes to be created safely.

pub trait HfGenMap<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: 'static + Pattern,
{
    /// Generate a node which maps over h_node.
    fn gen_map<U, I>(h_node: U, func: MapFunc<I, O>) -> HFState<'a>
    where
        U: HfGen<'a> + HNode<O = I>,
        I: 'static + Pattern;
}

impl<'a, T, O> HfGenMap<'a, O> for T
where
    O: 'static + Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_map<U, I>(h_node: U, func: MapFunc<I, O>) -> HFState<'a>
    where
        U: HfGen<'a> + HNode<O = I>,
        I: 'static + Pattern,
    {
        U::gen(h_node)
            .map(|hf_node| HfPlusNode::Map {
                f: Expr::from(func).into(),
                input: hf_node,
            })
            .map(Box::new)
    }
}

pub trait HfGenFilterMap<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: 'a + Pattern,
{
    /// Generate a node which filter maps over h_node.
    /// The body of func should handle returning None.
    fn gen_filter_map<U, I>(h_node: U, func: FilterMapFunc<I, O>) -> HFState<'a>
    where
        U: HfGen<'a> + HNode<O = I>,
        I: 'a + Pattern;
}

impl<'a, T, O> HfGenFilterMap<'a, O> for T
where
    O: 'a + Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_filter_map<U, I: Pattern>(h_node: U, func: FilterMapFunc<I, O>) -> HFState<'a>
    where
        U: HfGen<'a> + HNode<O = I>,
        I: 'a + Pattern,
    {
        U::gen(h_node)
            .map(|hf_node| HfPlusNode::FilterMap {
                f: Expr::from(func).into(),
                input: hf_node,
            })
            .map(Box::new)
    }
}

pub trait HfGenTee<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: 'a + Pattern,
{
    /// Generate a node which tees over a shared node.
    /// Does this by either
    /// 1) getting the generated node from memos,
    /// or 2) generating it and memoizing it into memos.
    fn gen_tee<U>(h_node: Rc<U>) -> HFState<'a>
    where
        U: 'a + HfGen<'a> + HNode<O = O> + Eq + Hash + Clone,
        HfMemos<'a>: HfMemoize<'a, U>;
}

impl<'a, T, O> HfGenTee<'a, O> for T
where
    O: 'a + Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_tee<U>(h_node: Rc<U>) -> HFState<'a>
    where
        U: 'a + HfGen<'a> + HNode<O = O> + Eq + Hash + Clone,
        HfMemos<'a>: HfMemoize<'a, U>,
    {
        HFState::memo(U::gen, h_node)
    }
}

pub trait HfGenUnion<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: Pattern,
{
    /// Generate a node merges two input streams.
    fn gen_union<U1, U2, I: Pattern>(h_node1: U1, h_node2: U2) -> HFState<'a>
    where
        U1: 'a + HfGen<'a> + HNode<O = I>,
        U2: 'a + HfGen<'a> + HNode<O = I>;
}

impl<'a, T, O> HfGenUnion<'a, O> for T
where
    O: Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_union<U1, U2, I: Pattern>(h_node1: U1, h_node2: U2) -> HFState<'a>
    where
        U1: 'a + HfGen<'a> + HNode<O = I>,
        U2: 'a + HfGen<'a> + HNode<O = I>,
    {
        U1::gen(h_node1).and_then(|hf_node1| {
            U2::gen(h_node2).map(|hf_node2| Box::new(HfPlusNode::Union(hf_node1, hf_node2)))
        })
    }
}
