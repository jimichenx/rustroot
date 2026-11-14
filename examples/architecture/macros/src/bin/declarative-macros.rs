// ANCHOR: full
use std::fmt;

// ANCHOR: my_vec_macro
macro_rules! my_vec {
    // Arm 1: empty call `my_vec![]`
    () => {
        Vec::new()
    };
    // Arm 2: one or more expressions separated by commas (with optional trailing comma)
    ($($elem:expr),+ $(,)?) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($elem);
        )+
        temp_vec
    }};
}
// ANCHOR_END: my_vec_macro

// ANCHOR: newtype_macro
macro_rules! define_id_newtypes {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name(pub u64);

            impl From<u64> for $name {
                fn from(raw: u64) -> Self {
                    Self(raw)
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}({})", stringify!($name), self.0)
                }
            }
        )+
    };
}

// One line generates three distinct newtypes and six Trait impls
define_id_newtypes!(UserId, OrderId, SkuId);
// ANCHOR_END: newtype_macro

fn main() {
    let empty: Vec<i32> = my_vec![];
    let nums = my_vec![10, 20, 30,];
    println!("[my_vec!] empty = {empty:?}, nums = {nums:?}");

    let uid = UserId::from(1001);
    let oid = OrderId::from(9527);
    let sku = SkuId::from(42);
    println!("[批量 Newtype] {uid}, {oid}, {sku}");
}
// ANCHOR_END: full
