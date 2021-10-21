fn main() {
    let mut input: Vec<i32> = vec![
        32, 25436543, 423, 234, 25, 34653, 24356, 2456, 34562, 324, 8, 65, 3256, 56437, 765, 345,
        63, 22435, 63567, 345, 6345, 635462, 3546, 345,
    ];

    for i in 1..input.len() {
        let key = input[i];

        let mut j = i;

        while j > 0 && input[j - 1] < key {
            input[j] = input[j - 1];
            j -= 1;
        }
        input[j] = key;
    }

    println!("{:?}", input);
}
