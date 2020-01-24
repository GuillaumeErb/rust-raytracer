import { memory } from "wasm-raytracer/wasm_raytracer_bg";
import { Screen } from "wasm-raytracer";

const PIXEL_SIZE = 1; // px

const textarea = document.getElementById("raytracer-scene-textarea");
const renderButton = document.getElementById("raytracer-render-button");
const canvas = document.getElementById("raytracer-screen-canvas");
let screen = null;
let width = 0;
let height = 0;
let animationId = null;
var renderingStep = 0;


const onClick = () => {
    cancelAnimationFrame(animationId);

    screen = Screen.new(textarea.value);
    width = screen.width();
    height = screen.height();
    canvas.height = PIXEL_SIZE * height;
    canvas.width = PIXEL_SIZE * width;

    renderingStep = 0;
    animationId = requestAnimationFrame(repeatOften);
}

renderButton.addEventListener("click", onClick);

onClick();

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
    return (row * width + column) * 3;
};

const paint = () => {
    const cellsPtr = screen.pixels();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height * 3);

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            paintPixel(cells, row, col, PIXEL_SIZE);
        }
    }
};

const PIXEL_SIZE_STEPS = [13, 11, 9, 7, 5, 4, 3, 2, 1];

const paintStep = (step) => {
    const cellsPtr = screen.pixels();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height * 3);

    for (let row = 0; row < height / PIXEL_SIZE_STEPS[step]; row++) {
        for (let col = 0; col < width / PIXEL_SIZE_STEPS[step]; col++) {
            paintPixel(cells, row, col, PIXEL_SIZE_STEPS[step]);
        }
    }
};

const paintPixel = (cells, row, col, pixelSize) => {
    const idx = getIndex(row * pixelSize, col * pixelSize);
    const r = cells[idx];
    const g = cells[idx + 1];
    const b = cells[idx + 2];

    ctx.fillStyle = 'rgb(' + r + ',' + g + ',' + b + ')';
    ctx.fillRect(col * pixelSize,
        row * pixelSize,
        pixelSize,
        pixelSize);
}

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / PIXEL_SIZE), height);
    const col = Math.min(Math.floor(canvasLeft / PIXEL_SIZE), width);

    screen.click(row, col);
});

canvas.addEventListener('keydown', function (event) {
    cancelAnimationFrame(animationId);
    renderingStep = 0;
    screen.keydown(event.keyCode);
    animationId = requestAnimationFrame(repeatOften);
});

function repeatOften() {
    const startTime = new Date();
    screen.renderStep(renderingStep);
    paintStep(renderingStep);
    if (renderingStep++ > PIXEL_SIZE_STEPS.length - 1) {
        return;
    }
    const timeDiff = new Date() - startTime;
    console.log("Step " + renderingStep + " " + timeDiff + " ms");
    animationId = requestAnimationFrame(repeatOften);
}
animationId = requestAnimationFrame(repeatOften);
//screen.render();

/*
const startTime = new Date();
screen.render();
paint();
const timeDiff = new Date() - startTime;
console.log("Step " + renderingStep + " " + timeDiff + " ms");
*/