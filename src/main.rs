
use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use num::integer::lcm;

#[derive(Debug, Clone)]
struct Pattern {
    start: usize,
    period: usize
}

impl Pattern {
    fn combine(&self, other: &Self) -> Self {

        let mut step_self = self.start;
        let mut step_other = other.start;

        while step_self != step_other {

            if step_self > step_other {
                step_other += other.period;
            } else {
                step_self += self.period;
            }
        }

        Self { start: step_self, period: lcm(self.period, other.period) }
    }
}

enum Step {
    Left,
    Right
}

impl From<&char> for Step {
    fn from(value: &char) -> Self {
        match value {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => panic!("Error converting {} to step. Must be L/R", value)         
        }
    }
}

fn walk_nodes_fixed_points(nodes: &HashMap<String, (String, String)>, instructions: &Vec<Step>) -> usize {

    let mut node = "AAA".to_string();
    let end_node = "ZZZ".to_string();

    let mut steps = 0;

    for instruction in instructions.iter().cycle() {
        steps += 1;

        node = match instruction {
            Step::Left => nodes.get(&node).unwrap().0.clone(),
            Step::Right => nodes.get(&node).unwrap().1.clone()
        };

        if node == end_node {
            break;
        }
    }
    return steps;
}

fn detect_cycle_from(nodes: &HashMap<String, (String, String)>, instructions: &Vec<Step>, start: &String) -> HashMap<String, (usize, usize, usize)> {

    let mut first_occurences: HashMap<String, usize> = HashMap::new();
    let mut second_occurences: HashMap<String, (usize, usize)> = HashMap::new();
    let mut third_occurences: HashMap<String, (usize, usize, usize)> = HashMap::new();

    let mut steps = 0;

    let mut node = start.clone();
    let mut instructions_stream = instructions.iter().cycle();

    while third_occurences.is_empty() || !first_occurences.is_empty() || !second_occurences.is_empty() {

        steps += 1;

        let instruction = instructions_stream.next().unwrap();

        node = match instruction {
            Step::Left => nodes.get(&node).unwrap().0.clone(),
            Step::Right => nodes.get(&node).unwrap().1.clone()
        };

        if node.chars().nth(2).unwrap() == 'Z' {
            println!("Found exit {} on step {}", node.clone(), steps);
            if third_occurences.contains_key(&node) {}
            else if let Some(second_occurence) = second_occurences.remove(&node) {
                third_occurences.insert(node.clone(), (second_occurence.0, second_occurence.1, steps));
            } else if let Some(first_occurence) = first_occurences.remove(&node) {
                second_occurences.insert(node.clone(), (first_occurence, steps));
            } else {
                first_occurences.insert(node.clone(), steps);
            }
        }
    }

    return third_occurences;  
}

fn part2(nodes: &HashMap<String, (String, String)>, instructions: &Vec<Step>) -> usize {

    let start_nodes: Vec<String> = nodes.keys().filter(|key| key.chars().nth(2).unwrap()=='A').map(|key| key.clone()).collect();
    let pattern_data: Vec<HashMap<String, (usize, usize, usize)>> = start_nodes.iter().map(|start_node| detect_cycle_from(nodes, instructions, start_node)).collect();
    let patterns: Vec<Pattern> = pattern_data.iter().fold(Vec::new(), |mut res, data_set| {
        for (_, pattern) in data_set {
            if pattern.1 - pattern.0 != pattern.2 - pattern.1 {
                panic!("Strange pattern detected {:?}", pattern);
            }
            res.push(Pattern{start: pattern.0, period: pattern.1 - pattern.0});
        }
        res
    });

    let final_pattern = patterns[1..].iter().fold(patterns[0].clone(), |global_pattern, new_pattern| global_pattern.combine(new_pattern));
    return final_pattern.start;
}

fn main() {
   
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    let instructions: Vec<Step> = lines[0].chars().map(|c| (&c).into()).collect();

    let nodes: HashMap<String, (String, String)> = lines[2..].iter().fold(HashMap::new(), |mut nodes, line| {
        nodes.insert(line[0..3].to_string(), (line[7..10].to_string(), line[12..15].to_string()));
        nodes
    });

    println!("part 1: {}", walk_nodes_fixed_points(&nodes, &instructions));

    println!("part 2: {}", part2(&nodes, &instructions));
}