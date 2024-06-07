import init, {
  integrate,
  evaluate,
  set_panic_hook,
} from "./dist/romberg_integration.js";

const funcInput = document.getElementById("function");
const funcText = document.getElementById("funct");
const aInput = document.getElementById("A");
const bInput = document.getElementById("B");
const variableInput = document.getElementById("variable");

funcText.innerText = "d";

await init();
set_panic_hook();

function fallible(func) {
  try {
    return func();
  } catch (e) {
    switch (e) {
      case 0:
        throw new Error("parse error");
      case 1:
        throw new Error("does not converge");
      default:
        throw new Error("unknown error");
    }
  }
}

const calculate = document.getElementById("calculate");
calculate.onclick = () => {
  let result;
  try {
    const a = evaluate(aInput.value);
    const b = evaluate(bInput.value);
    const variable = variableInput.value;
    const func = funcInput.value;
    result = integrate(func, a, b, variable);
  } catch (e) {
    switch (e) {
      case 0:
        result = "parsing error";
        break;
      case 1:
        result = "DNE";
        break;
      default:
        break;
    }
  }
  funcText.innerText =
    funcInput.value +
    " d" +
    document.getElementById("variable").value +
    "  " +
    " = " +
    result;
};
