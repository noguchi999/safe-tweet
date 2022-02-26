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

struct Person {
    name: String,
    age: u32
}
fn print_struct() {
    let p = Person {
        name: String::from("野口　修"),
        age: 20
    };
    println!("{:?}", p.name);
    println!("{}", p.age);
}

enum Event {
    Quit,
    KeyDown(u8),
    MouseDown {x: i32, y: i32}
}
fn print_enum() {
    let q = Event::Quit;
    let m = Event::MouseDown {x: 10, y: 20};
}

fn print_option() {
    let option: Option<String> = Some(String::from("She is Legend"));
    match option {
        None => println!("fuga"),
        Some(s) => println!("{}", s)
    }
}

fn print_result_1() {
    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err)
    }

    let result: Result<i32, String> = Ok(200);
    println!("code_2: {}", result.unwrap_or(-1));
}

fn print_result_2() {
    let result: Result<i32, String> = Ok(200);
    println!("code_2: {}", result.unwrap_or(-1));

    let result: Result<i32, String> = Err(String::from("error."));
    println!("code_3: {}", result.unwrap_or(-1));
}

fn print_result_3() {
    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }

    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func);
    println!("{:?}", next_result);

    let result: Result<i32, String> = Err(String::from("error."));
    let next_result = result.and_then(func);
    println!("{:?}", next_result);
}

fn main() {
    print_string();
    print_tuple();
    print_array();
    print_struct();
    print_enum();
    print_option();
    print_result_1();
    print_result_2();
    print_result_3();
}
