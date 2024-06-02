mod romberg;

use meval::Expr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, romberg-integration!");
}

#[wasm_bindgen]
pub enum Error {
    ParseError,
    DoesNotConverge,
}

#[wasm_bindgen]
pub fn evaluate(input: &str, mut a: f64, mut b: f64, binding: &str) -> Result<f64, Error> {
    let mut multiply = 1.0;
    if a > b {
        std::mem::swap(&mut a, &mut b);
        multiply = -1.0;
    }
    let expr = input.parse::<Expr>().map_err(|_err| Error::ParseError)?;
    let func = expr.bind(binding).map_err(|_err| Error::ParseError)?;
    romberg::integrate(func, a, b)
        .map(|x| x * multiply)
        .ok_or(Error::DoesNotConverge)
}
