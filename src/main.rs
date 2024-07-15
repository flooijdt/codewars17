// Summary: Write a function which takes an array data of numbers and returns the largest difference in indexes j - i such that data[i] <= data[j]
fn main() {
    println!("{:?}", largest_difference(&[9, 4, 1, 10, 3, 4, 0, -1, -2]));
}

fn largest_difference(data: &[i32]) -> usize {
    let mut ini = 0;

    for (i, item) in data.iter().enumerate() {
        for (j, aitem) in data.iter().enumerate() {
            if item <= aitem && j > i && j - i > ini {
                ini = j - i;
            }
        }
    }
    ini
}
