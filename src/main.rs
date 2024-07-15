use std::ops::IndexMut;

// Summary: Write a function which takes an array data of numbers and returns the largest difference in indexes j - i such that data[i] <= data[j]
fn main() {
    largest_difference(&[9, 4, 1, 10, 3, 4, 0, -1, -2]);
}

fn largest_difference(data: &[i32]) /*-> usize */
{
    let mut dif: i32 = 0;

    let mut counter1 = 0;
    let mut counter2 = 0;

    for (i, item) in data.iter().enumerate() {
        for (j, aitem) in data.iter().enumerate() {
            if i <= j {
                println!(
                    "{:?}",
                    data.iter().count() - data.iter().position(|&i| true).unwrap()
                );
            }
            counter1 += 1;
        }
        println!("{:?}", &i);
        counter2 += 1;
    }
}
