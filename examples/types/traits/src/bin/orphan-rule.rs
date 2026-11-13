// ANCHOR: full
use std::fmt;

// ANCHOR: generic_coverage
#[derive(Debug, Clone, Copy)]
struct LocalId(u8);

// Allowed: `LocalId` appears uncovered in `From<LocalId>`, even though `From` and `Vec` are in `std`.
impl From<LocalId> for Vec<u8> {
    fn from(id: LocalId) -> Self {
        vec![id.0]
    }
}

// Forbidden (error[E0117]): `LocalId` is wrapped inside foreign `Vec<...>`:
// impl fmt::Display for Vec<LocalId> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "ids")
//     }
// }
// ANCHOR_END: generic_coverage

// ANCHOR: newtype_and_ext
struct IdList(Vec<LocalId>);

impl fmt::Display for IdList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.0.iter().map(|id| format!("#{}", id.0)).collect();
        write!(f, "[{}]", parts.join(", "))
    }
}

trait JoinArrowExt {
    fn join_arrow(&self) -> String;
}

impl JoinArrowExt for [i32] {
    fn join_arrow(&self) -> String {
        let parts: Vec<String> = self.iter().map(|n| n.to_string()).collect();
        parts.join(" -> ")
    }
}
// ANCHOR_END: newtype_and_ext

fn main() {
    let raw_bytes: Vec<u8> = Vec::from(LocalId(42));
    println!("uncovered generic impl: {raw_bytes:?}");

    let ids = IdList(vec![LocalId(1), LocalId(2), LocalId(3)]);
    println!("newtype Display: {ids}");

    let steps = vec![10, 20, 30];
    println!("extension trait: {}", steps.join_arrow());
}
// ANCHOR_END: full
