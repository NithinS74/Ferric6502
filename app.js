import init, { CPU } from './pkg/nesoxide.js';

let cpu;
let cycleCount = 0;
let traceLog = [];      // Array of { pc, hex, instr, a, flags }
let prevRegs = {};      // For change-highlight animation
const MAX_TRACE = 64;

// ── Opcode → mnemonic lookup (for trace log display) ────────────────────────
const MNEMONIC = {
    0x00: 'BRK', 0x01: 'ORA', 0x05: 'ORA', 0x06: 'ASL', 0x08: 'PHP', 0x09: 'ORA', 0x0A: 'ASL',
    0x0D: 'ORA', 0x0E: 'ASL', 0x10: 'BPL', 0x11: 'ORA', 0x15: 'ORA', 0x16: 'ASL', 0x18: 'CLC',
    0x19: 'ORA', 0x1D: 'ORA', 0x1E: 'ASL', 0x20: 'JSR', 0x21: 'AND', 0x24: 'BIT', 0x25: 'AND',
    0x26: 'ROL', 0x28: 'PLP', 0x29: 'AND', 0x2A: 'ROL', 0x2C: 'BIT', 0x2D: 'AND', 0x2E: 'ROL',
    0x30: 'BMI', 0x31: 'AND', 0x35: 'AND', 0x36: 'ROL', 0x38: 'SEC', 0x39: 'AND', 0x3D: 'AND',
    0x3E: 'ROL', 0x40: 'RTI', 0x41: 'EOR', 0x45: 'EOR', 0x46: 'LSR', 0x48: 'PHA', 0x49: 'EOR',
    0x4A: 'LSR', 0x4C: 'JMP', 0x4D: 'EOR', 0x4E: 'LSR', 0x50: 'BVC', 0x51: 'EOR', 0x55: 'EOR',
    0x56: 'LSR', 0x58: 'CLI', 0x59: 'EOR', 0x5D: 'EOR', 0x5E: 'LSR', 0x60: 'RTS', 0x61: 'ADC',
    0x65: 'ADC', 0x66: 'ROR', 0x68: 'PLA', 0x69: 'ADC', 0x6A: 'ROR', 0x6C: 'JMP', 0x6D: 'ADC',
    0x6E: 'ROR', 0x70: 'BVS', 0x71: 'ADC', 0x75: 'ADC', 0x76: 'ROR', 0x78: 'SEI', 0x79: 'ADC',
    0x7D: 'ADC', 0x7E: 'ROR', 0x81: 'STA', 0x84: 'STY', 0x85: 'STA', 0x86: 'STX', 0x88: 'DEY',
    0x8A: 'TXA', 0x8C: 'STY', 0x8D: 'STA', 0x8E: 'STX', 0x90: 'BCC', 0x91: 'STA', 0x94: 'STY',
    0x95: 'STA', 0x96: 'STX', 0x98: 'TYA', 0x99: 'STA', 0x9A: 'TXS', 0x9D: 'STA', 0xA0: 'LDY',
    0xA1: 'LDA', 0xA2: 'LDX', 0xA4: 'LDY', 0xA5: 'LDA', 0xA6: 'LDX', 0xA8: 'TAY', 0xA9: 'LDA',
    0xAA: 'TAX', 0xAC: 'LDY', 0xAD: 'LDA', 0xAE: 'LDX', 0xB0: 'BCS', 0xB1: 'LDA', 0xB4: 'LDY',
    0xB5: 'LDA', 0xB6: 'LDX', 0xB8: 'CLV', 0xB9: 'LDA', 0xBA: 'TSX', 0xBC: 'LDY', 0xBD: 'LDA',
    0xBE: 'LDX', 0xC0: 'CPY', 0xC1: 'CMP', 0xC4: 'CPY', 0xC5: 'CMP', 0xC6: 'DEC', 0xC8: 'INY',
    0xC9: 'CMP', 0xCA: 'DEX', 0xCC: 'CPY', 0xCD: 'CMP', 0xCE: 'DEC', 0xD0: 'BNE', 0xD1: 'CMP',
    0xD5: 'CMP', 0xD6: 'DEC', 0xD8: 'CLD', 0xD9: 'CMP', 0xDD: 'CMP', 0xDE: 'DEC', 0xE0: 'CPX',
    0xE1: 'SBC', 0xE4: 'CPX', 0xE5: 'SBC', 0xE6: 'INC', 0xE8: 'INX', 0xE9: 'SBC', 0xEA: 'NOP',
    0xEC: 'CPX', 0xED: 'SBC', 0xEE: 'INC', 0xF0: 'BEQ', 0xF1: 'SBC', 0xF5: 'SBC', 0xF6: 'INC',
    0xF8: 'SED', 0xF9: 'SBC', 0xFD: 'SBC', 0xFE: 'INC',
};
const STORE_OPS = new Set(['STA', 'STX', 'STY', 'PHA', 'PHP', 'TXS']);

const SUN_SVG = `<path d="M8 1v2M8 13v2M1 8h2M13 8h2M3.05 3.05l1.42 1.42M11.53 11.53l1.42 1.42M3.05 12.95l1.42-1.42M11.53 4.47l1.42-1.42"/><circle cx="8" cy="8" r="3"/>`;
const MOON_SVG = `<path d="M14 10.5a5.5 5.5 0 1 1-7.5-7.5 7 7 0 1 0 7.5 7.5z"/>`;

const OPCODE_LENGTHS = {
    0x00: 1, 0x01: 2, 0x05: 2, 0x06: 2, 0x08: 1, 0x09: 2, 0x0A: 1, 0x0D: 3, 0x0E: 3, 0x10: 2, 0x11: 2, 0x15: 2, 0x16: 2, 0x18: 1,
    0x19: 3, 0x1D: 3, 0x1E: 3, 0x20: 3, 0x21: 2, 0x24: 2, 0x25: 2, 0x26: 2, 0x28: 1, 0x29: 2, 0x2A: 1, 0x2C: 3, 0x2D: 3, 0x2E: 3,
    0x30: 2, 0x31: 2, 0x35: 2, 0x36: 2, 0x38: 1, 0x39: 3, 0x3D: 3, 0x3E: 3, 0x40: 1, 0x41: 2, 0x45: 2, 0x46: 2, 0x48: 1, 0x49: 2,
    0x4A: 1, 0x4C: 3, 0x4D: 3, 0x4E: 3, 0x50: 2, 0x51: 2, 0x55: 2, 0x56: 2, 0x58: 1, 0x59: 3, 0x5D: 3, 0x5E: 3, 0x60: 1, 0x61: 2,
    0x65: 2, 0x66: 2, 0x68: 1, 0x69: 2, 0x6A: 1, 0x6C: 3, 0x6D: 3, 0x6E: 3, 0x70: 2, 0x71: 2, 0x75: 2, 0x76: 2, 0x78: 1, 0x79: 3,
    0x7D: 3, 0x7E: 3, 0x81: 2, 0x84: 2, 0x85: 2, 0x86: 2, 0x88: 1, 0x8A: 1, 0x8C: 3, 0x8D: 3, 0x8E: 3, 0x90: 2, 0x91: 2, 0x94: 2,
    0x95: 2, 0x96: 2, 0x98: 1, 0x99: 3, 0x9A: 1, 0x9D: 3, 0xA0: 2, 0xA1: 2, 0xA2: 2, 0xA4: 2, 0xA5: 2, 0xA6: 2, 0xA8: 1, 0xA9: 2,
    0xAA: 1, 0xAC: 3, 0xAD: 3, 0xAE: 3, 0xB0: 2, 0xB1: 2, 0xB4: 2, 0xB5: 2, 0xB6: 2, 0xB8: 1, 0xB9: 3, 0xBA: 1, 0xBC: 3, 0xBD: 3,
    0xBE: 3, 0xC0: 2, 0xC1: 2, 0xC4: 2, 0xC5: 2, 0xC6: 2, 0xC8: 1, 0xC9: 2, 0xCA: 1, 0xCC: 3, 0xCD: 3, 0xCE: 3, 0xD0: 2, 0xD1: 2,
    0xD5: 2, 0xD6: 2, 0xD8: 1, 0xD9: 3, 0xDD: 3, 0xDE: 3, 0xE0: 2, 0xE1: 2, 0xE4: 2, 0xE5: 2, 0xE6: 2, 0xE8: 1, 0xE9: 2, 0xEA: 1,
    0xEC: 3, 0xED: 3, 0xEE: 3, 0xF0: 2, 0xF1: 2, 0xF5: 2, 0xF6: 2, 0xF8: 1, 0xF9: 3, 0xFD: 3, 0xFE: 3,
};

async function start() {
    await init();
    cpu = new CPU();

    document.getElementById('btn-load').addEventListener('click', loadProgram);
    document.getElementById('btn-step').addEventListener('click', stepCpu);
    document.getElementById('btn-reset').addEventListener('click', resetCpu);
    document.getElementById('btn-clear-trace').addEventListener('click', clearTrace);
    document.getElementById('btn-theme').addEventListener('click', toggleTheme);

    document.addEventListener('keydown', (e) => {
        if (e.key === 'F8') {
            e.preventDefault();
            const btn = document.getElementById('btn-step');
            if (!btn.disabled) stepCpu();
        }
    });
}

function loadProgram() {
    const code = document.getElementById('code').value;
    const result = window.assemble6502(code);

    if (result.error) {
        alert("Assembler failed: " + result.message);
        return;
    }

    const compiledBytes = result.bytes;

    if (!compiledBytes || compiledBytes.length === 0) {
        alert("Assembler failed — check your source code.");
        return;
    }

    const hexLines = [];
    for (let i = 0; i < compiledBytes.length; i += 16) {
        const chunk = Array.from(compiledBytes.slice(i, i + 16));
        const addr = (0x8000 + i).toString(16).toUpperCase().padStart(4, '0');
        const hex = chunk.map(b => b.toString(16).padStart(2, '0').toUpperCase()).join(' ');
        hexLines.push(`$${addr}: ${hex}`);
    }
    const hexString = hexLines.join('\n');

    document.getElementById('compiled-output').value = hexString;

    const wasmArray = new Uint8Array(compiledBytes);
    cpu.load_program_from_js(wasmArray);
    cpu.reset();

    cycleCount = 0;
    traceLog = [];
    prevRegs = {
        pc: cpu.get_program_counter(),
        sp: cpu.get_stack_pointer(),
        a: cpu.get_register_a(),
        x: cpu.get_register_x(),
        y: cpu.get_register_y()
    };

    document.getElementById('btn-step').disabled = false;
    document.getElementById('halt-banner').classList.remove('visible');
    setStatus('ready');
    updateDisplay();
}

function resetCpu() {
    if (!cpu) return;
    cpu.reset();
    cycleCount = 0;
    traceLog = [];
    prevRegs = {
        pc: cpu.get_program_counter(),
        sp: cpu.get_stack_pointer(),
        a: cpu.get_register_a(),
        x: cpu.get_register_x(),
        y: cpu.get_register_y()
    };
    document.getElementById('btn-step').disabled = false;
    document.getElementById('halt-banner').classList.remove('visible');
    setStatus('ready');
    updateDisplay();
    renderTrace();
}

function stepCpu() {
    if (!cpu) return;

    // Capture state BEFORE step for trace
    const pcBefore = cpu.get_program_counter();
    const memSlice = cpu.get_memory_slice(pcBefore, 3);
    const opcode = memSlice[0];
    const mnemonic = MNEMONIC[opcode] || `$${opcode.toString(16).toUpperCase().padStart(2, '0')}`;
    const length = OPCODE_LENGTHS[opcode] || 3;

    // Build hex bytes string (1–3 bytes depending on opcode)
    const hexBytes = Array.from(memSlice.slice(0, length))
        .map(b => b.toString(16).toUpperCase().padStart(2, '0'))
        .join(' ');

    setStatus('running');
    const hitBrk = cpu.step();
    cycleCount++;

    // Build flags string
    const status = cpu.get_status_register();
    const flagNames = ['C', 'Z', 'I', 'D', 'B', '-', 'V', 'N'];
    const flagStr = flagNames.map((f, i) => ((status >> i) & 1) ? f : '-').reverse().join('');

    // Add to trace
    traceLog.push({
        pc: `$${pcBefore.toString(16).toUpperCase().padStart(4, '0')}`,
        hex: hexBytes,
        instr: mnemonic,
        a: `$${cpu.get_register_a().toString(16).toUpperCase().padStart(2, '0')}`,
        flags: flagStr,
        isStore: STORE_OPS.has(mnemonic),
    });
    if (traceLog.length > MAX_TRACE) traceLog.shift();

    updateDisplay();
    renderTrace();

    if (hitBrk) {
        document.getElementById('btn-step').disabled = true;
        document.getElementById('halt-banner').classList.add('visible');
        setStatus('halted');
    }
}

function clearTrace() {
    traceLog = [];
    renderTrace();
}

function setStatus(state) {
    const el = document.getElementById('cpu-status');
    el.className = 'cpu-status';
    if (state === 'running') { el.textContent = 'Running'; el.classList.add('running'); }
    else if (state === 'halted') { el.textContent = 'Halted'; el.classList.add('halted'); }
    else if (state === 'ready') { el.textContent = 'Ready'; }
    else { el.textContent = 'Idle'; }
}

function updateDisplay() {
    if (!cpu) return;
    const pc = cpu.get_program_counter();
    const a = cpu.get_register_a();
    const x = cpu.get_register_x();
    const y = cpu.get_register_y();
    const sp = cpu.get_stack_pointer();

    const fmt4 = v => '$' + v.toString(16).toUpperCase().padStart(4, '0');
    const fmt2 = v => '$' + v.toString(16).toUpperCase().padStart(2, '0');

    setReg('pc', fmt4(pc), prevRegs.pc !== undefined && prevRegs.pc !== pc);
    setReg('sp', fmt2(sp), prevRegs.sp !== undefined && prevRegs.sp !== sp);
    setReg('a', fmt2(a), prevRegs.a !== undefined && prevRegs.a !== a);
    setReg('x', fmt2(x), prevRegs.x !== undefined && prevRegs.x !== x);
    setReg('y', fmt2(y), prevRegs.y !== undefined && prevRegs.y !== y);

    prevRegs = { pc, a, x, y, sp };

    document.getElementById('cycle-counter').textContent = `Cycle: ${cycleCount.toLocaleString()}`;

    // Flags
    const status = cpu.get_status_register();
    const flags = ['C', 'Z', 'I', 'D', 'B', 'U', 'V', 'N'];
    for (let i = 0; i < 8; i++) {
        const bit = (status >> i) & 1;
        const dot = document.getElementById(`flag-${flags[i]}`);
        if (dot) {
            dot.textContent = bit;
            dot.classList.toggle('active', !!bit);
        }
    }

    // Stack
    renderStack(sp);
}

function setReg(name, val, changed) {
    const el = document.getElementById(`reg-${name}`);
    const card = document.getElementById(`card-${name}`);
    if (!el) return;
    el.textContent = val;
    if (changed) {
        card.classList.remove('changed');
        void card.offsetWidth;
        card.classList.add('changed');
        setTimeout(() => card.classList.remove('changed'), 600);
    }
}

function renderStack(sp) {
    if (!cpu) return;
    const tbody = document.getElementById('stack-body');
    const rows = [];
    // Show 8 rows around SP
    const spAddr = 0x0100 + sp;
    const topAddr = Math.min(0x01FF, spAddr + 2);
    const botAddr = Math.max(0x0100, topAddr - 7);

    const slice = cpu.get_memory_slice(botAddr, topAddr - botAddr + 1);

    for (let i = topAddr; i >= botAddr; i--) {
        const val = slice[i - botAddr];
        const isSP = i === spAddr;
        const addrHex = `$${i.toString(16).toUpperCase().padStart(4, '0')}`;
        const valHex = val.toString(16).toUpperCase().padStart(2, '0');
        rows.push(`<tr class="${isSP ? 'stack-row-sp' : ''}">
            <td class="addr">${addrHex}${isSP ? '<span class="sp-badge">SP</span>' : ''}</td>
            <td class="val">${valHex}</td>
        </tr>`);
    }
    tbody.innerHTML = rows.join('');
}

function renderTrace() {
    const tbody = document.getElementById('trace-body');
    if (traceLog.length === 0) {
        tbody.innerHTML = '<tr><td colspan="5" class="empty-trace" style="border:none">No instructions executed yet</td></tr>';
        return;
    }
    const rows = traceLog.map((entry, idx) => {
        const isLast = idx === traceLog.length - 1;
        const instrClass = entry.isStore ? 'trace-store' : 'trace-mnemonic';
        return `<tr class="${isLast ? 'current-row' : ''}">
            <td>${entry.pc}</td>
            <td>${entry.hex}</td>
            <td class="${instrClass}">${entry.instr}</td>
            <td>${entry.a}</td>
            <td>${entry.flags}</td>
        </tr>`;
    }).reverse().join(''); // Most recent at top
    tbody.innerHTML = rows;
}

function toggleTheme() {
    const html = document.documentElement;
    html.dataset.theme = html.dataset.theme === 'dark' ? 'light' : 'dark';
    const isNowLight = html.dataset.theme === 'light';
    document.getElementById('theme-icon').innerHTML = isNowLight ? MOON_SVG : SUN_SVG;
}

start();
