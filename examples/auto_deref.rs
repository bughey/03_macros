use macros::AutoDeref;

#[derive(AutoDeref)]
// #[deref(meta(mutable = true))]
#[deref(mutable = false, field = "inner")]
pub struct RespBulkString {
    inner: String,
    // nothing: (),
}

fn main() {}
