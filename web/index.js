import init, {
  integrate,
  evaluate,
  set_panic_hook,
} from "./dist/romberg_integration.js";

const funcInput = document.getElementById("function");
const aInput = document.getElementById("A");
const bInput = document.getElementById("B");
const variableInput = document.getElementById("variable");

document.getElementById("funct").innerText = "d";

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
  const a = fallible(() => evaluate(aInput.value));
  const b = fallible(() => evaluate(bInput.value));
  const variable = variableInput.value;
  const func = funcInput.value;
  const result = fallible(() => integrate(func, a, b, variable));
  console.log(result);
};
