# Sorting Library

This Rust library provides sorting algorithms including quick sort, selection sort, insertion sort, and merge sort.

## Installation

Before you begin, ensure that the folder where you want to create your Cargo project does not contain any Cyrillic or other special characters.

1. Create a new Cargo project by running the following command in your terminal:
   ```bash
   cargo init sort
   ```

2. Navigate into the newly created `sort` directory:
   ```bash
   cd sort
   ```

3. Open the `Cargo.toml` file in a text editor and add the following dependencies for your sorting library:
   ```toml
   [dependencies]
   sorting_library = { git = "https://github.com/vuilae/bl-sorting.git" }
   ```

4. Save the `Cargo.toml` file and close the text editor.

Now you can import and use the sorting functions in your Rust code. Add the following code to your Rust file (e.g., `main.rs`):
```rust
use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    let mut numbers = vec![4444, 22, 369, 7111, 9910, 0, 3, -22222, -77, 554];
    println!("Original: {:?}", numbers);

    quick_sort(&mut numbers);
    println!("Quick Sorted: {:?}", numbers);

    let mut numbers2 = vec![4444, 22, 369, 7111, 9910, 0, 3, -22222, -77, 554];
    selection_sort(&mut numbers2);
    println!("Selection Sorted: {:?}", numbers2);

    let mut numbers3 = vec![4444, 22, 369, 7111, 9910, 0, 3, -22222, -77, 554];
    insertion_sort(&mut numbers3);
    println!("Insertion Sorted: {:?}", numbers3);

    let mut numbers4 = vec![4444, 22, 369, 7111, 9910, 0, 3, -22222, -77, 554];
    merge_sort(&mut numbers4);
    println!("Merge Sorted: {:?}", numbers4);
}
```

To build and run your Rust program, use the following command in the terminal:
```bash
cargo build
cargo run
```

This is after build and run:

![image](https://github.com/vuilae/bl-sorting/assets/114561182/d0fa5cda-2c38-47a2-9eff-f4a48a057d28)



```toml
[dependencies]
sorting_library = { git = "https://github.com/vuilae/bl-sorting.git" }


## License

MIT License

Copyright (c) 2024 Darina Bakeyeva
