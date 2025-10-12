fn main(){
	let principal: f64= 520000000.0;
	let rate: f64= 10.0;
	let no: f64= 5.0;
	let amount = principal * (1.0 + (rate / 100.0)).powf(no);
	let compound_interest=amount-principal;
	println!("the compound interest is {}",compound_interest);
}