- Figure out animation
- [ ] Fix issue with init_vec; and using real only vs complex + real
[x] Remote dev environment using Remote vscode in Vmachine
- Figure out testing
- Figure out directory structure
[x] Basic WASM w/Game of life
[x] Interactivity with buttons
[x] Added unit test
- Change logic
- Understand debugging
[x] Start simple serve to be able to pass simple data
- Setup some dummy data in a simple data file, so that webpage can obtain the info
- - figure out basic sqlAlchemy using sqlite
- [x] Got basic query working
- [D] clean up table creation, insertion, and querying code
- [ ] Figure out CORS issue to get communication working with page
(e.g. top score stats) [Seems like a resource issue]
- [x] Figure out how to make table so that the same table doesn't get constantly generated 
- [ ] Figure out "Debug mode" for JS, Python
- [ ] Clean up ScoreHandler 
- [ ] Add functionality to add new user name and score
- [ ] Convert float to greyscale in index.html
- [ ] 3D wireframe in WASM/JS
- [ ] Water reflection on sea floor
- [ ] How to setup initial values? -- want droplet
- [ ] Implement time step and fft of sol
- [D] Work on a simple 2D/3D fft https://docs.rs/fft2d/latest/fft2d/slice/fn.fft_2d.html
- - [x] Figure out crates/modules for importing constants
- - [x] Figure out how to use vector slices with FFT
- - - [x] Get tranpose of 2d matrix working
- - - [x] Figure out how to templetize transpose
- - - [x] Write unit test for 2d FFT forward
- - - [ ] Clean up 2D matrix access so that we can use get_grid(x,y) instead of the other (maybe macro or inline?)
- - - - [x] Figure out indexing so vec_2d[a,b] works; vec_2d[a,b] = 1; and r = vec_2d[a,b]
- - - - [x] Rewrite so that transpose uses the new indexing
- - - - [x] Figure out copy trait
- - - - [x] Override Add, 
- - - - [x] scalar multiply, entry wise multiply
- - - - -[x] Figure out issue with using new
- - - - -[x] Figure out how to fix copy trait issue with buffer
- - [ ] Figure out how to use fast transpose with FFT
- - [ ] Figure out how to do Parallel FFT
- [ ] Create a grid object
- [ ] Want a better way to handle parameters (param files)
- [ ] Projections/Simulations on a sphere (?)
- [ ] Project text onto book page
- [Time profiling](https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html)

Day 4: [Work on Testing game of life](https://rustwasm.github.io/docs/book/game-of-life/testing.html#testing-conways-game-of-life)

Day 3: Go through tutorial; figure out directory structure

Day 2: Start WASM-Rust; Start set up dev environment
Started dev containers (Do a quicktest)
https://code.visualstudio.com/docs/devcontainers/containers

webassembly: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm

Day 1: Get Rust installed; Got HelloWorld working; started using cargo
