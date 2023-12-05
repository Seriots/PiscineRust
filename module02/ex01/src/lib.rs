struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx*dx + dy*dy).sqrt()
    }
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

#[cfg(test)]
#[test]
fn test_points() {
    let p1 = Point::zero();
    let mut p2 = Point::new(1.0, 1.0);

    assert_eq!(p1.distance(&p2), 2.0f32.sqrt());
    p2.translate(25.0, 42.0);
    assert_eq!(p1.distance(&p2), (26.0f32*26.0f32 + 43.0f32*43.0f32).sqrt());

}