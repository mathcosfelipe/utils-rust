pub fn validate(value: &str) -> bool{
    let numbers = value
        .chars()
        .filter(|s| !"./-".contains(s.to_owned()))
        .collect::<Vec<_>>();

    if numbers.len() != 11 || equal_digits(&numbers){
        return false;
    }

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[9].to_string().parse::<usize>().unwrap(){
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[10].to_string().parse::<usize>().unwrap(){
        return false;
    }

    return true;
}
