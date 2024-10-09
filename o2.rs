fn raw() {
    Block(RExprBlock {
        statements: [
            Raw("let x = 1 + 1 ;"),
            Expr(If(RExprIf {
                condition: Raw("(x < 5)"),
                then_block: RExprBlock {
                    statements: [
                        Raw("let z = 1 + 1 ;"),
                        LetAwait(RStmtLetAwait {
                            definition: "y",
                            future: Raw("a"),
                        }),
                        Expr(Raw("y + z")),
                    ],
                },
                else_block: Some(If(RExprIf {
                    condition: Raw("(x > 10)"),
                    then_block: RExprBlock {
                        statements: [
                            LetAwait(RStmtLetAwait {
                                definition: "z",
                                future: Raw("a"),
                            }),
                            Expr(Raw("z + x")),
                        ],
                    },
                    else_block: Some(Block(RExprBlock {
                        statements: [
                            LetAwait(RStmtLetAwait {
                                definition: "z",
                                future: Raw("a"),
                            }),
                            Return(RReturn {
                                value: Some(Raw("z")),
                            }),
                        ],
                    })),
                })),
            })),
            Expr(Raw("x + 2")),
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
