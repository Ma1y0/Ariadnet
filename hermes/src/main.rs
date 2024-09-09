fn main() {
    let s = "                <h1>Hello World</h1>";
    let mut a = s.chars().peekable();

    println!("{:?}", a.peek());
    println!("{:?}", a.next());
    println!("{:?}", a.peek());
}
