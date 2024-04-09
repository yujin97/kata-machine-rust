fn partition(arr: &mut [i64], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];

    let mut idx: isize = lo as isize - 1;

    for i in lo..hi {
        if arr[i] <= pivot {
            idx += 1;
            let temp = arr[idx as usize];
            arr[idx as usize] = arr[i];
            arr[i] = temp;
        }
    }

    let idx = (idx + 1) as usize;

    arr[hi] = arr[idx];
    arr[idx] = pivot;

    idx
}

fn qs(arr: &mut [i64], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot = partition(arr, lo, hi);

    qs(arr, lo, pivot - 1);
    qs(arr, pivot + 1, hi);
}

fn quick_sort(arr: &mut [i64]) {
    qs(arr, 0, arr.len() - 1)
}

fn main() {
    let mut arr: [i64; 7] = [3, 34, -12, 3, 94, 68, 0];

    quick_sort(&mut arr);

    println!("sorted: {:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_sorts() {
        let mut arr: [i64; 7] = [9, 3, 7, 4, 69, 420, 42];

        quick_sort(&mut arr);

        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 420]);
    }
}
