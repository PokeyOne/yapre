use super::*;
use crate::space::*;
use super::Collidable;

fn create_test_triangle() -> Triangle {
    Triangle::new([
        Point::new(0.0, 1.0, 1.0),
        Point::new(1.0, -1.0, 1.0),
        Point::new(-1.0, -1.0, 1.0)
    ])
}

#[test]
fn basic_collision_test() {
    let triangle = create_test_triangle();
    let ray = Ray::new(ORIGIN, Point::new(0.0, 0.0, 1.0));

    match triangle.intersection_point(&ray) {
        None => panic!("Ray should intersect"),
        Some(Collision { point, distance }) => {
            assert_eq!(Point::new(0.0, 0.0, 1.0), point);
            assert_eq!(1.0, distance);
        }
    };
}

#[test]
fn ray_backwards_into_triangle() {
    let triangle = create_test_triangle();
    let ray = Ray::new(ORIGIN, Point::new(0.0, 0.0, -1.0));

    match triangle.intersection_point(&ray) {
        None => panic!("Ray should not miss"),
        Some(Collision { point, distance }) => {
            assert_eq!(Point::new(0.0, 0.0, 1.0), point);
            assert_eq!(-1.0, distance);
        }
    };
}

#[test]
fn test_ray_collision_on_an_angle() {
    let triangle = create_test_triangle();
    let ray = Ray::new(
        Point::new(1.0, 1.0, 0.0),
        Point::new(-1.0, -1.0, 1.0)
    );

    match triangle.intersection_point(&ray) {
        None => panic!("Ray should intersect"),
        Some(Collision { point, distance }) => {
            assert_eq!(Point::new(0.0, 0.0, 1.0), point);
            println!("{}", distance);
            assert!((distance - 3.0f64.sqrt()).abs() < 0.00001);
        }
    };
}

#[test]
fn test_array_of_rays() {
    let mut triangle = create_test_triangle();
    triangle.shift(Point::new(0.0, 0.0, 2.0));

    let width = 100;
    let height = 50;

    for j in 0..height {
        let y: f64 = ((-j as f64) / (height as f64)) + 0.5;
        for i in 0..width {
            let x: f64 = ((i as f64) / (width as f64)) - 0.5;

            let ray = Ray::new(ORIGIN, Point::new(x, y, 1.0));
            let c = match triangle.intersection_point(&ray) {
                None => ' ',
                Some(_) => '#'
            };

            print!("{}", c);
        }
        print!("\n");
    }

    // Uncomment this line to print out the generated triangle
    // panic!();
}
