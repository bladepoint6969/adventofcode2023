use std::{
    collections::{HashMap, VecDeque},
    sync::OnceLock,
};

// my_useful.txt should be the array of the 4 conjunctors that feed into
// the conjunctor that feeds into rx
static USEFUL: [&str; 4] = include!("../my_useful.txt");

static USEFUL_1: OnceLock<u64> = OnceLock::new();
static USEFUL_2: OnceLock<u64> = OnceLock::new();
static USEFUL_3: OnceLock<u64> = OnceLock::new();
static USEFUL_4: OnceLock<u64> = OnceLock::new();

enum Module {
    FlipFlop {
        label: String,
        state: bool,
        connections: Vec<String>,
    },
    Conjunction {
        label: String,
        states: HashMap<String, bool>,
        connections: Vec<String>,
    },
    Broadcaster {
        label: String,
        connections: Vec<String>,
    },
}

impl Module {
    fn new(label: &str, connections: Vec<String>) -> Self {
        let stripped_label = label.replace(['%', '&'], "");
        if label.starts_with('%') {
            Self::FlipFlop {
                label: stripped_label,
                state: false,
                connections,
            }
        } else if label.starts_with('&') {
            Self::Conjunction {
                label: stripped_label,
                states: HashMap::new(),
                connections,
            }
        } else {
            Self::Broadcaster {
                label: stripped_label,
                connections,
            }
        }
    }

    fn label(&self) -> String {
        match self {
            Module::Broadcaster { label, .. } => label.clone(),
            Module::Conjunction { label, .. } => label.clone(),
            Module::FlipFlop { label, .. } => label.clone(),
        }
    }

    fn connections(&self) -> &[String] {
        match self {
            Module::Broadcaster { connections, .. } => connections,
            Module::Conjunction { connections, .. } => connections,
            Module::FlipFlop { connections, .. } => connections,
        }
    }

    fn initialize_inputs(&mut self, inputs: Vec<String>) {
        if let Self::Conjunction { states, .. } = self {
            for input in inputs {
                states.insert(input, false);
            }
        }
    }

    fn process(&mut self, source: String, pulse: bool, queue: &mut VecDeque<PulseTransmission>) {
        match self {
            Module::Broadcaster { label, connections } => {
                for connection in connections {
                    queue.push_back(PulseTransmission::new(
                        label.clone(),
                        connection.clone(),
                        pulse,
                    ));
                }
            }
            Module::FlipFlop {
                label,
                state,
                connections,
            } => {
                if !pulse {
                    *state = !*state;
                    for connection in connections {
                        queue.push_back(PulseTransmission::new(
                            label.clone(),
                            connection.clone(),
                            *state,
                        ));
                    }
                }
            }
            Module::Conjunction {
                label,
                states,
                connections,
            } => {
                states.insert(source, pulse);
                let to_send = states.values().filter(|v| **v).count() != states.len();
                for connection in connections {
                    queue.push_back(PulseTransmission::new(
                        label.clone(),
                        connection.clone(),
                        to_send,
                    ));
                }
            }
        }
    }
}

struct PulseTransmission {
    source: String,
    dest: String,
    pulse: bool,
}

impl PulseTransmission {
    fn new(source: String, dest: String, pulse: bool) -> Self {
        Self {
            source,
            dest,
            pulse,
        }
    }
}

fn initialize_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut inputs = HashMap::new();

    // Create Modules
    input.lines().for_each(|line| {
        let mut split = line.split(" -> ");
        let label = split.next().unwrap();
        let connections: Vec<String> = split
            .next()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect();
        let module = Module::new(label, connections);
        for input in module.connections() {
            let input_list = inputs.entry(input.clone()).or_insert(vec![]);
            input_list.push(module.label());
        }
        modules.insert(module.label(), module);
    });

    for (label, module) in &mut modules {
        if let Some(inputs) = inputs.remove(label) {
            module.initialize_inputs(inputs);
        }
    }

    modules
}

fn push_button(modules: &mut HashMap<String, Module>, pushes: u64) -> (u64, u64) {
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    let mut queue = VecDeque::new();

    queue.push_back(PulseTransmission::new(
        "button".into(),
        "broadcaster".into(),
        false,
    ));

    while let Some(transmission) = queue.pop_front() {
        if transmission.pulse {
            high_pulses += 1;
        } else {
            if let Some(idx) = USEFUL.iter().position(|&v| v == transmission.dest) {
                println!("Found {} at {}", transmission.dest, pushes);
                match idx {
                    0 => USEFUL_1.set(pushes).ok(),
                    1 => USEFUL_2.set(pushes).ok(),
                    2 => USEFUL_3.set(pushes).ok(),
                    3 => USEFUL_4.set(pushes).ok(),
                    _ => unreachable!(),
                };
            }
            low_pulses += 1;
        }
        if let Some(module) = modules.get_mut(&transmission.dest) {
            module.process(transmission.source, transmission.pulse, &mut queue);
        }
    }

    (high_pulses, low_pulses)
}

pub fn part1(input: &str) -> u64 {
    let mut modules = initialize_modules(input);

    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for pushes in 1..=1000 {
        let (high, low) = push_button(&mut modules, pushes);
        high_pulses += high;
        low_pulses += low;
    }

    let product = high_pulses * low_pulses;

    println!("{product}");
    product
}

pub fn part2(input: &str) -> u64 {
    let mut modules = initialize_modules(input);

    let mut count = 1;
    let mut lcm: u64;

    loop {
        push_button(&mut modules, count);

        if let (Some(idx0), Some(idx1), Some(idx2), Some(idx3)) = (
            USEFUL_1.get(),
            USEFUL_2.get(),
            USEFUL_3.get(),
            USEFUL_4.get(),
        ) {
            lcm = num::integer::lcm(*idx0, *idx1);
            lcm = num::integer::lcm(lcm, *idx2);
            lcm = num::integer::lcm(lcm, *idx3);
            break;
        }

        count += 1;
    }

    println!("{lcm}");
    lcm
}

#[cfg(test)]
mod tests {
    use crate::{initialize_modules, part1, push_button};

    #[test]
    fn test_push_button() {
        let input = include_str!("../input_simple_1.txt");
        let mut modules = initialize_modules(input);
        let (high, low) = push_button(&mut modules, 1);

        assert_eq!(high, 4);
        assert_eq!(low, 8);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple_1.txt");
        assert_eq!(part1(input), 32000000);

        let input = include_str!("../input_simple_2.txt");
        assert_eq!(part1(input), 11687500);
    }
}
