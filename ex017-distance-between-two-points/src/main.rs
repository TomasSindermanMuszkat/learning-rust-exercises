use text_io::read;
use std::io::{self, Write};

fn flushio(){
    io::stdout().flush().unwrap();
}

fn main() {
    struct Point {
        x: i32,
        y: i32
    }

    impl Point {
        fn create_point(x_coord: i32, y_coord: i32) -> Point{
            Point {
                x: x_coord,
                y: y_coord
            }
        }

        fn calculate_distance_between_two_points(point1:Point, point2:Point) -> f32 {
            ((i32::pow(point1.x - point2.x, 2) + i32::pow(point1.y - point2.y, 2)) as f32).sqrt()
        }
    }
    print!("Enter the value of the 'X' coordenate of the 1st point --> ");
    flushio();
    let mut x_coordenate: i32 = read!();
    print!("Enter the value of the 'Y' coordenate of the 1st point --> ");
    flushio();
    let mut y_coordenate: i32 = read!();
    let point_1 = Point::create_point(x_coordenate, y_coordenate);
    print!("Enter the value of the 'X' coordenate of the 2nd point --> ");
    flushio();
    x_coordenate = read!();
    print!("Enter the value of the 'Y' coordenate of the 2nd point --> ");
    flushio();
    y_coordenate = read!();
    let point_2 = Point::create_point(x_coordenate, y_coordenate);
    println!("Distance between the 1st point and the 2nd point is: {}", Point::calculate_distance_between_two_points(point_1, point_2));
}
