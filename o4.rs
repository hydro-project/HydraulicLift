fn test() {
    Map {
        f: |(value, _)| value,
        input: Map {
            f: |(hi,)| {
                let value = hi;
                ((hi,), value)
            },
            input: Map {
                f: |(value, ())| {
                    let hi = value;
                    (hi,)
                },
                input: Map {
                    f: |value| (value, ()),
                    input: Placeholder,
                },
            },
        },
    }
}
