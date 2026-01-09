trait Shape {
    fn draw(&self);
}

struct Circle;
impl Shape for Circle {
    fn draw(&self) { println!("Drawing a circle"); }
}

struct Square;
impl Shape for Square {
    fn draw(&self) { println!("Drawing a square"); }
}

pub fn main() {
    // статическая диспетчеризация
    draw_shape(Circle);
    
    // динамическая диспетчеризация
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle), Box::new(Square)];
    for s in shapes {
        s.draw(); // вызывается через vtable
    }
}

fn draw_shape<T: Shape>(s: T) {
    s.draw();
} 