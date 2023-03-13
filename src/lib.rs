mod utils;
use rand::{thread_rng, Rng};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, monte-carlo-pi!");
}

#[wasm_bindgen]
pub fn compute_pi(n: i32) -> f64{
    let mut count: i32 = 0;
    let mut random = thread_rng();

    for _i in 0..n{
        let x: f64 = random.gen::<f64>();
        let y: f64 = random.gen::<f64>();
        if x * x + y * y <= 1.0{
            count += 1;
        }
    }
    4.0*(count as f64 / n as f64)
}