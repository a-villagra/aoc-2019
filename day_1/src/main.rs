fn calculate_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn main() {
    use std::io;
    let mut res = 0;
    loop {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
	    Ok(_) => {
		match input.trim().parse() {
		    Ok(mass) => {
			let mut fuel = calculate_fuel(mass);
			while fuel > 0 {
			    res += fuel;
			    fuel = calculate_fuel(fuel);
			}
		    }
		    Err(error) => break
		}
	    }
	    Err(error) => {
		println!("{:?}", error);
		break;
	    }
	}
    }
    println!("Total fuel: {:?}", res);
}
