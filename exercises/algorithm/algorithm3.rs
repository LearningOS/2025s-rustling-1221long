/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


use std::fmt::Debug;

fn sort<T>(array: &mut [T]) 
where T: Ord + Debug{
	//TODO
    // let mut i: usize = 0;
    // let mut tmp_max: T;
    // tmp_max = array[i].clone();
    // while i < array.len() {
    //     if tmp_max > array[i] {

    //     }

    //     i += 1;
    // }

    /*
    // bubble
    let n = array.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
    */

    /*
    // insertion
    let n = array.len();
    let mut i = 1;
    while i <= n {
        for j in i..n {
            if array[j] < array[j -1] {
                array.swap(j, j-1);
            }
        }
        i += 1;
    }
    */

    // heap
    let n = array.len();
    // println!("###---debug");

    // 构建最大堆
    for i in (0..n / 2).rev() {
        // println!("#{}#", i);
        // println!("{:?}", array);
        heapify(array, n, i);
        //println!("{:?}", array);
    }
    
    // 一个个取出元素
    for i in (1..n).rev() {
    println!("#{}#", i);
        println!("{:?}", array);
        array.swap(0, i);
        heapify(array, i, 0);
    println!("{:?}", array);
    }
    
}

// i is the biggest index, n is the check range of the array
// 
fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize ){
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // if i == largest, this function iter will over.
    // other world, if no change action, this function is over.
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	// #[test]
    // fn test_sort_2() {
    //     let mut vec = vec![1];
    //     sort(&mut vec);
    //     assert_eq!(vec, vec![1]);
    // }
	// #[test]
    // fn test_sort_3() {
    //     let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
    //     sort(&mut vec);
    //     assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    // }
}