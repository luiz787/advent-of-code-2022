use std::collections::HashSet;


#[derive(Eq, PartialEq, Hash, Debug)]
struct Beacon((i32, i32));

#[derive(Eq, PartialEq, Hash, Debug)]
struct Sensor((i32, i32));

impl Sensor {
    fn distance(&self, beacon: &Beacon) -> u32 {
        let x_distance = self.0.0.abs_diff(beacon.0.0);
        let y_distance = self.0.1.abs_diff(beacon.0.1);

        (x_distance + y_distance) as u32
    }
}

fn main() {
    let input = include_str!("../input");
    let y = 2_000_000;

    let pairs: Vec<_> = input
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(sensor, beacon)| (get_sensor_position(sensor), get_beacon_position(beacon)))
        .collect();

    let beacons: HashSet<_> = pairs
        .iter()
        .map(|(_, beacon)| beacon)
        .collect();
    let sensors: HashSet<_> = pairs
        .iter()
        .map(|(sensor, _)| sensor)
        .collect();

    let result: HashSet<_> = pairs
        .iter()
        .map(|(sensor, beacon)| (sensor, sensor.distance(beacon)))
        .map(|(sensor, distance)| definitely_does_not_contain_beacon(&sensors, &beacons, sensor, distance, y))
        .reduce(|item, acc| {
            let mut union = HashSet::new();

            union.extend(acc.iter());
            union.extend(item.iter());

            union
        }).unwrap();

    println!("{}", result.len());
}

fn definitely_does_not_contain_beacon(sensors: &HashSet<&Sensor>, beacons: &HashSet<&Beacon>, sensor: &Sensor, distance: u32, y: i32) -> HashSet<(i32, i32)> {
    let y_diff = y.abs_diff(sensor.0.1);
    let x_diff = distance as i32 - y_diff as i32;

    if x_diff <= 0 {
        return HashSet::new();
    }

    let x_range_start = sensor.0.0 as i32 - x_diff as i32;
    let x_range_end = sensor.0.0 as i32 + x_diff as i32;
    
    let mut set = HashSet::new();
    for x in x_range_start..=x_range_end { 
        let pos = (x, y);
        if !sensors.contains(&Sensor(pos)) && !beacons.contains(&Beacon(pos)) {
            set.insert(pos);
        }
    }

    set
}

fn get_sensor_position(sensor_str: &str) -> Sensor {
    let (x, y) = sensor_str[10..]
        .split_once(", ")
        .unwrap();
    
    let x = x[2..].parse().unwrap();
    let y = y[2..].parse().unwrap();

    Sensor((x, y))
}

fn get_beacon_position(beacon_str: &str) -> Beacon {
    let (x, y) = beacon_str[22..]
        .split_once(", ")
        .unwrap();
    
    let x = x[2..].parse().unwrap();
    let y = y[2..].parse().unwrap();

    Beacon((x, y))
}
