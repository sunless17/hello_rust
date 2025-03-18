# create hello world
**cargo** automatically creates the project with a binary file

```bash
# create project
cargo new hello_world
# run project, use -p
cargo run --package hello_world
```

# helix ide exclusive
<kbd>g</kbd><kbd>f</kbd> = follow link

# quest
## markdown toc
using toc with helix doesn't produce the expected results (opens a new file #title after trying to follow link)

# lessons
## package vs bin
after reading
```bash
cargo help run
```
using --package makes more sense over --bin because the hello_world program is implied to be a *package*
