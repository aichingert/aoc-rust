// Advent of Code 2017, day 24
// (c) aichingert

#[derive(Clone,Debug)]
struct Component {
    ports: [u32;2],
    used: [bool;2]
}

#[derive(Clone,Debug)]
struct Bridge {
    connections: Vec<Component>,
}

impl Component {
    fn new(ports: [u32;2]) -> Component {
        Component { ports, used: [false;2] }
    }
}

impl Bridge {
    fn new() -> Bridge {
        Bridge { connections: Vec::new() }
    }

    fn add(&mut self, component: Component) -> bool {
        let len = self.connections.len();

        match len {
            0 => self.connections.push(component),
            _ => 'main: for i in 0..2 {
                for j in 0..2 {
                    if !self.connections[len-1].used[i] && self.connections[len-1].ports[i] == component.ports[j] {
                        self.connections[len-1].used[i] = true;
                        self.connections.push(component);
                        self.connections[len].used[j] = true;
                        break 'main;
                    }
                } 
            },
        }

        self.connections.len() != len
    }

    fn strength(&self) -> u32 {
        self.connections.iter().map(|comp| comp.ports[0] + comp.ports[1]).sum::<u32>()
    }
}

fn part1(comps: &mut Vec<Component>) -> u32 {
    let mut possible = Vec::<Bridge>::new();
    let mut starting = Vec::<Component>::new();
    let mut i: usize = 0;

    while i < comps.len() {
        if comps[i].ports[0] == 0 {
            starting.push(comps.remove(i));
        } else {
            i += 1;
        }
    }

    for start in starting {
        let mut cur: Bridge = Bridge::new();
        cur.add(start.clone());
        rec_find(comps, &mut cur, &mut possible);
    }
    
    let mut ans = 0;

    for pos in possible {
        ans = ans.max(pos.strength());
    }

    ans
}

fn rec_find(comps: &mut Vec<Component>, cur: &mut Bridge, bridges: &mut Vec<Bridge>) {
    bridges.push(cur.clone());

    let mut i: usize = 0;

    while i < comps.len() {
        let mut clone = cur.clone();
        if clone.add(comps[i].clone()) {
            let comp = comps.remove(i);
            rec_find(&mut comps.clone(), &mut clone, bridges);
            comps.insert(i, comp);
        }

        i += 1;
    }
}

fn part2() {}

fn main() {
    let mut inp = std::fs::read_to_string("../input/24").unwrap()
        .lines()
        .map(|l| {let p = l.split_once('/').unwrap(); Component::new([p.0.parse::<u32>().unwrap(),p.1.parse::<u32>().unwrap()])})
        .collect::<Vec<Component>>();

    println!("Part 1: {}", part1(&mut inp));
    //println!("Part 2: {}", part2());
}
