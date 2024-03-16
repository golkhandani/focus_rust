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

// ------------------------------------
// ------------------------------------
// ------------------------------------

// Note there is no way to use derive on a trait
// we can user something like inheritance to
// build a supertrait with other traits inside
use std::{
    fmt::{Debug, Display, Formatter, Result},
    mem,
    rc::Rc,
    vec,
};

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

// Exact Error
// recursive type `Category` has infinite size
#[derive(Debug)]
struct Category {
    title: String,
    subcategory: Option<Box<Category>>,
}

// ------------------------------------
// ------------------------------------
// ------------------------------------
// RC
// What is RC? it stands for reference counting
// usage?
// imagine we have a few delivery-person who work for
// uber, door dash, and skip the dishes
// if any of the the applications goes out of market
// the delivery-person who works for them can still works for
// other applications, so we still need the ownership
// of the delivery-person
// so instead of passing owner ship of delivery-person we can
// pass read-only reference
// but here is the catch!!!
// if all the applications goes out of the market, we need to release
// the person from being a delivery, they can do another job
// (free the memory as soon as possible to make sure there the rest
// of the code can use that memory)
// here we can use RC
// ------------------------------------
// ------------------------------------
// ------------------------------------

#[derive(Debug)]
struct DeliveryPerson {
    social_number: String,
    name: String,
}

impl Drop for DeliveryPerson {
    fn drop(&mut self) {
        println!("Dropped {} - SIN: {}", self.name, self.social_number)
    }
}

fn main() {
    // Side note: to use a trait as type we have to use
    // dyn [trait] instead on [trait]

    // Box first usage
    println!("--------- BOX ----------");
    let shape: Box<dyn Shape>;
    shape = Box::new(Rectangle {
        x: 13.2,
        y: 32.1,
        title: "boxed rect".to_string(),
    });

    println!("{}", shape);

    // Box second usage
    let cat = Category {
        title: "category 1".to_string(),
        subcategory: None,
    };

    println!("{} -> {:#?}", cat.title, cat.subcategory.is_some());
    println!("---------RC----------");
    // RC usage

    // define delivery-person items
    let delivery_person1 = Rc::new(DeliveryPerson {
        name: "Person1".to_string(),
        social_number: "2drgdni2".to_string(),
    });
    let delivery_person2 = Rc::new(DeliveryPerson {
        name: "Person2".to_string(),
        social_number: "223dwni2".to_string(),
    });
    let delivery_person3 = Rc::new(DeliveryPerson {
        name: "Person3".to_string(),
        social_number: "2dni2312".to_string(),
    });

    // define delivery-applications
    // if we don't use RC to handle memory drops
    // code below will cause a compile error
    // use of moved value: `delivery_person1`, ...

    // by using RC we ensure that we only works with references
    // which means clone only copy the pointer
    // DOC:: Makes a clone of the Rc pointer.
    // and while we don't care what each application (uber, ...)
    // will do to the delivery person
    // we want to be sure that when there is no more reference
    // we will drop the memory allocation
    let uber = vec![
        delivery_person1.clone(),
        delivery_person2.clone(),
        delivery_person3.clone(),
    ];
    let door_dash = vec![delivery_person1.clone()];
    let skip_dishes = vec![delivery_person2, delivery_person3];

    // notice that all drivers still valid after
    // each drop
    // and finally after all applications dropped
    // we free the delivery person items in memory

    // I created a custom drop for DeliveryPerson
    // for better understanding
    println!("Uber delivery: {:?}", uber);
    mem::drop(uber);

    println!("Uber delivery: {:?}", door_dash);
    mem::drop(door_dash);

    println!("Uber delivery: {:?}", skip_dishes);
    mem::drop(skip_dishes);

    println!("Focus Rust!")

    // drop(skip_dishes) is dropping the last references of
    // delivery_person2, delivery_person3
    // the logs would be like
    // Dropped Person2
    // Dropped Person3
    // Focus Rust!
    // Dropped Person1
    // because delivery_person2, delivery_person3 passed ownership
    // to the skip_dishes vector and with dropping the vector
    // there is no other owner
    // while delivery_person1 is still owned by main function
}
