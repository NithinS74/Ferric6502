'use strict';

function Labels() {
    var labelIndex = [];
    return {
        find: function(name) {
            for (var i = 0; i < labelIndex.length; i++) {
                if (name === labelIndex[i].split("|")[0]) return true;
            }
            return false;
        },
        getPC: function(name) {
            for (var i = 0; i < labelIndex.length; i++) {
                var parts = labelIndex[i].split("|");
                if (name === parts[0]) return parseInt(parts[1]);
            }
            return -1;
        },
        push: function(name, addr) {
            if (this.find(name)) return false;
            labelIndex.push(name + "|" + addr);
            return true;
        },
        reset: function() {
            labelIndex = [];
        }
    };
}

function Assembler() {
    var Opcodes = [
        ["ADC", 0x69, 0x65, 0x75, null, 0x6d, 0x7d, 0x79, null, 0x61, 0x71, null, null],
        ["AND", 0x29, 0x25, 0x35, null, 0x2d, 0x3d, 0x39, null, 0x21, 0x31, null, null],
        ["ASL", null, 0x06, 0x16, null, 0x0e, 0x1e, null, null, null, null, 0x0a, null],
        ["BIT", null, 0x24, null, null, 0x2c, null, null, null, null, null, null, null],
        ["BPL", null, null, null, null, null, null, null, null, null, null, null, 0x10],
        ["BMI", null, null, null, null, null, null, null, null, null, null, null, 0x30],
        ["BVC", null, null, null, null, null, null, null, null, null, null, null, 0x50],
        ["BVS", null, null, null, null, null, null, null, null, null, null, null, 0x70],
        ["BCC", null, null, null, null, null, null, null, null, null, null, null, 0x90],
        ["BCS", null, null, null, null, null, null, null, null, null, null, null, 0xb0],
        ["BNE", null, null, null, null, null, null, null, null, null, null, null, 0xd0],
        ["BEQ", null, null, null, null, null, null, null, null, null, null, null, 0xf0],
        ["BRK", null, null, null, null, null, null, null, null, null, null, 0x00, null],
        ["CMP", 0xc9, 0xc5, 0xd5, null, 0xcd, 0xdd, 0xd9, null, 0xc1, 0xd1, null, null],
        ["CPX", 0xe0, 0xe4, null, null, 0xec, null, null, null, null, null, null, null],
        ["CPY", 0xc0, 0xc4, null, null, 0xcc, null, null, null, null, null, null, null],
        ["DEC", null, 0xc6, 0xd6, null, 0xce, 0xde, null, null, null, null, null, null],
        ["EOR", 0x49, 0x45, 0x55, null, 0x4d, 0x5d, 0x59, null, 0x41, 0x51, null, null],
        ["CLC", null, null, null, null, null, null, null, null, null, null, 0x18, null],
        ["SEC", null, null, null, null, null, null, null, null, null, null, 0x38, null],
        ["CLI", null, null, null, null, null, null, null, null, null, null, 0x58, null],
        ["SEI", null, null, null, null, null, null, null, null, null, null, 0x78, null],
        ["CLV", null, null, null, null, null, null, null, null, null, null, 0xb8, null],
        ["CLD", null, null, null, null, null, null, null, null, null, null, 0xd8, null],
        ["SED", null, null, null, null, null, null, null, null, null, null, 0xf8, null],
        ["INC", null, 0xe6, 0xf6, null, 0xee, 0xfe, null, null, null, null, null, null],
        ["JMP", null, null, null, null, 0x4c, null, null, 0x6c, null, null, null, null],
        ["JSR", null, null, null, null, 0x20, null, null, null, null, null, null, null],
        ["LDA", 0xa9, 0xa5, 0xb5, null, 0xad, 0xbd, 0xb9, null, 0xa1, 0xb1, null, null],
        ["LDX", 0xa2, 0xa6, null, 0xb6, 0xae, null, 0xbe, null, null, null, null, null],
        ["LDY", 0xa0, 0xa4, 0xb4, null, 0xac, 0xbc, null, null, null, null, null, null],
        ["LSR", null, 0x46, 0x56, null, 0x4e, 0x5e, null, null, null, null, 0x4a, null],
        ["NOP", null, null, null, null, null, null, null, null, null, null, 0xea, null],
        ["ORA", 0x09, 0x05, 0x15, null, 0x0d, 0x1d, 0x19, null, 0x01, 0x11, null, null],
        ["TAX", null, null, null, null, null, null, null, null, null, null, 0xaa, null],
        ["TXA", null, null, null, null, null, null, null, null, null, null, 0x8a, null],
        ["DEX", null, null, null, null, null, null, null, null, null, null, 0xca, null],
        ["INX", null, null, null, null, null, null, null, null, null, null, 0xe8, null],
        ["TAY", null, null, null, null, null, null, null, null, null, null, 0xa8, null],
        ["TYA", null, null, null, null, null, null, null, null, null, null, 0x98, null],
        ["DEY", null, null, null, null, null, null, null, null, null, null, 0x88, null],
        ["INY", null, null, null, null, null, null, null, null, null, null, 0xc8, null],
        ["ROR", null, 0x66, 0x76, null, 0x6e, 0x7e, null, null, null, null, 0x6a, null],
        ["ROL", null, 0x26, 0x36, null, 0x2e, 0x3e, null, null, null, null, 0x2a, null],
        ["RTI", null, null, null, null, null, null, null, null, null, null, 0x40, null],
        ["RTS", null, null, null, null, null, null, null, null, null, null, 0x60, null],
        ["SBC", 0xe9, 0xe5, 0xf5, null, 0xed, 0xfd, 0xf9, null, 0xe1, 0xf1, null, null],
        ["STA", null, 0x85, 0x95, null, 0x8d, 0x9d, 0x99, null, 0x81, 0x91, null, null],
        ["TXS", null, null, null, null, null, null, null, null, null, null, 0x9a, null],
        ["TSX", null, null, null, null, null, null, null, null, null, null, 0xba, null],
        ["PHA", null, null, null, null, null, null, null, null, null, null, 0x48, null],
        ["PLA", null, null, null, null, null, null, null, null, null, null, 0x68, null],
        ["PHP", null, null, null, null, null, null, null, null, null, null, 0x08, null],
        ["PLP", null, null, null, null, null, null, null, null, null, null, 0x28, null],
        ["STX", null, 0x86, null, 0x96, 0x8e, null, null, null, null, null, null, null],
        ["STY", null, 0x84, 0x94, null, 0x8c, null, null, null, null, null, null, null]
    ];

    var labels = Labels(); // Brought out so all functions can see it

    function tryParseByte(param, symbols) {
        if (param.match(/^\w+$/) && symbols.lookup(param)) param = symbols.lookup(param);
        var m = param.match(/^([0-9]{1,3})$/);
        if (m) return parseInt(m[1], 10);
        m = param.match(/^\$([0-9a-f]{1,2})$/i);
        if (m) return parseInt(m[1], 16);
        m = param.match(/^%([0-1]{1,8})$/);
        if (m) return parseInt(m[1], 2);
        return -1;
    }

    function tryParseWord(param, symbols) {
        if (param.match(/^\w+$/) && symbols.lookup(param)) param = symbols.lookup(param);
        var m = param.match(/^\$([0-9a-f]{3,4})$/i);
        if (m) return parseInt(m[1], 16);
        m = param.match(/^([0-9]{1,5})$/i);
        if (m) return parseInt(m[1], 10);
        return -1;
    }

    function sanitize(line) {
        return line.replace(/^(.*?);.*/, "$1").replace(/^\s+/, "").replace(/\s+$/, "");
    }

    function preprocess(lines) {
        var table = {};
        for (var i = 0; i < lines.length; i++) {
            lines[i] = sanitize(lines[i]);
            var m = lines[i].match(/^define\s+(\w+)\s+(\S+)/);
            if (m) {
                table["__" + m[1]] = sanitize(m[2]);
                lines[i] = "";
            }
        }
        return { lookup: function(k) { return table["__" + k]; } };
    }

    function assembleLine(input, currentPC, symbols, output) {
        var command, param;
        if (input.match(/^\w+:/)) {
            input = input.replace(/^\w+:\s*/, "");
        }
        if (input === "") return true;
        
        var m = input.match(/^(\w+)\s+(.*)$/);
        if (m) {
            command = m[1].toUpperCase();
            param = m[2].replace(/\s/g, "");
        } else {
            command = input.toUpperCase();
            param = "";
        }

        for (var o = 0; o < Opcodes.length; o++) {
            if (Opcodes[o][0] === command) {
                var op = Opcodes[o];
                
                // Single (Implied/Accumulator) -- Fixed missing output array
                if (op[11] !== null && (param === "" || param === "A")) { output.push(op[11]); return true; }
                
                // Immediate
                if (op[1] !== null && param.match(/^#/)) {
                    var val = tryParseByte(param.substring(1), symbols);
                    if (val !== -1) { output.push(op[1], val); return true; }
                }
                
                // Zero Page X/Y
                if (op[3] !== null && param.match(/,X$/i)) {
                    var val = tryParseByte(param.replace(/,X$/i, ""), symbols);
                    if (val !== -1) { output.push(op[3], val); return true; }
                }
                if (op[4] !== null && param.match(/,Y$/i)) {
                    var val = tryParseByte(param.replace(/,Y$/i, ""), symbols);
                    if (val !== -1) { output.push(op[4], val); return true; }
                }
                
                // Zero Page
                if (op[2] !== null) {
                    var val = tryParseByte(param, symbols);
                    if (val !== -1) { output.push(op[2], val); return true; }
                }

                // Absolute X/Y
                if (op[6] !== null && param.match(/,X$/i)) {
                    var base = param.replace(/,X$/i, "");
                    var val = tryParseWord(base, symbols);
                    if (val !== -1) { output.push(op[6], val & 0xff, (val >> 8) & 0xff); return true; }
                    if (labels.getPC(base) !== -1) { var a = labels.getPC(base); output.push(op[6], a & 0xff, (a >> 8) & 0xff); return true; }
                    output.push(op[6], 0x00, 0x00); return true; // Pass 1 fallback
                }
                if (op[7] !== null && param.match(/,Y$/i)) {
                    var base = param.replace(/,Y$/i, "");
                    var val = tryParseWord(base, symbols);
                    if (val !== -1) { output.push(op[7], val & 0xff, (val >> 8) & 0xff); return true; }
                    if (labels.getPC(base) !== -1) { var a = labels.getPC(base); output.push(op[7], a & 0xff, (a >> 8) & 0xff); return true; }
                    output.push(op[7], 0x00, 0x00); return true; // Pass 1 fallback
                }

                // Absolute
                if (op[5] !== null) {
                    var val = tryParseWord(param, symbols);
                    if (val !== -1) { output.push(op[5], val & 0xff, (val >> 8) & 0xff); return true; }
                    if (labels.getPC(param) !== -1) { var a = labels.getPC(param); output.push(op[5], a & 0xff, (a >> 8) & 0xff); return true; }
                    if (param.match(/^[A-Za-z_]/)) { output.push(op[5], 0x00, 0x00); return true; } // Pass 1 fallback
                }
                
                // Indirect X/Y
                if (op[9] !== null && param.match(/^\(.*\),X$/i)) {
                    var val = tryParseByte(param.replace(/^\((.*)\),X$/i, "$1"), symbols);
                    if (val !== -1) { output.push(op[9], val); return true; }
                }
                if (op[10] !== null && param.match(/^\(.*?\),Y$/i)) {
                    var val = tryParseByte(param.replace(/^\((.*?)\),Y$/i, "$1"), symbols);
                    if (val !== -1) { output.push(op[10], val); return true; }
                }
                
                // Indirect
                if (op[8] !== null && param.match(/^\(.*?\)$/)) {
                    var val = tryParseWord(param.replace(/^\((.*?)\)$/, "$1"), symbols);
                    if (val !== -1) { output.push(op[8], val & 0xff, (val >> 8) & 0xff); return true; }
                }

                // Branch
                if (op[12] !== null) {
                    var addr = labels.getPC(param);
                    if (addr === -1) {
                        output.push(op[12], 0x00);
                        return true;
                    }
                    var distance = addr - (currentPC + 2);
                    if (distance < -128 || distance > 127) return false;
                    output.push(op[12], distance & 0xff);
                    return true;
                }
            }
        }
        return false;
    }

    return {
        assemble: function(codeString) {
            var lines = codeString.split("\n");
            labels.reset();
            var symbols = preprocess(lines);
            var outputBytes = [];

            // Pass 1: Index labels
            var pc = 0x8000;
            for (var i = 0; i < lines.length; i++) {
                if (lines[i].match(/^\w+:/)) {
                    labels.push(lines[i].replace(/(^\w+):.*$/, "$1"), pc);
                }
                if (lines[i] !== "" && !lines[i].match(/^\w+:$/)) {
                    var prevLen = outputBytes.length;
                    assembleLine(lines[i], pc, symbols, outputBytes);
                    pc += (outputBytes.length - prevLen); // Fixed bug where PC multiplied exponentially
                }
            }

            // Pass 2: Assemble
            outputBytes = [];
            pc = 0x8000;
            for (var i = 0; i < lines.length; i++) {
                if (lines[i] === "" || lines[i].match(/^\w+:$/)) continue;
                var prevLen = outputBytes.length;
                if (!assembleLine(lines[i], pc, symbols, outputBytes)) {
                    return { error: true, message: "Syntax error on line " + (i + 1) + ": " + lines[i] };
                }
                pc += (outputBytes.length - prevLen);
            }

            return { error: false, bytes: outputBytes };
        }
    };
}

window.assemble6502 = function(codeString) {
    var assembler = Assembler();
    var result = assembler.assemble(codeString);
    if (result.error) {
        console.error(result.message);
        return [];
    }
    return result.bytes;
};
