# WOPR

A text-based simulator inspired by the WOPR computer in the 1983 film
*WarGames*. It runs entirely in your terminal and does not connect to real
systems.

## Run

```sh
cargo run
```

Type `HELP` to see commands. The simulator supports a film-inspired game
catalog, chat-style responses, a fictional NORAD map and status display, the
Falken's Maze password puzzle, a playable tic-tac-toe game, and an abstract
Global Thermonuclear War scenario. The strategic scenario is simulation-only:
it has no real targets or system connections and always ends with no winner.

Other catalog games are listed for authenticity; interactive play is currently
implemented for tic-tac-toe.
