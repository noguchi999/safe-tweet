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

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age
        }
    }

    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} old.", self.age);
        self
    }
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

fn print_vec() {
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![0; 5];
    println!("{}", v1[0]);
    println!("{:?}", &v2[0..5]);

    let v = vec![1,2,3,4,5];
    for e in &v {
        println!("{}", e);
    }
}

fn print_box() {
    let byte_array = [b'1', b'2'];
    println!("{:?}", Box::new(byte_array));
    print(Box::new(byte_array));
}
fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}

fn print_loop() {
    let mut count = 0;
    let result = loop {
        println!("count_1: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("result_1: {}", result);

    let mut count = 0;
    while count < 10 {
        println!("count_2: {}", count);
        count += 1;
    }

    for count in 0..10 {
        println!("count_3: {}", count);
    }

    let array = [0,1,2,3,4,5];
    for e in &array {
        println!("element: {}", e);
    }

    for i in 1..5 {
        println!("range: {}", i);
    }

    struct Iter {
        current: usize,
        max: usize
    }
    impl Iterator for Iter {
        type Item = usize;
        fn next(&mut self) -> Option<usize> {
            self.current += 1;
            if self.current - 1 < self.max {
                Some(self.current - 1)
            } else {
                None
            }
        }
    }
    let it = Iter {
        current: 0,
        max: 10
    };
    for n in it {
        println!("iter: {}", n);
    }
}

fn print_impl() {
    let p = Person {
        name: String::from("Taro"),
        age: 20
    };
    p.say_name().say_age();

    let p = Person::new("Hanako", 16);
    p.say_name().say_age();
}

trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uoooooh");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Cool");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Shubaaau!!");
    }
}

fn print_trait() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();

    let duck = Duck {};
    duck.tweet();
    duck.shout();
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
    print_vec();
    print_box();
    print_loop();
    print_impl();
    print_trait();
}
