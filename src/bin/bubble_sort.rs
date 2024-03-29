fn bubble_sort(arr: &[i64]) -> Vec<i64> {
    let mut arr = Vec::from(arr);
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    arr
}

fn main() {
    let data: [i64; 10] = [3, 49, 1, 5, -3, 87, 29, 420, -34, 69];

    let sorted = bubble_sort(&data);

    println!("before sorting: {:?}", data);
    println!("after sorting: {:?}", sorted);
}
