fn swap<A, B>(a: A, b:B) -> (B, A) {
    (b, a)
}

fn main() {
    let a ="world";
    let b ="hello";
    println!("{:?}", swap(a, b));
}
