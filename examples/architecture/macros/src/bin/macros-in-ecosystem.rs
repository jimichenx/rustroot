// ANCHOR: full
// ANCHOR: mini_json_dsl
#[derive(Debug, PartialEq)]
enum MiniJson {
    Bool(bool),
    Number(i64),
    Str(String),
    Object(Vec<(String, MiniJson)>),
}

impl From<bool> for MiniJson {
    fn from(v: bool) -> Self { Self::Bool(v) }
}
impl From<i64> for MiniJson {
    fn from(v: i64) -> Self { Self::Number(v) }
}
impl From<&str> for MiniJson {
    fn from(v: &str) -> Self { Self::Str(v.to_string()) }
}

macro_rules! mini_json {
    // Match JSON-like `{ "key": value, ... }` object literal syntax at compile time
    ({ $($key:literal : $val:expr),* $(,)? }) => {
        MiniJson::Object(vec![
            $(
                ($key.to_string(), MiniJson::from($val)),
            )*
        ])
    };
}
// ANCHOR_END: mini_json_dsl

fn main() {
    let event = mini_json!({
        "order_id": 9527,
        "paid": true,
        "gateway": "stripe",
    });

    println!("[内嵌 JSON DSL 展开结果] {event:?}");
}
// ANCHOR_END: full
