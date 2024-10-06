fn main() {
    poll_future(async |a| {
        let x = 1 + 1;
        let b = a.await;
        (x, b)
    });
    poll_future(async |x, b| {
        let y = 1 + 2;
        let c = b.await;
        (y, x, c)
    });
    poll_future(async |y, x, c| {
        let z = x + y;
        let d = c.await;
        (d, z, x)
    });
    poll_future(async |d, z, x| {
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
