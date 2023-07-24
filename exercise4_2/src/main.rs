use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

trait Printable {
    fn print(&self);
}

impl Printable for Complex {
    fn print(&self) {
        if self.imag >= 0.0 {
            println!("Complex: {} + {}i", self.real, self.imag);
        } else {
            println!("Complex: {} - {}i", self.real, -self.imag);
        }
    }
}

fn call_print(item: &dyn Printable) {
    item.print();
}

fn main() {
    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 2.0, imag: 3.0 };
    let result = c1 + c2;
    call_print(&result);
}