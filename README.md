# Advent of Code in Rust - remmycat's solutions

## Benchmarks

Benchmarks are done using criterion.
The solvers always provide a solution to both parts of the day's puzzle in one go, and receive the input as an in-memory string or bytes (depending on what I preferred for that day).

Emojis reflect my feeling on the time and if it's improvable, based on intuition or other people's submissions.

Time precision depends on the variance measured during the benchmarks.

### 2023

Hardware: `MacBook Air (13-inch, 2020), Apple M1`

| Day | Name                            |     Time | Feeling |
| :-: | :------------------------------ | -------: | :-----: |
| 01  | Trebuchet?!                     |    33 μs |   🙂    |
| 02  | Cube Conundrum                  |  23.0 μs |   😎    |
| 03  | Gear Ratios                     |    52 μs |   🙂    |
| 04  | Scratchcards                    |   7.6 μs |   😎    |
| 05  | If You Give A Seed A Fertilizer |  17.7 μs |   🙂    |
| 06  | Wait For It                     |   155 ns |   😎    |
| 07  | Camel Cards                     |    42 μs |   😎    |
| 08  |                                 |          |         |
| 09  | Mirage Maintenance              |  42.6 μs |   🙂    |
| 10  | Pipe Maze                       |    46 μs |   😎    |
| 11  | Cosmic Expansion                |   9.5 μs |   🌈    |
| 12  | Hot Springs                     |  22.3 ms |   🤔    |
| 13  |                                 |          |         |
| 14  | Parabolic Reflector Dish        |  24.3 ms |   😕    |


### 2022

Hardware: `MacBook Pro (14-inch, 2021), Apple M1 Pro`

| Day | Name                    |          Time | Feeling |
| :-: | :---------------------- | ------------: | :-----: |
| 01  | Energy Counting         |       21.8 μs |   🙂    |
| 02  | Rock Paper Scissors     |       3.67 μs |   😎    |
| 03  | Rucksack Reorganization |        7.7 μs |   😎    |
| 04  | Camp Cleanup            |       34.1 μs |   🙂    |
| 05  | Supply Stacks           |       15.8 μs |   😎    |
| 06  | Tuning Trouble          |        5.9 μs |   😎    |
| 07  | No Space Left On Device |       18.2 μs |   😎    |
| 08  | Treetop Tree House      |        210 μs |   🤔    |
| 09  | Rope Bridge             |        241 μs |   🙂    |
| 10  | Cathode Ray Tube        |        1.5 μs |   😎    |
| 11  | Monkey in the Middle    |        2.0 ms |   😕    |
| 12  | Hill Climbing Algorithm | ~20 min[^bfs] |   😛    |
| 13  | Distress Signal         |        392 μs |   😕    |
| 14  | Regolith Reservoir      |        133 µs |   🙂    |
| 15  | Beacon Exclusion Zone   |       4.28 µs |   😎    |

[^bfs]: via BFS, as in [Big Friendly Sharpie](https://github.com/remmycat/advent-of-code-rs/blob/main/2022/days/12-hill-climbing-algorithm/nope.jpg)

### 2021

Hardware: `MacBook Pro (13-inch, 2019), 2.8 GHz Quad-Core Intel Core i7`

(Day 15 and 19, yikes…)

| Day | Name                    |    Time | Feeling |
| :-: | :---------------------- | ------: | :-----: |
| 01  | Sonar Sweep             |   82 μs |   😎    |
| 02  | Dive!                   |  277 μs |   😎    |
| 03  | Binary Diagnostic       |  1.3 ms |   🤨    |
| 04  | Giant Squid             |  6.2 ms |   😖    |
| 05  | Hydrothermal Venture    |  7.5 ms |   😖    |
| 06  | Lanternfish             |  4.9 μs |   😎    |
| 07  | The Treachery of Whales | 3.25 ms |   😖    |
| 08  | Seven Segment Search    |  1.2 ms |   🤨    |
| 09  | Smoke Basin             |  850 μs |   😎    |
| 10  | Syntax Scoring          |   95 μs |   😎    |
| 11  | Dumbo Octopμs           | 1.69 ms |   🤨    |
| 12  | Passage Pathing         |   44 μs |   😎    |
| 13  | Transparent Origami     |  155 μs |   😎    |
| 14  | Extended Polymerization |   30 μs |   😎    |
| 15  | Chiton                  |   1.9 s |   😭    |
| 16  | Packet Decoder          |   35 μs |   😎    |
| 17  | Trick Shot              |   11 μs |   😎    |
| 18  | Snailfish               |   41 ms |   🤨    |
| 19  | Beacon Scanner          |   4.5 s |   😭    |
| 20  | Trench Map              | 10.2 ms |   😎    |
| 21  | Dirac Dice              |  4.1 ms |   🤨    |
