# Triad - Puzzle Solver & Generator

Simple tool to solve and generate [triword.net](https://triword.net)-like puzzles.

### Triad

```sh
Triad Puzzle Generator and Solver

Usage: triad <COMMAND>

Commands:
  solve, -s, --solve        Solve Triad PUZZLE.
  generate, -g, --generate  Generate Triad PUZZLE. Provide KEY to base PUZZLE upon, otherwise random.
  help                      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Puzzle Solver

#### Usage

```sh
Solve Triad PUZZLE

Usage: triad {solve|--solve|-s} <PUZZLE> <SIZE>

Arguments:
  <PUZZLE>  PUZZLE to solve. Case insensitive. Must use / separator. Ex. TAR/RICE/IL
  <SIZE>    SIZE of missing letters in PUZZLE. Must be 3 or 4.

Options:
  -h, --help  Print help
```

#### Examples

```sh
triad solve <PUZZLE> <SIZE>
triad -s TAR/RICE/IL 3
triad --solve tar/rice/il 3

triad solve "gu/st/cas" 3
triad -s "GU / ST / CAS" 3
```

#### Output

```sh
Solution for puzzle TAR/RICE/IL with size 3 is AVA.
Words in puzzle: ["AVATAR", "AVARICE", "AVAIL"].

Solution for puzzle gu/st/cas with size 3 is AVA.
Words in puzzle: ["GUAVA", "AVAST", "CASAVA"].
```

### Puzzle Generator

#### Usage

```
Generate Triad PUZZLE. Provide KEY to base PUZZLE upon, otherwise random.

Usage: triad {generate|--generate|-g} [KEY]

Arguments:
  [KEY]  Generate PUZZLE based on KEY. KEY must 3 or 4 letters.

Options:
  -h, --help  Print help
```

#### Examples

```sh
triad generator car
triad -g car
triad --generator car

triad generator
triad --generator
triad -g
```

#### Output

```sh
Key: car
Puzzle: ["cargo", "cardamum", "vicar"]
```

### TODO | Future Optimizations

- [ ] pare down the words list (remove plurality, past tenses, etc)
- [ ] allow user to supply their own wordlist instead of builtin list.
- [ ] api with endpoints to solve and generate puzzles
- [ ] use a seed for randomization to generate the same 3 words with the same key provided a seek argument

### License

[MIT](LICENSE) | @cdrani
