fn main() {
    let input_file = "inputs/day07.txt";
    let contents = std::fs::read_to_string(input_file).unwrap();

    let mut result = 0;
    for equation in contents.lines() {
        let mut eq_split = equation.split(':');
        let test_num = eq_split.next().unwrap().parse::<usize>().unwrap();

        let nums = eq_split
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if valid_equation(test_num, &nums) {
            result += test_num;
        }
    }

    println!("The result is {}", result);
}

fn valid_equation(test_num: usize, nums: &Vec<usize>) -> bool {
    use Ops::*;
    valid_eq_recursive(test_num, nums, &[Mul]) || valid_eq_recursive(test_num, nums, &[Add])
}

fn valid_eq_recursive(test_num: usize, nums: &Vec<usize>, ops: &[Ops]) -> bool {
    use Ops::*;
    if nums.len() == ops.len() {
        if calc(nums, ops) == test_num {
            return true;
        } else {
            return false;
        }
    }

    if calc(nums, ops) > test_num {
        return false;
    }

    let mut vec1 = ops.to_vec();
    vec1.push(Mul);
    let mut vec2 = ops.to_vec();
    vec2.push(Add);
    valid_eq_recursive(test_num, nums, &vec1) || valid_eq_recursive(test_num, nums, &vec2)
}

fn calc(nums: &Vec<usize>, ops: &[Ops]) -> usize {
    use Ops::*;
    ops.iter()
        .zip(nums.iter())
        .fold(0, |acc, (op, num)| match op {
            Mul => acc * num,
            Add => acc + num,
        })
}

#[derive(Clone)]
enum Ops {
    Mul,
    Add,
}
