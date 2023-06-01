#CHECKERS INTERFACE: a standardized communications method for checkers engines and GUIs

----------------------------------

# COMMAND GUIDELINES

commands should be formatted such that they follow format
`<command> (parameters)`

----------------------------------

# GUI-SIDE COMMANDS

## `go depth`

asks the engine to go to a specified depth, e.g. `go depth 6`.

## `pcinewgame`

asks the engine to load a new game at the starting position.

## `move`

asks the engine to move (UCI-like coordinate notation):
- normal move:      `move e3f4`
- single capture:   `move f2d4`
- multiple capture: `move f2d4f6`
- multiple moves:   `move d4e3 f2d4f6`

## `endgame`

ends the game.

## `isready`

ensures that the engine is ready for input. no further commands shall be sent until a `readyok` response is recieved

----------------------------------

# ENGINE-SIDE COMMANDS

## `readyok`

shows that the engine is ready for input.

## `do move`

the engine should also send a move as parameter:
- normal move:      `do move e3f4`
- single capture:   `do move f2d4`
- multiple capture: `do move f2d4f6`
