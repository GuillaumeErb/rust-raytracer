run `cargo build` in either terminal-raytracer or sdl2-raytracer.

To compile wasm-raytracer, run in wasm-raytracer, `wasm-pack build`
then `npm install` in wasm-raytracer/www,
to run `npm run start` in wasm-raytracer/www. open http://localhost:8080/ to see the app.
Anytime you make changes and want them reflected on http://localhost:8080/, just re-run the wasm-pack build command within the wasm-game-of-life directory.

To build in order to deploy, in www, run `npm run-script build`
