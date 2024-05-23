```javascript
Usage: cargo run [OPTION]
    -h, --help      : print help message
    -F, --file      : set the file for delete
    -m, --move      : move the file
    -c, --copy      : copy the file
    -r, --rename    : rename the file
    -d, --delete    : delete the file
    -f, --from      : set the file from from location
    -t, --to        : set the file to to location
```

### Example:

- To Copy file from one location to another

```javascript
cargo run -- -c -f "./src/example.txt" -t "./example.txt"
```

- To Move file from one location to another

```javascript
cargo run -- -m -f "./src/example.txt" -t "./example.txt"
```

- To Delete file from one location to another

```javascript
cargo run -- -d -F "./src/example.txt"
```

- To Rename file

```javascript
cargo run -- -r -f "./src/example.txt" -t "./src/example.txt"
```
