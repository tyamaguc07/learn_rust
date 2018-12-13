fn main() {
    let f: fn(i32) -> i32;

    f = plus_on;

    println!("{}", f(1));
    println!("{}", f(10));
    println!("{}", f(100));

}

fn plus_on(i: i32) -> i32 {
    i + 1
}

