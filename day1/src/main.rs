fn majority_element(array: &[u8]) -> u8 {
    let mut majority_index=0;
    let mut count: u8 = 1;

    for i in 0..array.len() {
        if array[i] == array[majority_index as usize] {
            count+=1;
        } else {
            count-=1;
        }
        if count == 0 {
            majority_index = i;
            count = 1;
        }
    }

    array[majority_index] as u8
}

fn is_majority(array: &[u8], num: &u8) -> bool {
    let mut count = 0;
    for i in 0..array.len() {
        if array[i] == *num {
            count+=1;
        }
    }
    if count > array.len() / 2 { return true;}
    return false;
}

fn main() {
    let array : [u8; 5] = [3, 3, 3, 4, 4];
    let candidate = majority_element(&array);
    let is_majority = is_majority(&array, &candidate);
    if is_majority {
        println!("{} is majority element.", candidate);
    }
}