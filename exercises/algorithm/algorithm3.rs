/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::fs::read_link;

fn sort<T: Ord>(array: &mut [T]) {
    //TODO
    recursion(array);
}

fn recursion<T: Ord>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    } else if len == 2 {
        if array[0] > array[1] {
            array.swap(0, 1);
        }
        return;
    } else {
        let mid = 0;
        let mut low = 0;
        let mut hight = array.len() - 1;
        while low < hight {
            low = array
                .iter()
                .enumerate()
                .take_while(|(_, item)| **item > array[0])
                .next()
                .map(|item| item.0)
                .unwrap_or(hight);
            hight = array[low..len - 1]
                .iter()
                .enumerate()
                .rev()
                .take_while(|(_, item)| **item < array[0])
                .next()
                .map(|item| item.0)
                .unwrap_or(low);
            array.swap(low, hight);
        }
        array.swap(0, low);
        let (a1, a2) = array.split_at_mut(low);
        recursion(a1);
        recursion(a2);
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
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
