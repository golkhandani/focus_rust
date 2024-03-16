use std::collections::HashMap;

fn main() {
    println!("Focus Rust!");
    let mut donut_menu = HashMap::new();

    donut_menu.insert("Caramel", 300);
    donut_menu.insert("Chocolate", 100);
    donut_menu.insert("Vanilla Cream ", 200);

    for (key, val) in &donut_menu {
        println!(
            "{0:<50}...{1:<10}",
            format!("Price of a {} donut is:", key),
            val
        )
    }

    let banana = String::from("Banana");
    let banana_price: i32 = match donut_menu.get::<str>(&*banana) {
        Some(p) => *p,
        None => {
            println!("No item available");
            0
        }
    };
    println!("What is banana donut price!?\t{:?} ", banana_price);

    // You want to know why the hell &* ???
    // Okay, as you see, 'banana' is a String.
    // The 'get' method on the map requires a type with the 'Borrow' trait implemented.
    // String does not implement 'Borrow', but 'str' does. So, what's the solution?
    // '&variable' gives us a reference to the value of the variable.
    // '*value' will dereference the variable.
    // As we know (do we?), a String consists of two parts: value and pointer,
    // because it's dynamic data and resides in the heap. Blah, blah, blah.
    // So, by doing '*banana', we will get the value of the String.
    // The value type is 'str'. Cool.
    // Now, we need the pointer to that darn 'str' to use our 'get' method.
    // '&(*banana)' will give us the darn '&str' we need for the 'get' method!

    // a simple version without all the hacks are as below

    let mut donut_menu2 = HashMap::new();
    donut_menu2.insert(1, 300);
    let val2 = match donut_menu2.get(&1) {
        Some(p) => *p,
        None => {
            println!("No item available");
            0
        }
    };
    println!("What is banana donut price!?\t{:?} ", val2);
}
