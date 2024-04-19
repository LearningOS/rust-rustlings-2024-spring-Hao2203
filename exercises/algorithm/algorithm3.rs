/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::{fmt::LowerExp, fs::read_link};

fn sort<T: Ord + std::fmt::Debug>(array: &mut [T]) {
    //TODO
    recursion(array);
}

fn recursion<T: Ord + std::fmt::Debug>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    } else if len == 2 {
        if array[0] > array[1] {
            array.swap(0, 1);
        }
        return;
    } else {
        let low = 0;
        let hight = array.len() - 1;
        dbg!(low, hight);
        let arr = array as *mut [T];
        let mut left = (low..=hight).filter(|index| array[*index] > array[0]);
        let mut right = (low..=hight).rev().filter(|index| array[*index] < array[0]);
        let mid = loop {
            match (left.next(), right.next()) {
                (Some(low), Some(hight)) => {
                    if low < hight {
                        unsafe {
                            (*arr).swap(low, hight);
                        }
                    } else {
                        array.swap(0, hight);
                        break hight;
                    }
                }
                (None, Some(hight)) => {
                    array.swap(0, hight);
                    break hight;
                }
                (Some(low), None) => {
                    array.swap(0, low - 1);
                    break low - 1;
                }
                _ => {
                    break 1;
                }
            }
        };
        let mid = if mid == 0 { mid + 1 } else { mid };
        let (a1, a2) = array.split_at_mut(mid);
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
