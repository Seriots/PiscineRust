use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::ops::{Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self {
        return Vector { x, y }
    }
}

impl Vector<f32> {
    fn length(self) -> f32 {
        return f32::sqrt(self.x*self.x + self.y*self.y)
    }
}

impl Vector<f64> {
    fn length(self) -> f64 {
        return f64::sqrt(self.x*self.x + self.y*self.y)
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, rhs: &Vector<T>) -> bool {
        return self.x == rhs.x && self.y == rhs.y
    }
}

impl<T: Eq> Eq for Vector<T> {}

impl<T: Add<Output = T>> std::ops::Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        return Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T: Sub<Output = T>> std::ops::Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        return Self {x: self.x - rhs.x, y: self.y - rhs.y}
    }
} 

impl<T: AddAssign> std::ops::AddAssign<Vector<T>> for Vector<T> {

    fn add_assign(&mut self, rhs: Vector<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> std::ops::SubAssign<Vector<T>> for Vector<T> {

    fn sub_assign(&mut self, rhs: Vector<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> std::ops::Mul<T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        return Self {x: self.x * rhs, y: self.y * rhs}
    }
}

impl<T: Div<Output = T> + Copy> std::ops::Div<T> for Vector<T> {
    type Output = Vector<T>;
    fn div(self, rhs: T) -> Self::Output {
        return Self {x: self.x / rhs, y: self.y / rhs}
    }
}

impl<T: DivAssign + Copy> std::ops::DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: MulAssign + Copy> std::ops::MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}


#[cfg(test)]
#[test]
fn test_add() {
    let v1 = Vector::new(5, 6);
    let v2 = Vector::new(7, 8);
    let v7: Vector<f32> = Vector::new(2.0, 2.0);

    let v3 = v1 + v2;

    let mut v4 = v3.clone();
    let mut v5 = v3.clone();

    v4 += v1;
    v5 *= 2;

    assert_eq!(v3, Vector::new(12, 14));
    assert_eq!(v4, Vector::new(17, 20));
    assert_eq!(v4 * 5, Vector::new(85, 100));
    assert_eq!(v5, Vector::new(24, 28));
    assert_eq!(v7.length(), f32::sqrt(8.0));

}


#[cfg(test)]
#[test]
fn test_a() {
    let v = Vector {
        x: String::from("Hello, World!"),
        y: String::from("Hello, Rust!"),
    };

    let w = v.clone();

    assert_eq!(&v, &w);
}

#[cfg(test)]
#[test]
fn test_b() {
    let v = Vector::new("Hello, World!", "Hello, Rust!");
    let a = v;
    let b = v;
    assert_eq!(a, b);
}
