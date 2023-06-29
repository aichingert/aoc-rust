// Advent of Code 2022, day 15
// (c) aichingert

const Y: i64 = 2000000;
const BOUNDS: i64 = 4000000;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

impl Sensor {
    fn new(dist: i64, x: i64, y: i64) -> Self {
        Sensor { dist, x, y }
    }

    fn dist(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    fn faster(sensors: &Vec<Sensor>, x: &mut i64, y: i64) -> bool {
        for sensor in sensors {
            if Sensor::dist(sensor.x, *x, sensor.y, y) <= sensor.dist {
                *x = sensor.x + sensor.dist - (sensor.y - y).abs();
                return true;
            }
        }

        false
    }
}

fn part_one(sensors: &Vec<Sensor>, x: (i64, i64)) -> i64 {
    let mut y_count: i64 = 0;
    let mut cur = x.0;

    while cur <= x.1 {
        let cx = cur;

        if Sensor::faster(sensors, &mut cur, Y) {
            y_count += cur - cx + 1;
        }
        cur += 1;
    }

    y_count - 1
}

fn part_two(sensors: &Vec<Sensor>, beacons: &Vec<(i64, i64)>) -> i64 {
    let mut i: i64 = 0;
    let mut j: i64 = 0;

    'outer: while i < BOUNDS {
        j = 0;
        while j < BOUNDS {
            if !Sensor::faster(sensors, &mut j, i) && !beacons.contains(&(j, i)) {
                break 'outer;
            }
            j += 1;
        }
        i += 1;
    }

    j * BOUNDS + i
}

fn parse() -> (Vec<Sensor>, Vec<(i64, i64)>, (i64, i64)) {
    let inp = std::fs::read_to_string("../input/15").unwrap();
    let mut sensors = Vec::<Sensor>::new();
    let mut beacons = Vec::<(i64, i64)>::new();
    let mut min_x = i64::MAX;
    let mut max_x = -i64::MAX;

    for line in inp.lines() {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let cords_sensor = sensor.split(' ').collect::<Vec<&str>>();

        let (_, sx) = cords_sensor[2].split_once("x=").unwrap();
        let (_, sy) = cords_sensor[3].split_once("y=").unwrap();
        let sx = sx[..sx.len() - 1].parse::<i64>().unwrap();
        let sy = sy.parse::<i64>().unwrap();

        let cords_beacon = beacon.split(' ').collect::<Vec<&str>>();

        let (_, bx) = cords_beacon[4].split_once("x=").unwrap();
        let by: i64 = cords_beacon[5].split_once("y=").unwrap().1.parse().unwrap();
        let bx: i64 = bx[..bx.len() - 1].parse().unwrap();

        sensors.push(Sensor::new(Sensor::dist(sx, bx, sy, by), sx, sy));
        beacons.push((bx, by));

        min_x = min_x.min(sx - sensors[sensors.len() - 1].dist);
        max_x = max_x.max(sx + sensors[sensors.len() - 1].dist);
    }

    (sensors, beacons, (min_x, max_x))
}

fn main() {
    let (sensors, beacons, x) = parse();

    println!("Part 1: {}", part_one(&sensors, x));
    println!("Part 2: {}", part_two(&sensors, &beacons));
}
