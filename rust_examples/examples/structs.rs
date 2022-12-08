use core::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle r={}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 1 };
    println!("Hello {}", circle);
}
