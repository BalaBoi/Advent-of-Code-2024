use std::{cmp::Ordering, collections::{HashMap, HashSet}};

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
        
        let mut already_correctly_ordered = true;

        for i in 1..nums.len() {
            for prev in 0..i {
                if rules
                    .get(&nums[i])
                    .unwrap_or(&HashSet::default())
                    .contains(&nums[prev])
                {
                    already_correctly_ordered = false;
                    break;
                }
            }
        }

        if !already_correctly_ordered {
            let mut ordered = nums.clone();
            ordered.sort_by(|a, b| {
                if rules.get(a).unwrap_or(&HashSet::default()).contains(b) {
                    Ordering::Less
                } else if rules.get(b).unwrap_or(&HashSet::default()).contains(a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            // let mut i = 1;
            // while i < ordered.len() {
            //     let mut j = 0;
            //     while j < i { 
            //         if rules.get(&ordered[i]).unwrap_or(&HashSet::default()).contains(&ordered[j]) {
            //             ordered.insert(i + 1, ordered[j]);
            //             ordered.remove(j);
            //             i -= 1;
            //         } else {
            //             j += 1;
            //         }
            //     }
            //     i += 1;
            // }
            result += ordered[ordered.len()/2];
        }
    }

    println!("The result is {}", result);
}