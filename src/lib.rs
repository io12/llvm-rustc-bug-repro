#[derive(Copy, Clone)]
pub enum Foo {
    A,
    B(u8),
}

pub fn foo() -> Box<[[[Foo; 50]; 50]; 50]> {
    Box::new([[[Foo::A; 50]; 50]; 50])
}
