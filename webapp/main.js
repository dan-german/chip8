import init, { Emulator } from './pkg/chip8.js'


const pixels = Array.from({ length: 32 }, (_, i) => Array.from({ length: 64 }))
console.log(pixels)
const canvas = document.getElementById("canvas")
const context = canvas.getContext("2d")
const cols = 64, rows = 32, size = 9, gap = 1
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

    context.fillStyle = "#0c0c0c"
    context.fillRect(0, 0, canvas.width, canvas.height)
    context.fillStyle = "#c2ff78"

    for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
            if (display[r * cols + c]) {
                context.fillRect(c * (size + gap), r * (size + gap), size, size)
            }
        }
    }
}

let running = true;
let emu
async function run() {
    await init()
    const emu = new Emulator()
    setupUI(emu)
    console.log(Object.getOwnPropertyNames(Object.getPrototypeOf(emu)));

    const romInput = document.getElementById("rom-input")
    romInput.addEventListener("change", async (e) => {
        const file = e.target.files[0]
        const buffer = await file.arrayBuffer();
        emu.load_rom(new Uint8Array(buffer))
        running = true;
    })

    function frame() {
        if (running) {
            for (let i = 0; i < 10; i++) {
                emu.step()
            }

            // console.log(emu.display_flat().filter(p => p === 1).length); // count of lit pixels
            emu.tick_timers()
            draw(emu)
        }
        requestAnimationFrame(frame)
    }

    requestAnimationFrame(frame)
    // document.querySelectorAll(".key").forEach((button) => {
    //     const label = button.textContent;
    //     button.addEventListener("mousedown", () => emu.key_down(KEYS[label]));
    //     button.addEventListener("mouseup", () => emu.key_up(KEY_VALUES[label]));
    //     button.addEventListener("mouseleave", () => emu.key_up(KEY_VALUES[label])); // avoid stuck keys if dragged off
    // });
}
run()