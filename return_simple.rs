fn input() {
    {
        let x = hf_in + 1;
        return x;
        x + 2
    }
}
fn HFPlus() {
    node0 = Placeholder;

    node0
        .MAP(|(hf_in)| {
            let value = hf_in + 1;
            (value, ())
        })
        .MAP(|(x, ())| (x))
        .MAP(|(x)| {
            let value = x;
            (value, ())
        })
        .MAP(|(value, _)| value)
}
