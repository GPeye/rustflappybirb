# Rust Based Flappy Bird Clone

yet another flappy bird clone. Just a personal excercise to get comfortable with the Bevy framework and ECS system.

## Features:
- 2d game with heron collision
- Uses states to move between playing and gameover
- Setup / Cleanup pattern
- super basic and minimal animation
- basic UI that updates with score
- Sounds
- Hot reload of game_config.ron so you can edit while the game runs, it will only take effect though once you gameover and restart
- Hopefully a sane and scalable structure


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
- [x] edit game settings with config with hot load