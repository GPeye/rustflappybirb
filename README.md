# Rust Based Flappy Bird Clone

yet another one. Just a personal excercise to get comfortable with the Bevy framework and ECS system.


This particular project isn't intended to run in wasm so just:
```
cargo run
```

### Todo
- [x] add background asset
- [x] Implement Game Over state where things stop
- [x] Clicking restarts the game
- [x] Collision with ground goes to game over states
- [x] Collision with pipes goes to game over
- [x] Load sprite assets for bird
- [x] animate bird based on y velocity?
- [x] add assets to pipe
- [x] Randomize pipe heights
- [x] sound effects
- [x] display score
- [ ] edit game settings with config (hot load?)