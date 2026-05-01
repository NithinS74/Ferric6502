import init, { CPU } from './pkg/nesoxide.js';

let cpu;

async function start() {
    console.log("Loading Wasm...");
    await init();
    cpu = new CPU();
    console.log("Wasm loaded. CPU initialized.");

    document.getElementById('btn-load').addEventListener('click', loadProgram);
    document.getElementById('btn-step').addEventListener('click', stepCpu);
}

function loadProgram() {
    const code = document.getElementById('code').value;
    console.log("1. Raw Assembly:", code);

    const compiledBytes = window.assemble6502(code);
    console.log("2. Compiled Bytes from assembler.js:", compiledBytes);

    if (!compiledBytes || compiledBytes.length === 0) {
        alert("Assembler failed to output bytes. Check console.");
        return;
    }

    // --- NEW: Display the compiled hex bytes cleanly ---
    const hexString = compiledBytes
        .map(b => b.toString(16).padStart(2, '0').toUpperCase())
        .join(' ');
    document.getElementById('compiled-output').value = hexString;

    const wasmArray = new Uint8Array(compiledBytes);
    cpu.load_program_from_js(wasmArray);
    cpu.reset();
    console.log("3. Program loaded into Rust CPU and reset.");

    document.getElementById('btn-step').disabled = false;
    updateDisplay();
}

function stepCpu() {
    console.log("Stepping CPU...");
    const hitBrk = cpu.step();
    updateDisplay();

    if (hitBrk) {
        console.log("BRK HIT!");
        document.getElementById('btn-step').disabled = true;
        document.getElementById('output').innerText += "\n\n*** HALTED ON BRK ***";
    }
}

function updateDisplay() {
    const pc = cpu.get_program_counter().toString(16).padStart(4, '0').toUpperCase();
    const a = cpu.get_register_a().toString(16).padStart(2, '0').toUpperCase();
    const x = cpu.get_register_x().toString(16).padStart(2, '0').toUpperCase();
    const y = cpu.get_register_y().toString(16).padStart(2, '0').toUpperCase();
    const sp = cpu.get_stack_pointer().toString(16).padStart(2, '0').toUpperCase();
    const status = cpu.get_status_register().toString(2).padStart(8, '0');

    const display = `PC: $${pc}\nA:  $${a}\nX:  $${x}\nY:  $${y}\nSP: $${sp}\nNV-BDIZC: %${status}`;
    document.getElementById('output').innerText = display;
}

start();
