# Rust Number Match Game

A simple CLI puzzle game written in Rust for practicing core Rust concepts.

The game generates a 5x5 grid with random numbers.  
The player selects two cells and removes them if:

- the numbers are equal
- or their sum equals `10`

The selected cells must also be next to each other.

---

## 🦀 What I practiced

- Structs and `impl`
- Ownership and borrowing
- Vectors (`Vec<T>`)
- Modules
- User input with `std::io`
- Loops and conditions
- Random number generation with `rand`

---

## 📁 Project Structure

```bash
src/
├── main.rs
├── grid.rs
├── element.rs
├── input.rs
└── big_print.rs
