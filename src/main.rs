// Note there is no way to use derive on a trait
// we can user something like inheritance to
// build a supertrait with other traits inside
use std::fmt::{Debug, Display, Formatter, Result};

trait Shape: Display + Debug {
    fn area(&self) -> f32;
}

#[derive(Debug)]

struct Rectangle {
    x: f32,
    y: f32,
    title: String,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        return self.x * self.y;
    }
}

impl Display for Rectangle {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "rect \"{}\" with dimensions: {} {}",
            self.title, self.x, self.y
        )
    }
}

// Box first use case
// 1. Box force rust to use heap for the memory allocation
// 2.
// Imagine we have a trait (or rust interface!!!)
// we want to have a variable of a struct
// that implemented trait functionality
// here we have a struct called Rectangle with Shape functionality
// when we declaring the variable to be type of Shape
// compiler does not know how much memory should allocate to the variable
// it could be rectangle with x, and y or an Triangle with a, b, and c
// so compiler show us an error telling us that they don't know how they
// have to behave about this (unexpected behavior)
// by telling the compiler that shape is a Box of Shape
// we telling them to use Heap, so somehow like Vec or String
// it is a dynamic size object

// Exact error:
// the size for values of type `dyn Shape` cannot be known at compilation time
// ------------------------------------
// Box second use case
// recursive data types
// for example we have categories that might have some categories inside
// as sub categories, we need to wrap it inside a box

// Exact Error
// recursive type `Category` has infinite size
#[derive(Debug)]
struct Category {
    title: String,
    subcategory: Option<Box<Category>>,
}

fn main() {
    // Side note: to use a trait as type we have to use
    // dyn [trait] instead on [trait]
    let shape: Box<dyn Shape>;
    shape = Box::new(Rectangle {
        x: 13.2,
        y: 32.1,
        title: "boxed rect".to_string(),
    });

    println!("{}", shape);

    let cat = Category {
        title: "category 1".to_string(),
        subcategory: None,
    };

    println!("{:#?}", cat);

    println!("Focus Rust!")
}
