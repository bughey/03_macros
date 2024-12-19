use anyhow::Result;

fn main() -> Result<()> {
    let v = my_vec![1, 2, 3];
    println!("{:?}", v);

    let v1 = <[i32]>::into_vec(Box::new([1, 2, 3, 4, 5]));
    println!("{:?}", v1);

    Ok(())
}

#[macro_export]
macro_rules! my_vec  {
    ($($x:expr),*) => {{
        let v = vec![$($x),*];

        /* let mut v = Vec::new();
        $(
            v.push($x);
        )* */

        v
    }};
}
