### Question: What is the time, space complexity of following code :
```python
        int a = 0, b = 0;    
        for (i = 0; i < N; i++) {
            a = a + rand();  
        }
        for (j = 0; j < M; j++) {
            b = b + rand();
        }
```
Assume that rand() is O(1) time, O(1) space function.

### Options
- [ ] O(N * M) time, O(1) space
- [ ] O(N + M) time, O(N + M) space
- [X] O(N + M) time, O(1) space
- [ ] O(N * M) time, O(N + M) space
- [ ] O(N * M) time, O(N * M) space



## Explanation
### Time Complexity

The code consists of two independent for-loops:

1. The first for-loop runs `(N)` times:
    ```python
    for (i = 0; i < N; i++) {
        a = a + rand();
    }
    ```
    - The loop runs `(N)` times.
    - The body of the loop executes a single operation, `a = a + rand()`, which is `(O(1))` since `rand()` is `(O(1))`.

    Therefore, the time complexity of the first loop is `(O(N))`.

2. The second for-loop runs `(M)` times:
    ```python
    for (j = 0; j < M; j++) {
        b = b + rand();
    }
    ```
    - The loop runs `(M)` times.
    - The body of the loop executes a single operation, `b = b + rand()`, which is `(O(1))` since `rand()` is `(O(1))`.

    Therefore, the time complexity of the second loop is `(O(M))`.

Since the two loops run independently of each other, we add their complexities. The overall time complexity of the code is:
`[ O(N) + O(M) = O(N + M) ]`

### Space Complexity

The space complexity depends on the amount of extra space used by the algorithm. In this code snippet:

- The variables `a` and `b` are simple integers, requiring `(O(1))` space.
- The variables `i` and `j` used in the for-loops are also simple integers, requiring `(O(1))` space.

The `rand()` function is stated to be `(O(1))` space, meaning it does not use any additional space that scales with input size.

Therefore, the overall space complexity of the code is: `[ O(1) ]`

### Summary

- **Time Complexity:** `(O(N + M))`
- **Space Complexity:** `(O(1))`