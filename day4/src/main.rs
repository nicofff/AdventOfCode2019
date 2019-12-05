fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn main() {
    let lower_bound = 264360;
    let upper_bound = 746325;

    let candidates: Vec<u32> = (lower_bound..=upper_bound).collect();

    let possible_password: Vec<Vec<u32>> = candidates.iter()
    .map(|&x| number_to_vec(x))
    .filter(|digits|{
        digits.windows(2).all(|arr| arr[0] <= arr[1])
    })
    .filter(|digits|{
        digits.windows(2).any(|arr| arr[0] == arr[1])
    }).collect();


    println!("Possible passwords: {}", possible_password.iter().count());


    // part2

    let part2_passwords = possible_password.iter()
    .filter(|&digits|{
        digits.iter().fold(vec![0;10],|mut acc: Vec<u32>,&digit| {
            acc[digit as usize] +=1;
            acc
        }).iter()
        .any(|&digit_instances| digit_instances == 2)
    });

    println!("Possible passwords: {}", part2_passwords.count());
}
