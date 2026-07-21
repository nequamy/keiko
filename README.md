# KEIKO
a simple sorting algorithm visualizer in the terminal
![img](./assets/demo.gif)

Keiko, which means "practice, training" in Japanese, is a project for learning and honing my own skills. Building a TUI is new to me, and I started learning Rust about six months ago, spending most of that time on a project that uses network communication and on my thesis project.

Because of that, I was curious to try building this kind of application, so that later on I could build more complex applications for my own use, and I decided to create Keiko, whose goal is to display how sorting algorithms work.

## Next steps

I plan to expand this application with more sorting algorithms, as well as other, more complex algorithms. One of my future goals is also to build a sandbox. Inside this sandbox, users would be able to build their own algorithms based on this code, choose visualizations, and watch how they work.

## Algorithms

| Method | Implemented | Big O |
| :--- | :--- | :--- |
| Bubble Sort | yes | O(n^2) |
| Quick Sort | no | O(???) |
| Insertion Sort | no | O(???) |
| Selection Sort | no | O(???) |
| Heap Sort | no | O(???) |
| Merge Sort | no | O(???) |

p.s. in the TUI you can see an "other" method — right now it's just a stub, used to check whether list switching from ratatui works correctly or not.

## Build

```bash
cargo build
```

## Run

```
cargo run
```

## Controls

- `j` or `arrow down` - next algorithm
- `k` or `arrow up` - previous algorithm
- `enter` - start the animation
- `esc` during animation - stop the sort
- `esc` or `q` outside animation - quit the app
