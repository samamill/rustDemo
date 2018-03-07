fn mean(numbers: Vec<i32>) -> i32 {
	let mut sum = 0;
	for number in &numbers {
		sum += number;
	}
	let length = numbers.len() as i32;
	sum / length
}

fn median(numbers: Vec<i32>) -> i32 {
	let mut temp = numbers.clone();
	temp.sort();
	let index = temp.len() / 2;
	temp[index]
}

#[cfg(test)]
mod tests {
    #[test]
    fn mean_test() {
		let vector = vec![1, 2, 4, 5];
		let mean_int = ::mean(vector);
        assert_eq!(mean_int, 3);
    }
		
    #[test]
    fn median_test() {
		let vector = vec![1, 2, 5, 6, 3, 4, 5];
		let median_int = ::median(vector);
        assert_eq!(median_int, 4);
    }
	
	#[test]
    fn median_test2() {
		let vector = vec![3, 7, 1, 5, 4, 6];
		let median_int = ::median(vector);
        assert_eq!(median_int, 5);
    }
}
