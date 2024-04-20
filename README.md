# Sorting Library

This Rust library provides sorting algorithms including quick sort, selection sort, insertion sort, and merge sort.

## Installation

Before you begin, ensure that the folder where you want to create your Cargo project does not contain any Cyrillic or other special characters.

1. Create a new Cargo project by running the following command in your terminal:
   ```bash
   cargo init [folder name]
   ```

2. Navigate into the newly created `sort` directory:
   ```bash
   cd [folder name]
   ```

3. Open the `Cargo.toml` file in a text editor and add the following dependencies for your sorting library:
   ```toml
   [dependencies]
   sorting_library = { git = "https://github.com/vuilae/bl-sorting.git" }
   ```
Save the `Cargo.toml` file and close the text editor.

4. Import functions from sorting_library
   ```main.rs
   use sorting_library::sorting::{quick_sort, selection_sort, insertion_sort, merge_sort};
   ```
5. In order to use the functions, declare mutable array of any object type, ex strings, chars, floats, numbers.

```bash
    let mut floats = vec![3.5, 1.2, 5.6, 2.3, 4.7];
    selection_sort(&mut floats);
    println!("Selection Sorted: {:?}", floats);
```

Using all functions looks like this:
```main.rs
fn main() {
    let mut numbers = vec![5, 2, 7, 3, 9, 1, 4, 6, 8];
    quick_sort(&mut numbers);
    println!("Quick Sorted: {:?}", numbers);

    let mut chars = vec!['c', 'b', 'a', 'e', 'd'];
    merge_sort(&mut chars);
    println!("Merge Sorted: {:?}", chars);

    let mut floats = vec![3.5, 1.2, 5.6, 2.3, 4.7];
    selection_sort(&mut floats);
    println!("Selection Sorted: {:?}", floats);

    let mut strings = vec!["banana", "apple", "orange", "grape", "kiwi"];
    insertion_sort(&mut strings);
    println!("Insertion Sorted: {:?}", strings);
    
}
```

To build and run your Rust program, use the following command in the terminal:
!Remember there shouldn't be any Cyrillic or other special characters in your path.
```bash
cargo build
cargo run
```

This is after build and run:

![image](https://github.com/vuilae/bl-sorting/assets/114561182/8a287c35-1edc-4c12-8a34-af4fe09dbd59)

## License

MIT License

Copyright (c) 2024 Darina Bakeyeva
