import init, { evaluate } from "./dist/romberg_integration.js";

const funcInput = document.getElementById("function");
const aInput = document.getElementById("A");
const bInput = document.getElementById("B");
const variableInput = document.getElementById("variable");

document.getElementById("funct").innerText = "d";

await init();

const calcaulte = document.getElementById("calculate");
calcaulte.onclick = () => {
  const a = parseFloat(aInput.value);
  const b = parseFloat(bInput.value);
  const variable = variableInput.value;
  const func = funcInput.value;
  try {
    const result = evaluate(func, a, b, variable);
    console.log(result);
  } catch (e) {
    switch (e) {
      case 0:
        console.log("parse error");
        break;
      case 1:
        console.log("does not converge");
        break;
      default:
        console.log("unknown error");
        break;
    }
  }
};
