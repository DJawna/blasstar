# Blaster Game
## How to build and run

### Requirements:

Rust and cargo need to be installed, and cargo

**Dependencies that musst be installed externally:**

Install the libraries (*.so and *.dll libs):

- SDL3 base libary [link](https://www.libsdl.org/) 
- SDL3 ttf libary [link](https://github.com/libsdl-org/SDL_ttf)


### Build:

```
cargo build
```

```
cargo run
```


## What is the game about

- 2D shootemp
- vertical scrolling
- item pickups
- ship upgrades (shields, weapons)
- Bossfights at the end of each level
- Bonus weapons


## Todos:

- [ ] implement start menu (with start new game and exit option)
  - [x] render the options and add red underline for selected option
  - [ ] make the selected option move arround on cursor up and down
  - [ ] implement enter button react to start and exit
- [ ] start the first level (with a placeholder texture for player)
- [ ] implement the pause screen (with unpause and exit option)
