fn main() {
    let data = "0123456789";
    for _ in 0..3 {
        for i in 0..10 {
            for _ in 0..3 {
                print!("{}", data.chars().nth(i).unwrap());
            }
        }
        println!("");
    }
}
