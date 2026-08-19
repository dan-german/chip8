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

async function run() {
    await init()
    const emu = new Emulator()
    setupUI(emu)

    const romInput = document.getElementById("rom-input")
    romInput.addEventListener("change", async (e) => {
        const file = e.target.files[0]
        const buffer = await file.arrayBuffer();
        emu.load_rom(new Uint8Array(buffer))
    })

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