#![allow(dead_code, unused_imports)]

use criterion::{Criterion, criterion_main, criterion_group};
use task::{sequential, parallel};

fn std_mpsc(count: usize) -> usize {
	let (px, cx) = std::sync::mpsc::channel();
	
	std::thread::spawn(move || {
		for i in 0..count {
			px.send(i).unwrap();
		}
	});
	
	std::thread::spawn(move || {
		let mut sum = 0usize;
		
		while let Ok(i) = cx.recv() {
			sum += i;
		}
		
		sum
	}).join().unwrap()
}

fn crb_mpmc(count: usize) -> usize {
	let (px, cx) = crossbeam::channel::unbounded();
	
	std::thread::spawn(move || {
		for i in 0..count {
			px.send(i).unwrap();
		}
	});
	
	std::thread::spawn(move || {
		let mut sum = 0usize;
		
		while let Ok(i) = cx.recv() {
			sum += i;
		}
		
		sum
	}).join().unwrap()
}

fn mpsc_vs_mpmc(c: &mut Criterion) {
	use criterion::{PlotConfiguration, AxisScale, BenchmarkId};
	
	let mut group = c.benchmark_group("Channel Bench");
	
	// set-up the benchmark parameters: give the 99% CI and log scale axes
	group.confidence_level(0.99);
	group.plot_config(
		PlotConfiguration::default()
			.summary_scale(AxisScale::Logarithmic)
	);
	
	// run the benchmark multiple times with exponentially increasing input 
	for count in (0..8).map(|i| 100 << i) {
		// time the standard library
		group.bench_function(
			BenchmarkId::new("Std MPSC", count), 
			|b| {
				b.iter(|| std_mpsc(count));
			}
		);
		
		// time the crossbeam library
		group.bench_function(
			BenchmarkId::new("Crb MPMC", count), 
			|b| {
				b.iter(|| crb_mpmc(count));
			}
		);
	}
	
	// generate the HTML report
	group.finish();
}

criterion_group!(benches, mpsc_vs_mpmc);
criterion_main!(benches);
