import * as wasm from "./pkg/monte_carlo_pi";

var slider = document.getElementById("myRange");
var output = document.getElementById("demo");
var result = document.getElementById("result");
result.innerHTML = wasm.compute_pi(slider.value)
output.innerHTML = slider.value;

slider.oninput = function() {
  output.innerHTML = this.value;
  result.innerHTML = wasm.compute_pi(this.value)
}