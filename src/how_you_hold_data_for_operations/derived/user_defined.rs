pub fn run(){
//     struct Person {
//         name: String,
//         age: u8
//     }
//     struct Unit;
    
//     struct Pair(i32, i32); //No named fields but has fields
    
//     //a struct with two fields
//     struct Point{
//         x: f32,
//         y: f32
//     }
//     struct Rectangle{
//         top_left : Point,
//         bottom_right: Point,
//     }

//     //Instantiate a point
//     let point : Point = Point {x: 10.3, y:0.4};
//     //access the fields of the point
//     println!("point coordinates: ({},{})", point.x, point.y);

//     let bottom_right: Point = Point {x:5.2, ..point};
//     println!("Second point: ({},{})", bottom_right.x, bottom_right.y);

//     //Destructure the point using a let binding
//     let point{
//         x: left_edge,
//         y: top_edge
//     } = point;

//     let rectangle: Rectangle = Rectangle{
//         top_left: Point {x: left_edge, y:top_edge},
//         bottom_right:                                    //incomplete
//     }

//     trait Shape{
//         fn new(length: i32, width: i32, name: &str) -> Self;
//         fn area (&self) -> i32;
//         fn set_length(&mut self, length: i32);
//         fn get_length(&self) -> i32;
//         fn set_width (&mut self, width: i32);
//         fn get_width(&self) -> i32;
//         fn set_name (&mut self, name: &str);
//         fn get_name (&self) -> &str;
//     }

//     struct Rect{
//         length: i32,
//         width: i32,
//         name: &'static str
//     }

}
