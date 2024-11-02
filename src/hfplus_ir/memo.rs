use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use by_address::ByAddress;
use hydroflow_plus::ir::HfPlusNode;

use crate::{
    h_ir::ir::{HExpr, HInput},
    utils::functional::FakeFunctor,
};

use super::gen::HfGen;

/// Memoized conversions from Rc<K: HNode> to Rc<HfPlusNode>.
/// This allowes for constructing tees off of shared inputs
pub struct HfMemos<'a> {
    exprs: HfMemo<'a, HExpr>,
    inputs: HfMemo<'a, HInput>,
}

/// Memoized conversion from K to a HF+ node.
struct HfMemo<'a, K> {
    map: HashMap<Rc<K>, Rc<RefCell<HfPlusNode<'a>>>>,
}

pub trait HfMemoize<'a, K> 
where K: Hash + Eq + Clone {
    /// Get a clone of the corresponding value's rc pointer if present.
    fn get(&self, key: &Rc<K>) -> Option<Rc<RefCell<HfPlusNode<'a>>>>;

    /// Memoize a mapping.
    fn with(self, key: Rc<K>, value: HfPlusNode<'a>) -> Self;
}

impl<'a> HfMemos<'a> {
    pub fn new() -> Self {
        Self {
            exprs: HfMemo::new(),
            inputs: HfMemo::new(),
        }
    }
}

impl<'a, K> HfMemo<'a, K> where K: Hash + Eq + Clone {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<'a, K> HfMemoize<'a, K> for HfMemo<'a, K> 
where K: Hash + Eq + Clone {
    fn get(&self, key: &Rc<K>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.map.get(key).map(Clone::clone)
    }

    fn with(mut self, key: Rc<K>, value: HfPlusNode<'a>) -> Self {
        self.map
            .insert(key, Rc::new(RefCell::new(value)));
        self
    }
}

impl<'a> HfMemoize<'a, HExpr> for HfMemos<'a> {
    fn get(&self, key: &Rc<HExpr>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.exprs.get(key)
    }

    fn with(mut self, key: Rc<HExpr>, value: HfPlusNode<'a>) -> Self {
        self.exprs = self.exprs.with(key, value);
        self
    }
}

impl<'a> HfMemoize<'a, HInput> for HfMemos<'a> {
    fn get(&self, key: &Rc<HInput>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.inputs.get(key)
    }

    fn with(mut self, key: Rc<HInput>, value: HfPlusNode<'a>) -> Self {
        self.inputs = self.inputs.with(key, value);
        self
    }
}

pub trait HfMemoGen<'a, K: Hash+Eq>: HfMemoize<'a, K> + Sized 
where K: Hash + Eq + Clone {
    /// Either gets the memoized value, or generates and memoize it
    fn get_or_gen(self, key: Rc<K>) -> (Self, Rc<RefCell<HfPlusNode<'a>>>);
}

impl<'a, K> HfMemoGen<'a, K> for HfMemos<'a>
where
    HfMemos<'a>: HfMemoize<'a, K>,
    K: Hash+Eq + Clone + HfGen<'a>,
{
    fn get_or_gen(self, key: Rc<K>) -> (Self, Rc<RefCell<HfPlusNode<'a>>>) {
        match self.get(&key) {
            Some(value) => (self, value),
            None => K::gen((*key).clone(), self).map(|box node| Rc::new(RefCell::new(node))),
        }
    }
}
