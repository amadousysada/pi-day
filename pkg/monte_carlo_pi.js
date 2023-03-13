import * as wasm from "./monte_carlo_pi_bg.wasm";
import { __wbg_set_wasm } from "./monte_carlo_pi_bg.js";
__wbg_set_wasm(wasm);
export * from "./monte_carlo_pi_bg.js";
