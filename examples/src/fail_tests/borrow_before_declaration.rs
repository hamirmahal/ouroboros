use ouroboros::self_referencing;

#[self_referencing]
struct S {
    #[borrows(a)]
    b: &'this i32,
    a: i32,
}

fn main() { }
