// src/task6.rs
pub enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

pub fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => std::f64::consts::PI * r * r,
    }
}

pub fn use_shape() {
    let rect = Shape::Rectangle(1.0, 2.0);
    println!("The area of the rectangle is {}", calculate_area(rect));

    let circle = Shape::Circle(1.0);
    println!("The area of the circle is {}", calculate_area(circle));
}
