/// 
/// let x = i + 1;
/// if (x < 5) {
///     let z = 1+1;
///     let y = a.await;
///     y + z
/// } else {
///     let z = a.await;
///     return z;
/// }
/// x + 2
/// 
/// ->
/// 
/// input 
/// -> map(|i| { // raw stmt
///     let x = i + 1;
///     (x)
/// }) 
/// -> map(|(x))| { // raw expr
///     let _temp1 = x < 5;
///     (_temp1, in)
/// })
/// -> demux(|(_temp1, in)| {
///     _temp1 => then
///     !_temp1 => else
/// })
/// 
/// then = map(|(x))
