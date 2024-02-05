Letter Key:
x: task done
D: Done
T: Need to test test

- Figure out animation
- [c] Figure out how to use math-utils in compute-engine, so that we can
- - [x] read in test data from asset_reader
- - [x] make the data accessible for the JS side
- - [x] Figure out why data is not being piped through; possibly due to using another library
- - [x] Figure out how to use match in init or use some other init func; seems like match/result is causing the issue
- - [ ] Fix issue of numbers being passed in being not the expected values
- - [ ] Try/Catch issue in JS
- - - [ ] Why issue reading in file when wasm? 
- - [ ] Eventually go for getting info from server; use dummy function to set up init vals
- - - - [ ] Work around, read in data from JS side and pass to Rust?
- - - - [ ] Figure out how to send request from Rust to server
- - [] How to handle test data vs not test data? Use flag to determine which file to read?
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
- - [x] Figure out issue about reading in intial values (asset_reader)
- - [x] Convert initial values to complex
- - [ ] CURRENT_TASK  Figure out initialization of the u,v, h (fix test error of self)
- - [ ] Get passing of the u, v, h data to JS
- - [ ] Visualize h initial data
- - [ ] implement the actual time-stepping & algorithm
- - [T] Conversion back to float from complex
- [ ] Implement time step and fft of sol
- [ ] Ray tracing based on height of water
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
