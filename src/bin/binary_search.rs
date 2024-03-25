fn binary_search(haystack: &[i32], needle: i32) -> bool {
    let mut lo = 0usize;
    let mut hi = haystack.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;

        if haystack[m] == needle {
            return true;
        } else if needle > haystack[m] {
            lo = m + 1;
        } else {
            hi = m
        }
    }

    false
}

fn main() {
    let foo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    binary_search(&foo, 69);
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    const FOO: [i32; 11] = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    #[test]
    fn binary_search_array() {
        assert_eq!(binary_search(&FOO, 69), true);
        assert_eq!(binary_search(&FOO, 1336), false);
        assert_eq!(binary_search(&FOO, 69420), true);
        assert_eq!(binary_search(&FOO, 69421), false);
        assert_eq!(binary_search(&FOO, 1), true);
        assert_eq!(binary_search(&FOO, 0), false);
    }
}
