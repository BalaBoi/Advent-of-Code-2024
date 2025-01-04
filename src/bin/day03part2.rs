use std::usize;

use regex::Regex;

fn main() {
    let input_file = "inputs/day03.txt";
    let contents = std::fs::read_to_string(input_file).unwrap();

    let mul_rgx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_rgx = Regex::new(r"do\(\)").unwrap();
    let dont_rgx = Regex::new(r"don't\(\)").unwrap();

    let mut mul_iter = indices_iter(&mul_rgx, &contents).peekable();
    let mut do_iter = indices_iter(&do_rgx, &contents).peekable();
    let mut dont_iter = indices_iter(&dont_rgx, &contents).peekable();

    let op_iter = std::iter::from_fn(move || {
        let mul_op = mul_iter.peek();
        let dont_op = dont_iter.peek();
        let do_op = do_iter.peek();

        if mul_op.is_some()
            && mul_op.unwrap() < dont_op.unwrap_or(&usize::MAX)
            && mul_op.unwrap() < do_op.unwrap_or(&usize::MAX)
        {
            Some((mul_iter.next().unwrap(), OpType::Mul))
        } else if dont_op.is_some()
            && dont_op.unwrap() < mul_op.unwrap_or(&usize::MAX)
            && dont_op.unwrap() < do_op.unwrap_or(&usize::MAX)
        {
            Some((dont_iter.next().unwrap(), OpType::Dont))
        } else if do_op.is_some()
            && do_op.unwrap() < mul_op.unwrap_or(&usize::MAX)
            && do_op.unwrap() < dont_op.unwrap_or(&usize::MAX)
        {
            Some((do_iter.next().unwrap(), OpType::Do))
        } else {
            None
        }
    });

    let mut result = 0;
    let mut mul_allowed = true;
    for (index, optype) in op_iter {
        match optype {
            OpType::Mul => {
                if mul_allowed {
                    result += mul(index, &contents);
                }
            }
            OpType::Dont => {
                mul_allowed = false;
            }
            OpType::Do => {
                mul_allowed = true;
            }
        }
    }

    println!("The result is {result}");
}

enum OpType {
    Mul,
    Dont,
    Do,
}

fn indices_iter<'a>(regex: &'a Regex, haystack: &'a str) -> impl Iterator<Item = usize> + 'a {
    regex.find_iter(haystack).map(|m| m.start())
}

fn mul(index: usize, content: &str) -> usize {
    let rgx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let captures = rgx.captures(&content[index..]).unwrap();
    captures[1].parse::<usize>().unwrap() * captures[2].parse::<usize>().unwrap()
}
