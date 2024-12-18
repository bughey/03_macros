use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    let v = my_vec![1, 2, 3];
    println!("{:?}", v);

    let v1 = <[i32]>::into_vec(Box::new([1, 2, 3, 4, 5]));
    println!("{:?}", v1);

    Ok(())
}
