fn main() {
    let mut input = vec![
        vec![3, 43, 23, 41, 53],
        vec![43, 23, 234, 43, 23],
        vec![32, 34, 421, 43667, 23, 54],
        vec![123, 3214341, 43, 234, 524],
        vec![32, 34, 421, 43667, 23, 54],
        vec![43, 23, 234, 43, 23],
        vec![44, 32423, 25654646, 43, 23],
    ];

    let mut array_swap = true;
    while array_swap == true {
        array_swap = false;
        for i in 0..input.len() - 1 {
            if input[i].iter().sum::<i32>() / input[i].len() as i32
                > input[i + 1].iter().sum::<i32>() / input[i + 1].len() as i32
            {
                let temp = input[i + 1].clone();
                input[i + 1] = input[i].clone();
                input[i] = temp;
                array_swap = true;
            }
        }
        println!("{:?}", input);
    }
}
