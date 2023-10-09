# dec: Source decommenter

A command-line utility to remove the comments from scripts and source files.

## Usage

```

$ echo "/* my source */
fn main() {
    println!("Hi, planet."); // Non-standard message
}" > main.rs

$ dec main.rs
fn main() {
    println!("Hi, planet.");
}
```


