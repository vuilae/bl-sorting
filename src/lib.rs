
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn merge_sort<T: Ord + Clone + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut merged = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut merged);
    arr.copy_from_slice(&merged);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], merged: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut merged_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            merged[merged_index] = left[left_index].clone();
            left_index += 1;
        } else {
            merged[merged_index] = right[right_index].clone();
            right_index += 1;
        }
        merged_index += 1;
    }

    while left_index < left.len() {
        merged[merged_index] = left[left_index].clone();
        left_index += 1;
        merged_index += 1;
    }

    while right_index < right.len() {
        merged[merged_index] = right[right_index].clone();
        right_index += 1;
        merged_index += 1;
    }
}
