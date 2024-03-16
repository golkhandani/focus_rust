#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32,
    title: String,
}

impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("dropping rectangle {}", self.title)
    }
}

fn main() {
    {
        let rect = Rectangle {
            x: 2.32,
            y: 23.0,
            title: "Rectangle 1".to_string(),
        };
        println!("rect {:#?} = {} * {}", rect, rect.x, rect.y)
    }

    println!("Focus Rust!")
}
