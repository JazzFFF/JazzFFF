pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            x,
            y,
            radius,
        }
    }

    pub fn area(&self) -> f64 { 
        self.x * self.radius * std::f64::consts::PI
    }
}