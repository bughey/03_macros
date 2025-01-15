use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    // #[debug(skip = false)]
    inner: String,
    #[debug(skip)]
    nothing: (),
}

fn main() {
    let resp = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };

    println!("{:?}", resp);
}
