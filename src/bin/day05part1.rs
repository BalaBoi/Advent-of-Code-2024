use std::collections::{HashMap, HashSet};

fn main() {
    let input_file = "inputs/day05.txt";
    let contents = std::fs::read_to_string(input_file).unwrap();
    let mut sections = contents.split("\n\n");
    
    let rules_str = sections.next().unwrap();
    
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    
    for rule_str in rules_str.lines() {
        let mut nums = rule_str.split('|');
        let num1 = nums.next().unwrap().parse::<usize>().unwrap();
        let num2 = nums.next().unwrap().parse::<usize>().unwrap();
        if rules.contains_key(&num1) {
            rules.get_mut(&num1).unwrap().insert(num2);
        } else {
            let set = HashSet::from([num2]);
            rules.insert(num1, set);
        }
    }

    let updates_str = sections.next().unwrap();

    let mut result = 0;
    
    for update_str in updates_str.lines() {
        let nums = update_str
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        
        let mut correctly_ordered = true;
        for i in 1..nums.len() {
            for prev in 0..i {
                if rules
                    .get(&nums[i])
                    .unwrap_or(&HashSet::default())
                    .contains(&nums[prev])
                {
                    correctly_ordered = false;
                    break;
                }
            }
        }

        if correctly_ordered {
            result += nums[nums.len()/2];
        }

    }

    println!("The result is {}", result);
}