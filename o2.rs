fn raw() {
    Block(RExprBlock {
        stmt: Expr(Raw(DebugStr { inner: "()" })),
        return_expr: Block(RExprBlock {
            stmt: Raw(DebugStr {
                inner: "let x = 1 + 1 ;",
            }),
            return_expr: Block(RExprBlock {
                stmt: Expr(If(RExprIf {
                    condition: Raw(DebugStr { inner: "(x < 5)" }),
                    then_block: RExprBlock {
                        stmt: Expr(Raw(DebugStr { inner: "()" })),
                        return_expr: Block(RExprBlock {
                            stmt: Raw(DebugStr {
                                inner: "let z = 1 + 1 ;",
                            }),
                            return_expr: Block(RExprBlock {
                                stmt: LetAwait(RStmtLetAwait {
                                    definition: Ident { sym: y },
                                    future: Raw(DebugStr { inner: "a" }),
                                }),
                                return_expr: Raw(DebugStr { inner: "y + z" }),
                            }),
                        }),
                    },
                    else_block: Some(If(RExprIf {
                        condition: Raw(DebugStr { inner: "(x > 10)" }),
                        then_block: RExprBlock {
                            stmt: Expr(Raw(DebugStr { inner: "()" })),
                            return_expr: Block(RExprBlock {
                                stmt: LetAwait(RStmtLetAwait {
                                    definition: Ident { sym: z },
                                    future: Raw(DebugStr { inner: "a" }),
                                }),
                                return_expr: Raw(DebugStr { inner: "z + x" }),
                            }),
                        },
                        else_block: Some(Block(RExprBlock {
                            stmt: Expr(Raw(DebugStr { inner: "()" })),
                            return_expr: Block(RExprBlock {
                                stmt: LetAwait(RStmtLetAwait {
                                    definition: Ident { sym: z },
                                    future: Raw(DebugStr { inner: "a" }),
                                }),
                                return_expr: Block(RExprBlock {
                                    stmt: Return(RReturn {
                                        value: Raw(DebugStr { inner: "z" }),
                                    }),
                                    return_expr: Raw(DebugStr { inner: "()" }),
                                }),
                            }),
                        })),
                    })),
                })),
                return_expr: Raw(DebugStr { inner: "x + 2" }),
            }),
        }),
    })
}
