struct ConcreteOldCircle {
    radius: f64,
}

impl OldCircle for ConcreteOldCircle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let old_circle = ConcreteOldCircle { radius: 5.0 };
    let adapter = CircleAdapter { circle: Box::new(old_circle) };

    let area = adapter.get_area();
    println!("Area: {}", area);
}