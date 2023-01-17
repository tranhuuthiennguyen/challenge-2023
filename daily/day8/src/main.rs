use std::cmp::min;
use std::cmp::max;

fn area(head: &usize, tail: &usize, height: &[u32]) -> u32 {
    min(height[*head], height[*tail]) * (tail- head) as u32
}

fn find_max(height: &[u32]) -> u32 {
    let mut head = 0;
    let mut tail = height.len() - 1;

    let mut max_area = 0;

    while head != tail {
        max_area = max(max_area, area(&head, &tail, &height));
        if height[head] > height[tail] {
            tail-=1;
        } else {
            head+=1;
        }
    }

    max_area
}
fn main() {
    assert_eq!(find_max(&[1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(find_max(&[1, 1]), 1);
}
