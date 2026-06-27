import init, { WasmChip8 } from "../pkg/chip8_rs.js";

const wasm = await init();

const chip8 = new WasmChip8();

const romInput = document.getElementById("romInput");
const scaleInput = document.getElementById("videoScale");
const delayInput = document.getElementById("msDelay");
const playButton = document.getElementById("playButton");
const canvas = document.getElementById("screen");
const ctx = canvas.getContext("2d");

let rom = null;
let scale = -1;
let delay = -1;

let running = false;
let inputRegistered = false;
let cpuIntervalId = null;
let animationFrameId = null;

romInput.addEventListener("change", async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  const buffer = await file.arrayBuffer();
  rom = new Uint8Array(buffer);
});

scaleInput.addEventListener("change", (event) => {
  const value = event.target.value;
  if (!value) return;

  scale = parseInt(value, 10);
});

delayInput.addEventListener("change", (event) => {
  const value = event.target.value;
  if (!value) return;

  delay = parseInt(value, 10);
});

function stopEmulator() {
  running = false;

  if (cpuIntervalId !== null) {
    clearInterval(cpuIntervalId);
    cpuIntervalId = null;
  }

  if (animationFrameId !== null) {
    cancelAnimationFrame(animationFrameId);
    animationFrameId = null;
  }
}

function registerInputHandlers(chip8) {
  const keyMap = {
    x: 0x0,
    1: 0x1,
    2: 0x2,
    3: 0x3,
    q: 0x4,
    w: 0x5,
    e: 0x6,
    a: 0x7,
    s: 0x8,
    d: 0x9,
    z: 0xA,
    c: 0xB,
    4: 0xC,
    r: 0xD,
    f: 0xE,
    v: 0xF,
  };

  window.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      stopEmulator();
      return;
    }

    const key = keyMap[event.key.toLowerCase()];

    if (key !== undefined) {
      chip8.key_down(key);
      event.preventDefault();
    }
  });

  window.addEventListener("keyup", (event) => {
    const key = keyMap[event.key.toLowerCase()];

    if (key !== undefined) {
      chip8.key_up(key);
      event.preventDefault();
    }
  });
}

function getFramebufferView() {
  return new Uint32Array(
    wasm.memory.buffer,
    chip8.framebuffer_ptr(),
    chip8.width() * chip8.height()
  );
}

function drawScreen() {
  const width = chip8.width();
  const height = chip8.height();

  const framebuffer = getFramebufferView();
  const imageData = ctx.createImageData(width, height);

  for (let i = 0; i < framebuffer.length; i++) {
    const pixel = framebuffer[i];
    const offset = i * 4;

    if (pixel !== 0) {
      imageData.data[offset + 0] = 255;
      imageData.data[offset + 1] = 255;
      imageData.data[offset + 2] = 255;
      imageData.data[offset + 3] = 255;
    } else {
      imageData.data[offset + 0] = 0;
      imageData.data[offset + 1] = 0;
      imageData.data[offset + 2] = 0;
      imageData.data[offset + 3] = 255;
    }
  }

  ctx.putImageData(imageData, 0, 0);
}

function renderLoop() {
  if (!running) return;

  drawScreen();
  animationFrameId = requestAnimationFrame(renderLoop);
}

function startEmulator() {
  stopEmulator();

  running = true;

  cpuIntervalId = setInterval(() => {
    chip8.cycle();
  }, delay);

  animationFrameId = requestAnimationFrame(renderLoop);
}

playButton.addEventListener("click", () => {
  if (!rom) {
    alert("Add a ROM file to play.");
    return;
  }

  if (scale === -1) {
    alert("Define a video scale.");
    return;
  }

  if (delay === -1) {
    alert("Define a delay.");
    return;
  }

  const width = chip8.width();
  const height = chip8.height();

  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width * scale}px`;
  canvas.style.height = `${height * scale}px`;

  chip8.load_rom(rom);

  if (!inputRegistered) {
    registerInputHandlers(chip8);
    inputRegistered = true;
  }

  startEmulator();
});