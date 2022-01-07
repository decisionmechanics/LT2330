fn main() {
    const LENGTH: i32 = 10;

    let mut previous = 0;
    let mut current = 1;

    println!("{}", previous);

    for _ in 1..LENGTH {
        println!("{}", current);

        let next = current + previous;
        previous = current;
        current = next;
    }
}
