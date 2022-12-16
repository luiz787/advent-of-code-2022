#[derive(Eq, PartialEq, Hash, Debug)]
struct Beacon((i32, i32));

#[derive(Eq, PartialEq, Hash, Debug)]
struct Sensor((i32, i32));

impl Sensor {
    fn distance(&self, beacon: &Beacon) -> u32 {
        let x_distance = self.0 .0.abs_diff(beacon.0 .0);
        let y_distance = self.0 .1.abs_diff(beacon.0 .1);

        (x_distance + y_distance) as u32
    }
}

fn main() {
    let input = include_str!("../input");

    let pairs: Vec<_> = input
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(sensor, beacon)| (get_sensor_position(sensor), get_beacon_position(beacon)))
        .collect();

    (0..=4_000_000).into_iter().for_each(|y| {
        let mut no_beacon: Vec<_> = pairs
            .iter()
            .map(|(sensor, beacon)| (sensor, sensor.distance(beacon)))
            .map(|(sensor, distance)| definitely_does_not_contain_beacon(sensor, distance, y))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();

        no_beacon.sort();
        let mut merged_intervals = Vec::new();
        merged_intervals.push(no_beacon[0]);
        for i in no_beacon.iter().skip(1) {
            let curr = merged_intervals[merged_intervals.len() - 1];
            if curr.1 < i.0 {
                merged_intervals.push(*i);
            } else if curr.1 < i.1 {
                let new = (curr.0, i.1);
                merged_intervals.pop();
                merged_intervals.push(new);
            }
        }

        if merged_intervals.len() == 2 {
            let first = merged_intervals[0];

            let x = first.1 + 1;
            let result = x as u64 * 4_000_000 as u64 + y as u64;
            println!("{}", result);
            return;
        }
    });
}

fn definitely_does_not_contain_beacon(
    sensor: &Sensor,
    distance: u32,
    y: i32,
) -> Option<(i32, i32)> {
    let y_diff = y.abs_diff(sensor.0 .1);
    let x_diff = distance as i32 - y_diff as i32;

    if x_diff <= 0 {
        return Option::None;
    }

    let x_range_start = (sensor.0 .0 as i32 - x_diff as i32).max(0);
    let x_range_end = (sensor.0 .0 as i32 + x_diff as i32).min(4_000_000);

    Option::Some((x_range_start, x_range_end))
}

fn get_sensor_position(sensor_str: &str) -> Sensor {
    let (x, y) = sensor_str[10..].split_once(", ").unwrap();

    let x = x[2..].parse().unwrap();
    let y = y[2..].parse().unwrap();

    Sensor((x, y))
}

fn get_beacon_position(beacon_str: &str) -> Beacon {
    let (x, y) = beacon_str[22..].split_once(", ").unwrap();

    let x = x[2..].parse().unwrap();
    let y = y[2..].parse().unwrap();

    Beacon((x, y))
}
