# Blaster Game
## How to build and run

### Requirements:

Rust and cargo need to be installed, and cargo

**Dependencies that musst be installed externally:**

Install the libraries (*.so and *.dll libs):

- SDL3 base libary [link](https://www.libsdl.org/) 
- SDL3 ttf libary [link](https://github.com/libsdl-org/SDL_ttf)


### Build:

```console
cargo build
```

```console
cargo run
```

### Set the log verbosity

the following example will set the input category to debug verbosity level:

```console
SDL_LOGGING="input=debug" cargo run
```

## What is the game about

- 2D shootemp
- vertical scrolling
- item pickups
- ship upgrades (shields, weapons)
- Bossfights at the end of each level
- Bonus weapons


## Licenses

The Game itself:
- GPLv3 [License](./License)
- Font [see license in the font folder](./assets/fonts/LICENSE-OrbitronFont)

## Todos:

- [ ] fix bug: check if any input was true not just cursor up and down (maby implement equals method?)
- [x] add frame rate limit mechanism
- [x] add the event processing loop into the update state method
- [x] transform the esc button gesture to sdl event quit event
- [x] modify the start menu update function so it sends Quit event when button exit selected and user prsses enter
- [ ] implement start menu (with start new game and exit option)
  - [x] render the options and add red underline for selected option
  - [x] make the selected option move arround on cursor up and down
  - [ ] implement enter button react to start and exit
- [ ] start the first level (with a placeholder texture for player)
- [ ] implement the pause screen (with unpause and exit option)
- [ ] fix the text rendering so the textfields are not improperly scaled
- [ ] add frame rate counter to debug overlay
- [ ] remove anyhow again its bloated
