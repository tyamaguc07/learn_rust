fn main() {
    tuple();
    tuple_assign();
    tuple_through_a_destructuring_let();
    one_element_tuple();
    tuple_indexing();
    function_point();
}

fn tuple() {
    let x = (1, "hello");
    let y: (i32, &str) = (1, "hello");

    println!("{:?}", x);
    println!("{:?}", y);
}

fn tuple_assign() {
    let mut x = (1, 2);
    let y = (2, 3);

    println!("{:?}", x);

    x = y;

    println!("{:?}", x);
}

fn tuple_through_a_destructuring_let() {
    let (x, y, z) = (1, "hoge", 2,);

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}

fn one_element_tuple() {
    let x = (0,);
    let y = (0);

    println!("{:?}", x); //=> (0,)
    println!("{:?}", y); //=> 0　
}

fn tuple_indexing() {
    let tuple = ("foo", "bar", "baz");

    println!("{}", tuple.0); //=> foo
    println!("{}", tuple.1); //=> bar
    println!("{}", tuple.2); //=> baz
}

fn function_point() {
    fn foo(x: i32) -> i32 {
        println!("{}", x);
        x
    }

    let x: fn(i32) -> i32 = foo;


    let y = foo(1);

}
