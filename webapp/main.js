import init, { Emulator } from './pkg/chip8.js'

const canvas = document.getElementById("canvas")
const context = canvas.getContext("2d")
const CONFIG = { COLS: 64, ROWS: 32, PIXEL_SIZE: 9, PIXEL_GAP: 1, CYCLES_PER_FRAME: 10 }
const KEYS = [
    [["1", 1], ["2", 2], ["3", 3], ["C", 12]],
    [["4", 4], ["5", 5], ["6", 6], ["D", 13]],
    [["7", 7], ["8", 8], ["9", 9], ["E", 14]],
    [["A", 10], ["0", 0], ["B", 11], ["F", 15]]
]

function setupUI(emu) {
    const keyboard = document.getElementById("keyboard")
    for (const row of KEYS) {
        for (const label of row) {
            const button = document.createElement("div")
            button.className = "key"
            button.textContent = label[0]
            button.addEventListener("mousedown", () => emu.key_down(label[1]));
            button.addEventListener("mouseup", () => emu.key_up(label[1]));
            button.addEventListener("mouseleave", () => emu.key_up(label[1])); // avoid stuck keys if dragged off
            keyboard.appendChild(button)
        }
    }
}

function draw(emu) {
    const display = emu.display_flat();
    context.fillStyle = "#696969"
    context.fillRect(0, 0, canvas.width, canvas.height)
    context.fillStyle = "#c2ff78"

    for (let r = 0; r < CONFIG.ROWS; r++) {
        for (let c = 0; c < CONFIG.COLS; c++) {
            if (display[r * CONFIG.COLS + c]) {
                context.fillRect(
                    c * (CONFIG.PIXEL_SIZE + CONFIG.PIXEL_GAP),
                    r * (CONFIG.PIXEL_SIZE + CONFIG.PIXEL_GAP),
                    CONFIG.PIXEL_SIZE,
                    CONFIG.PIXEL_SIZE
                )
            }
        }
    }
}

function getPresentationOpcodes(view) {
    const res = []
    for (let i = 0; i < view.length - 1; i += 2) {
        const addr = (0x200 + i).toString(16).toUpperCase()//.padStart(4, "0")
        const opcode = ((view[i] << 8) | view[i + 1]).toString(16).toUpperCase().padStart(4, "0")
        res.push({ addr, opcode })
    }
    return res
}

function renderTable(opcodes) {
    const tbody = document.getElementById("opcode-body")
    tbody.innerHTML = "";

    const frag = document.createDocumentFragment()
    for (const { addr, opcode } of opcodes) {
        const tr = document.createElement("tr")
        tr.innerHTML = `
           <td style="padding:2px 8px;">0x${addr}</td>
           <td style="padding:2px 8px;">${opcode}</td>
        `
        frag.appendChild(tr)
    }
    tbody.appendChild(frag)
}

async function load_rom(emu, file) {
    const romBuffer = await file.arrayBuffer();
    const view = new Uint8Array(romBuffer)
    renderTable(getPresentationOpcodes(view))
    emu.load_rom(view)
}

async function run() {
    await init()
    const emu = new Emulator()
    setupUI(emu)
    load_rom(emu, await fetch("./roms/IBM Logo.ch8"))
    document.getElementById("rom-input").onchange = ({ target }) => load_rom(emu, target.files[0]);

    function frame() {
        for (let i = 0; i < CONFIG.CYCLES_PER_FRAME; i++) {
            emu.step()
        }

        emu.tick_timers()
        draw(emu)
        requestAnimationFrame(frame)
    }

    requestAnimationFrame(frame)
}
run()