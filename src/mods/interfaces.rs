/* A Rust struct with a point trait and a implementation that allows me to move the point around in space with a function called translatePoint2d */
struct Point2d {
    x: i32,
    y: i32,
}

trait Point {
    fn translatepoint2d(&mut self, x: i32, y: i32);
}

impl Point for Point2d {
    fn translatepoint2d(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

pub fn translatepoint() {
    let mut point = Point2d { x: 5, y: 10 };
    println!("Point is at ({}, {})", point.x, point.y);
    point.translatepoint2d(10, 10);
    println!("Point is at ({}, {})", point.x, point.y);
}

/* A test that uses translatePoint to verify its function. */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_translatepoint() {
        let mut point = Point2d { x: 5, y: 10 };
        point.translatepoint2d(10, 10);
        assert_eq!(point.x, 15);
        assert_eq!(point.y, 20);
    }
}