
pub mod sequential {
	use std::cmp::Ordering;
	use std::ops::{Add, Mul};
	
	pub fn factorial(n: u32) -> u32 {
		let mut a = 1;
		
		for i in 1..=n {
			a *= i;
		}
		
		a
	}
	
	pub fn equals<T: PartialEq>(a: &[T], b: &[T]) -> bool {
		if a.len() != b.len() {
			return false;
		}
		
		for (e1, e2) in a.iter().zip(b.iter()) {
			if *e1 != *e2 {
				return false;
			}
		}
		
		true
	}
	
	pub fn compare<T: PartialOrd>(a: &[T], b: &[T]) -> Option<Ordering> {
		for (e1, e2) in a.iter().zip(b.iter()) {
			match e1.partial_cmp(e2) {
				Some(cmp) if cmp == Ordering::Equal => continue,
				cmp => return cmp
			};
		}
		
		a.len().partial_cmp(&b.len())
	}
	
	pub fn matmul<T>(a: &[[T; 4]; 4], b: &[[T; 4]; 4]) -> [[T; 4]; 4]
		where T: Default + Copy + Mul<Output = T> + Add<Output = T> {
		let mut m = [[T::default(); 4]; 4];
		
		for i in 0..4 {
			for j in 0..4 {
				for k in 0..4 {
					m[i][j] = m[i][j] + a[i][k]*b[k][j];
				}
			}
		}
		
		m
	}
	
	pub fn quicksort<T: PartialOrd>(v: &mut [T]) {
		if v.len() <= 1 { return; }
		
		let mid = partition(v);
		let (lo, hi) = v.split_at_mut(mid);
		
		quicksort(lo);
		quicksort(&mut hi[1..]);
	}
	
	fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
		let pivot = v.len() - 1;
		let mut l = 0;
		let mut r = v.len() - 2;
		
		loop {
			while l < r && v[l] < v[pivot] { l += 1; }
			while l < r && v[r] >=v[pivot] { r -= 1; }
			
			if l >= r { break; }
			v.swap(l, r);
		}
		
		if v[l] >= v[pivot] { v.swap(l, v.len() - 1); l }
		else { v.len() - 1 }
	}
}

#[allow(unused_imports, unused_variables, dead_code)]
pub mod parallel {
	use std::cmp::Ordering;
	use std::ops::{Add, Mul};
	use rayon::prelude::*;
	
	pub fn factorial(n: u32) -> u32 {
		unimplemented!()
	}
	
	pub fn equals<T: PartialEq + Send + Sync>(a: &[T], b: &[T]) -> bool {
		unimplemented!()
	}
	
	pub fn compare<T: PartialOrd + Send + Sync>(a: &[T], b: &[T]) -> Option<Ordering> {
		unimplemented!()
	}
	
	pub fn matmul<T>(a: &[[T; 4]; 4], b: &[[T; 4]; 4]) -> [[T; 4]; 4]
	where T: Default + Copy + Mul<Output = T> + Add<Output = T> + Send + Sync {
		unimplemented!()
	}
	
	pub fn quicksort<T: PartialOrd + Send + Sync>(v: &mut [T]) {
		unimplemented!()
	}
	
	fn partition<T: PartialOrd + Send + Sync>(v: &mut [T]) -> usize {
		unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use proptest::prelude::*;
	use proptest::{collection, array};
	
	proptest! {
		#[test]
		fn factorial(n in 0..10u32) {
			prop_assert_eq!(sequential::factorial(n), parallel::factorial(n));
		}
		
		#[test]
		fn equals(v in collection::vec((0usize.., 0usize..), 0..100)) {
			let (v1, v2): (Vec<_>, Vec<_>) = v.into_iter().unzip();
			
			prop_assert_eq!(sequential::equals(&v1, &v2), parallel::equals(&v1, &v2));
			prop_assert!(parallel::equals(&v1, &v1));
			prop_assert!(parallel::equals(&v2, &v2));
		}
		
		#[test]
		fn compare(v1 in collection::vec(isize::MIN.., 0..100),
		           v2 in collection::vec(isize::MIN.., 0..100)) {
			use std::cmp::Ordering;
			prop_assert_eq!(sequential::compare(&v1, &v2), parallel::compare(&v1, &v2));
			prop_assert_eq!(parallel::compare(&v1, &v1), Some(Ordering::Equal));
			prop_assert_eq!(parallel::compare(&v2, &v2), Some(Ordering::Equal));
		}
		
		#[test]
		fn matmul(v1 in array::uniform4(array::uniform4(-10000..10000)),
		          v2 in array::uniform4(array::uniform4(-10000..10000))) {
			prop_assert_eq!(sequential::matmul(&v1, &v2), parallel::matmul(&v1, &v2));
		}
		
		#[test]
		fn quicksort(v in collection::vec(0usize.., 0..100)) {
			let mut v1 = v.clone();
			let mut v2 = v.clone();
			
			sequential::quicksort(&mut v1);
			parallel::quicksort(&mut v2);
			
			prop_assert_eq!(v1, v2);
		}
		
		#[test]
		fn quicksort_correct(mut v in collection::vec(0usize.., 1..100)) {
			sequential::quicksort(&mut v);
			
			let mut it = v.into_iter();
			let mut pred = it.next().unwrap();
			for curr in it {
				prop_assert!(pred <= curr);
				pred = curr;
			}
		}
	}
}
