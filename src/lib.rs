mod romberg;

use meval::Expr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum EvalError {
    ParseError,
    DoesNotConverge,
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn integrate(input: &str, mut a: f64, mut b: f64, binding: &str) -> Result<f64, EvalError> {
    let mut multiply = 1.0;
    if a > b {
        std::mem::swap(&mut a, &mut b);
        multiply = -1.0;
    }
    let expr = input
        .parse::<Expr>()
        .map_err(|_err| EvalError::ParseError)?;
    let func = expr.bind(binding).map_err(|_err| EvalError::ParseError)?;
    romberg::integrate(func, a, b)
        .map(|x| x * multiply)
        .ok_or(EvalError::DoesNotConverge)
}

#[wasm_bindgen]
pub fn evaluate(input: &str) -> Result<f64, EvalError> {
    meval::eval_str(input).map_err(|_err| EvalError::ParseError)
}
