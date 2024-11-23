fn main() {
    println!("Hello, world!");
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;
    let string = "hello world";
    let x1 = 42;
    let y1 = Box::new(84);
    {
        let z = (x1, y1);
    }
    let x2 = x1;
    // let y2 = y1;
}

// Chapter 1 of Rustaceans. Cover the same topics in book and elsewhere