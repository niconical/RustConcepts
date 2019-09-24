fn makes_copy(some_integer: i32) {  // some_integer comes into scope.
    println!("{}", some_integer);
}   // here, some_integer goes out of scope.Nothing special happens.

fn takes_ownership(some_string: String) {   // some_string comes into scope.
    println!("{}", some_string);
}   // here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

fn main() {
    let mut s = String::from("hello");
    println!("{}", s);
    
    s.push_str(", world");  // push_str() appends a literal to a string
    println!("{}", s);      // this will print 'hello, world'

    {
        let _s = String::from("hello");

        // do stuff with s
    }   // this scope is now over, and s is no
        // longer valid
    
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);

    // clone 
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}   // s3 goes out of scope and is dropped. s2 goes out of scope
    // but moved. s1 goes out of scope and is dropped.
