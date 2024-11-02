// HNode utility for tagging h nodes with their "output types," 
// "Output types" are patterns (a value, a scope, an expression, etc.)

use crate::utils::{pattern::Pattern, tagged::Tagged};

/// Specifies the pattern which can match the output of the node.
pub trait HNode {
    type O: Pattern;
}

// Tagged<T, _> just passes T's output through
impl<T, O, M> HNode for Tagged<T, M>
where
    O: Pattern,
    T: HNode<O = O>,
{
    type O = O;
}

/// derive_hnode!(T, O) derives HNode for type T with output pattern O.
#[macro_export]
macro_rules! derive_hnode {
    ($t:ty: $o:ty ) => {
        impl super::node::HNode for $t {
            type O = $o;
        }
    };
}