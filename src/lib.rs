
use std::cmp::Ord;


pub fn quick_sort<T>(mut data: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    if data.len() <= 1 {
        return data;
    }

    let pivot_index = partition(&mut data);
    let (left, right) = data.split_at(pivot_index);

    let mut left_sorted = quick_sort(left.to_vec());
    left_sorted.append(&mut quick_sort(right.to_vec()));

    left_sorted
}

pub fn selection_sort<T>(mut data: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    for i in 0..data.len() - 1 {
        let mut min_index = i;
        for j in i + 1..data.len() {
            if data[j] < data[min_index] {
                min_index = j;
            }
        }
        data.swap(i, min_index);
    }

    data
}

pub fn insertion_sort<T>(mut data: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    for i in 1..data.len() {
        let mut key = data[i].clone();
        let mut j = i - 1;

        while j >= 0 && data[j] > key {
            data[j + 1] = data[j].clone();
            j -= 1;
        }

        data[j + 1] = key;
    }

    data
}

pub fn merge_sort<T>(mut data: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    if data.len() <= 1 {
        return data;
    }

    let mid = data.len() / 2;
    let left = merge_sort(data[..mid].to_vec());
    let right = merge_sort(data[mid..].to_vec());

    merge(left, right)
}

fn merge<T>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut result = Vec::new();
    while !left.is_empty() && !right.is_empty() {
        if left[0] < right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }

    result.append(&mut left);
    result.append(&mut right);

    result
}

fn partition<T>(data: &mut Vec<T>) -> usize
where
    T: Ord + Clone,
{
    let pivot = data.last().unwrap().clone();
    let mut i = 0;

    for j in 0..data.len() - 1 {
        if data[j] <= pivot {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, data.len() - 1);
    i
}
