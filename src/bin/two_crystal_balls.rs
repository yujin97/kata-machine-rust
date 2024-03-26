fn two_crystal_balls(breaks: &[bool]) -> u32 {
    let jump_amount = {
        let len = breaks.len() as f64;
        len.sqrt() as u64
    };

    let total = breaks.len() as u64;
    let mut jump = 1;
    let mut current_point = jump_amount - 1;
    let mut current = breaks[current_point as usize];
    while !current && current_point + jump_amount < total {
        jump += 1;
        current_point += jump_amount;
        current = breaks[current_point as usize];
    }

    let mut start = (jump - 1) * jump_amount;
    if jump == jump_amount && current_point < total - 1 && !current {
        start = jump * jump_amount;
        current_point = total - 1;
    }

    for position in start..=current_point {
        if breaks[position as usize] {
            return position.try_into().unwrap();
        }
    }

    return total.try_into().unwrap();
}

fn main() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..10000);
    let mut data: [bool; 10000] = [false; 10000];

    for i in idx..10000 {
        data[i] = true;
    }

    let ans = two_crystal_balls(&data);
    println!("calculated ans: {}, ans: {}", ans, idx);
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::two_crystal_balls;

    #[test]
    fn test_two_crystal_balls() {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..10000);
        let mut data: [bool; 10000] = [false; 10000];

        for i in idx..10000 {
            data[i] = true;
        }

        assert_eq!(two_crystal_balls(&data), idx as u32);
    }
}
