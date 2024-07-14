use std::ops::IndexMut;

// Summary: Write a function which takes an array data of numbers and returns the largest difference in indexes j - i such that data[i] <= data[j]
fn main() {
    largest_difference(&[9, 4, 1, 10, 3, 4, 0, -1, -2]);
}

fn largest_difference(data: &[i32]) /*-> usize */
{
    let mut dif: i32 = 0;

    for i in data {
        for j in data {
            if i <= j {
                println!(
                    "{:?}",
                    data.iter().position(|&j| true).unwrap()
                        - data.iter().position(|&i| true).unwrap()
                );
            }
        }
        println!("{:?}", &i);
    }
}
