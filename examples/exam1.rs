use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
// 为 Point 类型实现 Add trait，这样两个Point实例就可以直接相加
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
fn main() {
    let pp = Point { x: 1, y: 2 };
    let pq = Point { x: 3, y: 4 };
    //这里move了
    let p3 = pp + pq; // 这里直接用+号作用在两个Point实例上
                      // //这里不能再用会报错
                      // assert_eq!(p3.x, p1.x + p2.x);
                      // //这里不能用
                      // assert_eq!(p3.y, p1.y + p2.y);
    println!("{:?}", p3);
    let a = String::from("aa");
    println!("{a:?}");

    let mut vec = vec![1, 2, 3];
    vec.push(4);
}
