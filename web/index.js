import init, { WasmChip8 } from "../pkg/chip8_rs.js";

async function main() {
  await init();

  const chip8 = new WasmChip8();
  const rom = document.getElementById("romInput");
  
}

main();