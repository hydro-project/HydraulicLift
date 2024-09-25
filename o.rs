fn main() {
    poll_future(async |a| {
        let x = 1 + 1;
        let b = a.await;
        (b, x)
    });
    poll_future(async |b, x| {
        let y = 1 + 2;
        let c = b.await;
        (y, x, c)
    });
    poll_future(async |y, x, c| {
        let z = x + y;
        let d = c.await;
        (z, x, d)
    });
    poll_future(async |z, x, d| {
        if z + 1 > 3 {
            let unused = x + 1;
            let asdf = unused + 1;
        }
        let e = d.await;
        (z)
    });
    map(|z| {
        let out = z + 2;
        out
    });
}
