use std::thread::Thread;
use std::sync::{Arc, Mutex, mpsc};

fn main() {
	let (tx, rx) = mpsc::channel();

	let data = [2, 3, 4, 5, 6, 7];

	for i in 0..data.len() {
		
		if i % 2 == 0 { continue; }
		let (data, tx) = (data.clone(), tx.clone());

		Thread::spawn(move || {
			let a = data[i-1];
			let b = data[i];

			let answer = a + b;
			tx.send(answer);
		});
	}

	let mut sum = 0;
	
	for _ in 0..data.len()/2 {
		match rx.recv() {
			Ok(result) => {
				sum += result;
				println!("{}", result);
			},
			Err(e) => println!("Error: {:?}", e), 
		}
	}

	println!("{:?}", sum);
}
