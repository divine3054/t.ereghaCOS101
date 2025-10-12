fn main(){
	let toshiba_price:f64=450000.0;
	let mac_price:f64=2500000.0;
	let hp_price:f64=750000.0;
	let dell_price:f64=2850000.0;
	let acer_price:f64=250000.0;
	let toshiba_qty=2.0;
	let mac_qty=1.0;
	let hp_qty=3.0;
	let dell_qty=3.0;
	let acer_qty=1.0;

	let sum=(toshiba_price*toshiba_qty+mac_price*mac_qty+hp_qty*hp_price+dell_price*dell_qty+acer_price*acer_qty);
 	let total_qty=toshiba_qty+hp_qty+dell_qty+mac_qty+acer_qty;

 	let avg=sum/total;
 	println!("the sum is {},and the average is {}", sum,avg);
}