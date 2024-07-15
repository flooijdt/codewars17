use std::ops::IndexMut;

// Summary: Write a function which takes an array data of numbers and returns the largest difference in indexes j - i such that data[i] <= data[j]
fn main() {
    largest_difference(&[9, 4, 1, 10, 3, 4, 0, -1, -2]);
}

fn largest_difference(data: &[i32]) /*-> usize */
{
    for (i, item) in data.iter().enumerate() {
        for (j, aitem) in data.iter().enumerate() {
            if item <= aitem {
                println!("{:?}", data[j] - data[i]);
            }
        }
        println!("{:?}", &i);
    }
}
