#[macro_export]
macro_rules! my_vec  {
    ($($x:expr),*) => {{
        // let v = vec![$($x),*];

        let mut v = Vec::new();
        $(
            v.push($x);
        )*

        v
    }};
}

#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}
