fn two_crystal_balls(breaks: &[bool]) -> Option<u64> {
    let jump_amount = {
        let len = breaks.len() as f64;
        len.sqrt() as u64
    };

    let total: u64 = breaks.len().try_into().unwrap();
    let mut jump = 1;
    let mut current_point = jump_amount - 1;
    let mut current = breaks[usize::try_from(current_point).unwrap()];
    while !current && current_point + jump_amount < total {
        jump += 1;
        current_point += jump_amount;
        current = breaks[usize::try_from(current_point).unwrap()];
    }

    let mut start = (jump - 1) * jump_amount;
    if jump == jump_amount && current_point < total - 1 && !current {
        start = jump * jump_amount;
        current_point = total - 1;
    }

    for position in start..=current_point {
        if breaks[usize::try_from(position).unwrap()] {
            return Some(position.try_into().unwrap());
        }
    }

    None
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

    if let Some(ans) = ans {
        println!("calculated ans: {}, ans: {}", ans, idx);
    } else {
        println!("no valid answer")
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::two_crystal_balls;

    #[test]
    fn it_works_with_random_number_within_10000() {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..10000);
        let mut data: [bool; 10000] = [false; 10000];

        for i in idx..10000 {
            data[i] = true;
        }

        assert_eq!(two_crystal_balls(&data).unwrap(), idx as u64);
    }

    #[test]
    fn it_works_when_there_is_no_solution() {
        let data: [bool; 10000] = [false; 10000];

        assert!(two_crystal_balls(&data).is_none());
    }

    #[test]
    fn it_works_when_the_break_point_is_first_element() {
        let data: [bool; 10000] = [true; 10000];

        assert_eq!(two_crystal_balls(&data).unwrap(), 0);
    }

    #[test]
    fn it_works_when_the_break_point_is_last_element() {
        let mut data: [bool; 10000] = [false; 10000];
        data[9999] = true;

        assert_eq!(two_crystal_balls(&data).unwrap(), 9999);
    }
}
