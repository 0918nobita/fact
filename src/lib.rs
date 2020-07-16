use std::convert::identity;

fn fact_cps<R: 'static>(n: u32, k: Box<dyn FnOnce(u32) -> R>) -> R {
    if n == 0 {
        k(1)
    } else {
        fact_cps(n - 1, Box::new(move |x| k(n * x)))
    }
}

#[no_mangle]
pub extern "C" fn fact(n: u32) -> u32 {
    fact_cps(n, Box::new(identity))
}
