use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_multiply_matrices() {
	let matrix_a: Vec<Vec<f32>> = vec![
		vec![1.0, 2.0, 3.0],
		vec![4.0, 5.0, 6.0],
		vec![7.0, 8.0, 9.0],
	];
	let matrix_b: Vec<Vec<f32>> = vec![
		vec![9.0, 8.0, 7.0],
		vec![6.0, 5.0, 4.0],
		vec![3.0, 2.0, 1.0],
	];

	let expected_result: Vec<Vec<f32>> = vec![
		vec![30.0, 24.0, 18.0],
		vec![84.0, 69.0, 54.0],
		vec![138.0, 114.0, 90.0],
	];

	let result = multiply_matrices(matrix_a, matrix_b);

	assert_eq!(result, expected_result);
}

fn multiply_matrices(matrix_a: Vec<Vec<f32>>, matrix_b: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
	assert!(matrix_a[0].len() == matrix_b.len());
	let (row, common, col) = (matrix_a.len(), matrix_a[0].len(), matrix_b[0].len());
	let (a, b) = (Arc::new(matrix_a), Arc::new(matrix_b));
	let result = Arc::new(Mutex::new(vec![vec![0f32; col]; row]));
	let mut handle = Vec::new();
	for i in 0..row {
		for j in 0..col {
			let (a, b, r) = (a.clone(), b.clone(), result.clone());
			handle.push(thread::spawn(move || {
				r.lock().unwrap()[i][j] =
					(0..common).map(|k| a.clone()[i][k] * b.clone()[k][j]).sum();
			}));
		}
	}
	for handle in handle {
		handle.join().unwrap();
	}
	let x = result.lock().unwrap().clone();
	x
}
