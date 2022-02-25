fn print_string() {
    let s = String::from("Hello, world!");
    println!("{}", s);
}

fn print_tuple() {
    let mut t = (1, "2");
    t.0 = 2;
    println!("{}", t.0);
    println!("{}", t.1);
}

fn print_array() {
    let mut a: [i32; 3] = [1, 2, 3];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", a);
    println!("{:?}", &a[1..3]);
}   

fn main() {
    print_string();
    print_tuple();
    print_array();
}
