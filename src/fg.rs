fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot = partition(arr);
    let (left, right) = arr.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(&mut right[1..]);
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

// 使用示例
fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("排序前: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("排序后: {:?}", numbers);
}
