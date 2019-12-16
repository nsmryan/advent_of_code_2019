

fn test_string() -> String {
	let test_string = "62259 75368 93740 119724 112546 137714 96999 130673 102398 73819 100734 85337 62764 82115 127696 54391 103213 77954 112513 112392 138404 92989 108521 83163 109720 91918 114443 54306 90623 66833 58505 85919 77539 149419 128385 66452 94677 109179 62072 137245 136226 145783 60689 103320 145931 101286 63458 122468 87858 105675 146185 57417 96883 70739 97494 140951 149416 83137 66122 134319 58511 139600 102929 112240 149634 64142 83332 129526 99058 148889 50087 74961 133606 143518 68849 97045 73920 61357 115941 56740 111773 77880 90792 77103 111355 125898 56547 84918 113822 74113 98557 80928 60519 146379 59354 102490 72584 59000 63151 114253".to_string();
	 return test_string;
}

fn fuel_for(mass: u32) -> u32 {
    return (mass / 3) - 2;
}

fn fuel_iteration(fuel: u32) -> u32 {
    let mut result: u32 = 0;
    let mut current: u32 = fuel;

    while current >= 6 {
        current = fuel_for(current);
        result += current;
    }

    return result;
}

fn main() {
	let test_string = test_string();
	let result: u32 =
		test_string.split_whitespace()
                   .map(|word| word.parse::<u32>().unwrap())
		           .map(|mass| fuel_for(mass))
                   .map(|fuel| fuel + fuel_iteration(fuel))
                   .sum();

	println!("Result = {}", result);
}
