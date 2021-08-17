# Game Of Life
- implementation in RUST
- compiled to WASM
- current state: game is displayed in the Terminal as unicode chars

![GOL](https://user-images.githubusercontent.com/44790691/129497550-5493e665-dde5-4542-b2f7-1b113474bc79.gif)


# ToDo's (improvements)
- [ ] drop matrix, index grid as single vector
- [ ] make `Grid` and `Cell` structs that are linked. So you only store references to the values. Each Cell should have `current_value` (bool), `next_value` (i8), neighbors (references of other cells? => i dont think that will work though, because you cannot mutate a Cell if you have read only references, but not 100% sure) 
- [ ] transform to library crate

# Grid Indexing (not implemented yet)
currently the grid is implemented as `Vec<Vec<bool>>`
```
 0  1  2  3  4  5  6  7  8  9
10 11 12 13 14 15 16 17 18 19
20 21 22 23 24 25 26 27 28 29
30 31 32 33 34 35 36 37 38 39
40 41 42 43 44 45 46 47 48 49
50 51 52 53 54 55 56 57 58 59
60 61 62 63 64 65 66 67 68 69
70 71 72 73 74 75 76 77 78 79
80 81 82 83 84 85 86 87 88 89
90 91 92 93 94 95 96 97 98 99
```

# Optimization of GOL Algorithm
- [Hashlife (predict future states)](https://jennyhasahat.github.io/hashlife.html)
- [Optimizing Game Of Life by Teds Lab](https://medium.com/tebs-lab/optimizing-conways-game-of-life-12f1b7f2f54c)
