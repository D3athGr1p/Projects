What is the time complexity of the following code :

```python
        int a = 0, i = N;
        while (i > 0) {
            a += i;
            i /= 2;
        }
```

### Options

- [ ] `O(N)`
- [ ] `O(Sqrt(N))`
- [ ] `O(N / 2)`
- [x] `O(log N)`
- [ ] `O(log(log N))`

## Explanation

### Time Complexity

### Code Snippet

1. **While Loop:**
   ```cpp
   while (i > 0) {
       a += i;
       i /= 2;
   }
   ```
   - The condition of the while loop is `i > 0`.
   - Inside the loop, `a` is incremented by `i`.
   - The value of `i` is then halved (`i /= 2`).

### Iterations of the Loop

To determine the number of iterations of the while loop, we need to analyze how `i` changes with each iteration:

- Initially, `( i = N )`.
- In each iteration, `i` is halved: `i = N, N/2, N/4, N/8, ...`.

The loop continues until `i` becomes `0`. The number of times you can divide `N` by `2` before it becomes 0 is approximately `log_2(N)`.

Therefore, the number of iterations of the loop is `log_2(N)`.

### Time Complexity

Since the loop runs approximately `log_2(N)` times and each iteration involves a constant amount of work (adding `i` to `a` and dividing `i` by 2), the overall time complexity of the code is O(`log(N)`).

### Summary

The time complexity of the given code is:

- **O(log N)**
