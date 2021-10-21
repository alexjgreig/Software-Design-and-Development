fn move_(n: u32, from: u32, to: u32, via: u32) {
    if n > 0 {
        move_(n - 1, from, via, to);
        // println!("Move disk from pole {} to pole {}", from, to);
        move_(n - 1, via, to, from);
    }
}

fn main() {
    move_(20, 1, 2, 3);
}
