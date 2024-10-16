fn raw() {
    Block(AExprBlock {
        statements: [
            Raw(DebugStr {
                inner: "let x = 1 + 1 ;",
            }),
            Expr(If(AExprIf {
                condition: Raw(DebugStr { inner: "(x < 5)" }),
                then_block: AExprBlock {
                    statements: [
                        Raw(DebugStr {
                            inner: "let z = 1 + 1 ;",
                        }),
                        LetAwait(AStmtLetAwait {
                            definition: DebugStr { inner: "y" },
                            future: Raw(DebugStr { inner: "a" }),
                        }),
                        Expr(Raw(DebugStr { inner: "y + z" })),
                    ],
                },
                else_block: Some(If(AExprIf {
                    condition: Raw(DebugStr { inner: "(x > 10)" }),
                    then_block: AExprBlock {
                        statements: [
                            LetAwait(AStmtLetAwait {
                                definition: DebugStr { inner: "z" },
                                future: Raw(DebugStr { inner: "a" }),
                            }),
                            Expr(Raw(DebugStr { inner: "z + x" })),
                        ],
                    },
                    else_block: Some(Block(AExprBlock {
                        statements: [
                            LetAwait(AStmtLetAwait {
                                definition: DebugStr { inner: "z" },
                                future: Raw(DebugStr { inner: "a" }),
                            }),
                            Return(AReturn {
                                value: Some(Raw(DebugStr { inner: "z" })),
                            }),
                        ],
                    })),
                })),
            })),
            Expr(Raw(DebugStr { inner: "x + 2" })),
        ],
    })
}
fn main() {
    {
        let x = 1 + 1;
        if (x < 5) {
            let z = 1 + 1;
            let y = a.await;
            y + z
        }
        if (x > 10) {
            let z = a.await;
            z + x
        }
        {
            let z = a.await;
            return z;
        }
        x + 2
    }
}
