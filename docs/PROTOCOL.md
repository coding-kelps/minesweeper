# Minesweeper Protocol

## Introduction

This document describe the communication protocol between an AI and the minesweeper manager.
The manager creates two pipes. The first pipe is used to send commands from the manager to the AI.
The second pipe is used to send replies from the AI to the manager.
The AI uses standard input-output functions (e.g: scanf and printf in C, readln and writeln in Pascal,...)
therefore it can be written in any programming language.

## Commands

### START (--dimensions [x],[y]) (--mines-spawning-rate [rate])
Send to the manager, it generates a new grid entirely undiscovered.
The `dimensions` of the new grid and the `mines spawning rate` from (0 to 1) can be optionally specified.

In response the manager send a GRID command.

```
START --dimensions 10,10
GRID
---
 1X**
 1***
11***
***21
***1 
---
```

### PICK [x],[y]
Send to the manager, it discover a specific cell of the minesweeper grid.

In response the manager send a GRID command.

### GRID
Send by the manager, it sends in multiple lines an ASCII representation of the manager minesweeper grid.
The ASCII representation is contained between two "---" guard line, the element in the grid are defined like so:
  - a space ( ), define an empty discovered cell
  - a cross (X), define a mine discovered cell
  - a number (1-9), define a discovered cell next to mine cell (the number indicates the number of mine cell next to it)
  - an hashtag ('#'), define an undiscovered cell

like so:
```
GRID
---
 1X**
 1***
11***
***21
***1 
---
```

### END [discovery rate]
Send by the manager, it indicates to the AI that the game is finished.
It also return a `discovery rate` (from 0% to 100%), 100% indicates that the minesweeper
game was winned, less that the game was loosed.

### UNKWOWN [msg]
Send by the manager, it indicates that the previous command send by the AI isn't supported
by the manager. It goes with a `description message`.

### ERROR [msg]
Send by the manager, it indicates that the previous command send by the AI is supported
by the manager, but failed to process it. It goes with a `description message`.