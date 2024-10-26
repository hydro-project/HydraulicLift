use hydroflow_plus::ir::HfPlusNode;

use crate::{compile, visualize::visualize};

#[test]
fn test_return_simple() {
    let output = compile!(let hf_in = HfPlusNode::Placeholder => {
        let x = hf_in + 1;
        return x;
        x + 2 // this doesn't show up in the resulting HF+!
    });
    
    insta::assert_debug_snapshot!(output);
    insta::assert_snapshot!(visualize(output));
}