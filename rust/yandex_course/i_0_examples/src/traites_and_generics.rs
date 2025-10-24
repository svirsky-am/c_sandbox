// Define a trait with one required method and two default methods
trait Drawable {
    // Required: every implementor must provide area
    fn area(&self) -> f64;

    // Default method: uses `self.name()` and `self.area()`
    fn draw(&self) {
        println!("Drawing '{}' (area: {:.2})", self.name(), self.area());
    }

    // Another default method
    fn is_large(&self) -> bool {
        self.area() > 100.0
    }

    // Required method to access the name — or we could use a field directly if public
    fn name(&self) -> &str;
}

// Structs with a `name: String` field
#[derive(Debug)]
struct Circle {
    name: String,
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    name: String,
    width: f64,
    height: f64,
}

// Implement the trait for Circle
impl Drawable for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // Optional: override draw
    fn draw(&self) {
        println!(
            "⭕ Drawing circle '{}' with radius {}",
            self.name, self.radius
        );
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Implement the trait for Rectangle (use all defaults except required ones)
impl Drawable for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        &self.name
    }

    // Uses default `draw()` and `is_large()`
}

// Optional: a function that accepts any type implementing Parseble
fn render_shape<T: Drawable>(shape: T) {
    shape.draw();
}

pub fn fake_main() {
    let c = Circle {
        name: "Sun".to_string(),
        radius: 10.0,
    };

    let r = Rectangle {
        name: "Screen".to_string(),
        width: 20.0,
        height: 6.0,
    };

    c.draw(); // Uses overridden draw
    r.draw(); // Uses default draw

    render_shape(Circle {
        name: "Sun via generic".to_string(),
        radius: 10.0,
    }); // use generic
    render_shape(Rectangle {
        name: "Screen via generic".to_string(),
        width: 20.0,
        height: 6.0,
    }); // use generic

    println!("{} is large? {}", c.name(), c.is_large()); // true (area ~314)
    println!("{} is large? {}", r.name(), r.is_large()); // false (area = 120 → actually true! 120 > 100)
}
