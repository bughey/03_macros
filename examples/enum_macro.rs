use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right(u32, u32),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

fn main() {
    // let up = Direction::Up(DirectionUp::new(10));
    let up: Direction = DirectionUp::new(10).into();
    let left: Direction = 10.into();
    println!("up: {:?}, left: {:?}", up, left);
}
