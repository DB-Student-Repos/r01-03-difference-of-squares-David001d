pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..=n).sum::<u32>(); // Calculate the sum of the first n natural numbers
    sum * sum // Return the square of the sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x * x).sum() // Calculate the sum of the squares of the first n natural numbers
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n) // Calculate the difference between the square of the sum and the sum of squares
}
