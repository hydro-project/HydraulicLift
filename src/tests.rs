use hydroflow_plus::ir::HfPlusNode;

use crate::{compile, hfplus_ir::visualize::visualize};

macro_rules! test_compile {
    (let $input:ident = $hf_input:expr => $($body:tt)*) => {
        let hf = compile!(let $input = $hf_input => $($body)*);
        
        insta::assert_debug_snapshot!(hf);
        insta::assert_snapshot!(visualize(hf));
    };
}

#[test]
fn test_pipeline() {
    test_compile!(let hf_in = HfPlusNode::Placeholder => {
        let x = 1;
        let y = hf_in + 2;
        let z = x + y + 3;
        let o = z + 4;
        o + 5
    });
}

#[test]
fn test_return_simple() {
    test_compile!(let hf_in = HfPlusNode::Placeholder => {
        return hf_in;
    });
}

#[test]
fn test_return() {
    test_compile!(let hf_in = HfPlusNode::Placeholder => {
        let x = hf_in + 1;
        return x;
        x + 2 // this doesn't show up in the resulting HF+!
    });
}

#[test]
fn test_if_simple() {
    test_compile!(let hf_in = HfPlusNode::Placeholder => {
        if 1 == 2 {
            3
        } else {
            4
        }
    });
}