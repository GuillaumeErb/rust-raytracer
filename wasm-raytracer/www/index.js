import { memory } from "wasm-raytracer/wasm_raytracer_bg";
import { Screen } from "wasm-raytracer";

const PIXEL_SIZE = 6; // px

const screen = Screen.new();
const width = screen.width();
const height = screen.height();

const canvas = document.getElementById("raytracer-screen-canvas");
canvas.height = PIXEL_SIZE * height;
canvas.width = PIXEL_SIZE * width;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
    return (row * width + column) * 3;
};

const render = () => {
    const cellsPtr = screen.pixels();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height * 3);

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            const r = cells[idx];
            const g = cells[idx + 1];
            const b = cells[idx + 2];

            ctx.fillStyle = 'rgb(' + r + ',' + g + ',' + b + ')';
            ctx.fillRect(col * PIXEL_SIZE,
                row * PIXEL_SIZE,
                PIXEL_SIZE,
                PIXEL_SIZE);
        }
    }
};

screen.render();
render();
