fn HFPlus() {
    Map {
        f: |(value, _)| value,
        input: Map {
            f: |(x)| {
                let value = x + 1;
                (value, ())
            },
            input: Map {
                f: |(x, ())| (x),
                input: Map {
                    f: |()| {
                        let value = 5;
                        (value, ())
                    },
                    input: Map {
                        f: |(_, ())| (),
                        input: Map {
                            f: |()| {
                                let value = ();
                                (value, ())
                            },
                            input: Map {
                                f: |(_, (hf_in))| (),
                                input: Map {
                                    f: |(hf_in)| {
                                        let value = hf_in;
                                        (value, ())
                                    },
                                    input: Map {
                                        f: |(_, (hf_in))| (hf_in),
                                        input: Map {
                                            f: |(hf_in)| {
                                                let value = ();
                                                (value, (hf_in))
                                            },
                                            input: Tee {
                                                inner: RefCell { value: Placeholder },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    }
}
