fn replace_string_literals(input: &str) -> String {
    let mut manipulated_string : String = input.to_string();
    let mut found_lookup: Vec<(usize, char)> = Vec::new();
    if input.find("one") != None {
        for (index, _) in input.match_indices("one") {
            found_lookup.push((index, '1'));
        }
    }
    if input.find("two") != None {
        for (index, _) in input.match_indices("two") {
            found_lookup.push((index, '2'));
        }
    }
    if input.find("three") != None {
        for (index, _) in input.match_indices("three") {
            found_lookup.push((index, '3'));
        }
    }
    if input.find("four") != None {
        for (index, _) in input.match_indices("four") {
            found_lookup.push((index, '4'));
        }
    }
    if input.find("five") != None {
        for (index, _) in input.match_indices("five") {
            found_lookup.push((index, '5'));
        }
    }
    if input.find("six") != None {
        for (index, _) in input.match_indices("six") {
            found_lookup.push((index, '6'));
        }
    }
    if input.find("seven") != None {
        for (index, _) in input.match_indices("seven") {
            found_lookup.push((index, '7'));
        }
    }
    if input.find("eight") != None {
        for (index, _) in input.match_indices("eight") {
            found_lookup.push((index, '8'));
        }
    }
    if input.find("nine") != None {
        for (index, _) in input.match_indices("nine") {
            found_lookup.push((index, '9'));
        }
    }
    if ! found_lookup.is_empty() {
        let first_found = found_lookup.iter().min_by_key(|&(value,_)| value);
        let last_found = found_lookup.iter().max_by_key(|&(value,_)| value);
        if first_found != None {
            let first_found_tuple = first_found.unwrap();
            manipulated_string.insert(first_found_tuple.0, first_found_tuple.1);
        }
        if last_found != None {
            let last_found_tuple = last_found.unwrap();
            if last_found != None && first_found != None {
                if last_found.unwrap().0 != first_found.unwrap().0 {
                    manipulated_string.insert(last_found_tuple.0+1, last_found_tuple.1);
                }
            }
        } 
    }
    // dbg!(&manipulated_string);
    manipulated_string
}

fn part1(input: &str) -> u32 {
    let result : u32;
    let mut params: Vec<i32> = Vec::new();
    let mut nr: i32;
    let lines = input.split("\n");
    for line in lines {
        let str_nr = line
                .replace(&['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'], "");

        // dbg!(&str_nr);
        if str_nr != "" {
            if str_nr.len() >= 2 {
                let first_digit: char = str_nr.chars().nth(0).unwrap();
                let last_digit:char = str_nr.chars().nth(str_nr.len()-1).unwrap();
                let mut str_dd: String = first_digit.to_string();
                str_dd.push(last_digit);
                // dbg!(&str_dd);
                nr = str_dd.parse::<i32>().unwrap();
            }
            else if str_nr.len() == 2{
                nr = str_nr.parse::<i32>().unwrap();
            }
            else if str_nr.len() == 1 {
                let digit: char = str_nr.chars().nth(0).unwrap();
                let mut str_sg: String = digit.to_string();
                str_sg.push(digit);
                // dbg!(&str_sg);
                nr = str_sg.parse::<i32>().unwrap();
            }
            else {
                panic!();
            }
            // dbg!(nr);
            params.push(nr);
        }
    }
    let sum: i32 = params.iter().sum();
    result = sum as u32;
    result
}

fn part2(input: &str) -> u32{
    let result : u32;
    let mut params: Vec<i32> = Vec::new();
    let mut nr: i32;
    let lines = input.split("\n");
    for line in lines {
        // dbg!(&line);
        let str_nr = replace_string_literals(line)
                .replace(&['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'], "");

        // dbg!(&str_nr);
        if str_nr != "" {
            if str_nr.len() >= 2 {
                let first_digit: char = str_nr.chars().nth(0).unwrap();
                let last_digit:char = str_nr.chars().nth(str_nr.len()-1).unwrap();
                let mut str_dd: String = first_digit.to_string();
                str_dd.push(last_digit);
                // dbg!(&str_dd);
                nr = str_dd.parse::<i32>().unwrap();
            }
            else if str_nr.len() == 2{
                nr = str_nr.parse::<i32>().unwrap();
            }
            else if str_nr.len() == 1 {
                let digit: char = str_nr.chars().nth(0).unwrap();
                let mut str_sg: String = digit.to_string();
                str_sg.push(digit);
                // dbg!(&str_sg);
                nr = str_sg.parse::<i32>().unwrap();
            }
            else {
                panic!();
            }
            // dbg!(nr);
            params.push(nr);
        }
    }
    let sum: i32 = params.iter().sum();
    result = sum as u32;
    result
}

fn main() {
    let input: &str = include_str!("./input.txt");
    // dbg!(input);
    let result: u32 = part1(input);
    let result2: u32 = part2(input);
    println!("Result part one = {} ", result);
    println!("Result part two = {} ", result2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works()
    {
        let sut: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        let sut2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result: u32 = part1(sut);
        let result2: u32 = part2(sut2);
        assert_eq!(result, 142);
        assert_eq!(result2, 281);
        let result_string: String = replace_string_literals("two1one");
        assert_eq!(result_string, "2two11one");
    }
}
