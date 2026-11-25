fn main() -> Option<i32> {
    let num = 100;
    let res = typy_vm::add(num, num);
    println!("Hello, world! {num} plus {num} is {}!", res?);
    res
}
