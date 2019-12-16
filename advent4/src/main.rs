
const START: usize = 137683;
const END:   usize = 596253;


fn increases(password: &[usize]) -> bool {
    return password.windows(2).all(|pair| pair[0] <= pair[1]);
}

fn duplicate(password: &[usize]) -> bool {
    return password.windows(2).any(|pair| pair[0] == pair[1]);
}

fn no_running_duplicates(password: &[usize]) -> bool {
    let mut run_count = 1;
    let mut prev = password[0];

    let mut has_two_count = false;

    for value in password[1..].iter() {
        if *value == prev {
            run_count += 1;
        } else {
            if run_count == 2 {
                has_two_count = true;
            }
            run_count = 1;
        }

        prev = *value;
    }

    if run_count == 2 {
        has_two_count = true;
    }

    return has_two_count;
}

fn passes(password: &[usize]) -> bool {
    return increases(password) && no_running_duplicates(password);
}

fn to_password(password_string: String) -> Vec<usize> {
    let password = password_string.chars().map(|chr| chr as usize - ('0' as usize)).collect::<Vec<usize>>();
    //dbg!(&password);
    return password;
}

#[test]
fn test_increases() {
    assert!(increases(&[1, 2, 3]));
    assert!(increases(&[1, 1, 1]));
    assert!(!increases(&[1, 1, 0]));
}

#[test]
fn test_duplicate() {
    assert!(duplicate(&[1, 1, 3]));
    assert!(duplicate(&[1, 2, 2]));
}

#[test]
fn test_passes() {
    assert!(passes(&[1, 1, 2, 2, 3, 3]));
    assert!(!passes(&[1, 2, 3, 4, 4, 4]));
    assert!(passes(&[1, 1, 1, 1, 2, 2]));
}

fn main() {
    let count: usize = (START..=END).map(|num| to_password(num.to_string()))
                            .filter(|password| passes(password))
                            .map(|_| 1)
                            .sum();

    println!("Result = {}", count);
}
