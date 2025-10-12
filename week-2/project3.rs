fn main(){
	let price:f64=510000.0;
	let rate:f64=5.0;
	let no:f64=5.0;

	let depreciation=price*((1-rate/100).powf(no));
	println!("the value after 3 years is{}",depreciation );
}