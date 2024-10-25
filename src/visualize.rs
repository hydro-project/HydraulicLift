use std::{borrow::Borrow, collections::HashMap};

use hydroflow_plus::ir::HfPlusNode;
use quote::ToTokens;


// maybe rename to decompile
pub fn visualize(node: HfPlusNode) -> String {
    let mut memo = NodeMapping { map: HashMap::new(), i: 0 };
    let out = to_vis(&node, &mut memo);
    let mut out_str = String::new();
    for (name, val) in memo.map.values() {
        out_str.push_str(&format!("\n\n{}={};", name, val));
    }
    out_str.push_str(&format!("\n\n{}", out));
    out_str
}

/// Quick and dirty hack:
/// Mapping from addrress of HfNode to (node name, node visualization string)
struct NodeMapping {
    map: HashMap<usize, (String, String)>,
    i: u32
} 

impl NodeMapping {
    fn insert(&mut self, addr: usize, vis_string: String) {
        self.map.insert(addr, (format!("node{}", self.i), vis_string));
        self.i += 1;
    }
}

fn to_vis(node: &HfPlusNode, memo: &mut NodeMapping) -> String {
    match node {
        HfPlusNode::Placeholder => "Placeholder".to_string(),
        HfPlusNode::Tee { inner } => {
            let addr = (*inner).as_ptr() as usize;
            if let  None = memo.map.get(&addr) {
                let x = to_vis(&*inner.borrow_mut(), memo);
                memo.insert(addr, x);
            }
            memo.map.get(&addr).unwrap().0.clone()
        },
        HfPlusNode::Union(n1, n2) =>  {
            let x1 = to_vis(n1, memo);
            let x2 = to_vis(n2, memo);
            format!("UNION {{\n{}\n|{}}}", x1, x2)
        },
        HfPlusNode::Map { f, input } => {
            let x = to_vis(&input, memo);
            let f = f.to_token_stream();
            format!("{} \n\t. MAP({})", x, f)
        },
        HfPlusNode::FilterMap { f, input } => {
            let x = to_vis(&input, memo);
            let f = f.to_token_stream();
            format!("{} \n\t. FILTER_MAP({})", x, f)
        },
        _ => panic!("Visualizer doesn't support this hf+ node yet.")
    }
}