fn main() {
    let num = 100;
    let res = typy_vm::add(num, num);
    println!("Hello, world! {num} plus {num} is {}!", res.unwrap());
}
