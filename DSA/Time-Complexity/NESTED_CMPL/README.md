What is the time, space complexity of following code :

```python
    int a = 0, b = 0;
    for (i = 0; i < N; i++) {
        for (j = 0; j < N; j++) {
            a = a + j;
        }
    }
    for (k = 0; k < N; k++) {
        b = b + k;
    }
```

### Options

- [x] `O(N * N) time, O(1) space`
- [ ] `O(N) time, O(N) space`
- [ ] `O(N * N * N) time, O(1) space`
- [ ] `O(N * N) time, O(N) space`

## Explanation

### Time Complexity

The code consists of three loops: a nested loop and a single loop.

1. The nested for-loops:

   ```python
   for (i = 0; i < N; i++) {
       for (j = 0; j < N; j++) {
           a = a + j;
       }
   }
   ```

   - The outer loop runs `(N)` times.
   - For each iteration of the outer loop, the inner loop runs `(N)` times.
   - The body of the inner loop executes a single operation, `a = a + j`, which is `(O(1))`.

   Therefore, the time complexity of the nested loops is:
   `[ O(N) * O(N) = O(N^2) ]`

2. The single for-loop:

   ```python
   for (k = 0; k < N; k++) {
       b = b + k;
   }
   ```

   - The loop runs `(N)` times.
   - The body of the loop executes a single operation, `b = b + k`, which is `(O(1))`.

   Therefore, the time complexity of the single loop is: `[ O(N) ]`

- Combining the two parts, the overall time complexity is: `[ O(N^2) + O(N) ]`
- Since `(O(N^2))` dominates `(O(N))`, the overall time complexity is: `[ O(N^2) ]`

### Space Complexity

The space complexity depends on the amount of extra space used by the algorithm. In this code snippet:

- The variables `a`, `b`, `i`, `j`, and `k` are simple integers, requiring `(O(1))` space each.
- There are no additional data structures being used that depend on `(N)`.

Therefore, the overall space complexity is: `[ O(1) ]`

- **Time Complexity:** `(O(N^2))`
- **Space Complexity:** `(O(1))`


Answer:
- **O(N \* N) time, O(1) space**
