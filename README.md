# Triword Solver

Simple tool to solve [triword.net](https://triword.net) daily puzzle.

### Usage

```sh
triword <puzzle> <size>
triword "TAR/RICE/IL" 3
```

### DEMO
![image](https://github.com/cdrani/triword-solver/assets/18746599/8c308751-d27f-4cf3-ade2-61ae48bd2307)

<img width="986" alt="image" src="https://github.com/cdrani/triword-solver/assets/18746599/abfc65d1-e2cb-40bc-9fdc-a9ea7517a661">


### TODO | Future Optimizations

- [ ] pare down the words list (remove plurality, past tenses, etc)
- [ ] allow user to supply their own wordlist instead of builtin list.
- [ ] Separate library into own repo/crate and utilize in the solver tool.
- [ ] Create a puzzle generator. (Need to decide if it will be a part of this tool or separate.)

### License

[MIT](LICENSE) | @cdrani
