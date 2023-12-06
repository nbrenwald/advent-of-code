use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;

fn main() -> io::Result<()> {
// pares each HashMap
// calculate a start, end (start plus range), offset (what we need to add on).
// for each seed, try and find in each map, or return itddlf by looking for a range it is within.
// 50 98 2 => 98,100 (start+range), -48 (start-destination)

let mut whichMap: String = "".to_string();
let mut seed_to_soil: Vec<(i64,i64,i64)> = Vec::new();
let mut soil_to_fertilizer: Vec<(i64,i64,i64)> = Vec::new();
let mut fertilizer_to_water: Vec<(i64,i64,i64)> = Vec::new();
let mut water_to_light: Vec<(i64,i64,i64)> = Vec::new();
let mut light_to_temperature: Vec<(i64,i64,i64)> = Vec::new();
let mut temperature_to_humidity: Vec<(i64,i64,i64)> = Vec::new();
let mut humidity_to_location: Vec<(i64,i64,i64)> = Vec::new();
//let seeds: Vec<i64> = vec![79, 14, 55, 13]; <Down>
let seeds: Vec<i64> = vec![202517468, 131640971, 1553776977, 241828580, 1435322022, 100369067, 2019100043, 153706556, 460203450, 84630899, 3766866638, 114261107, 1809826083, 153144153, 2797169753, 177517156, 2494032210, 235157184, 856311572, 542740109]; 
    for line in read_to_string("sample.txt").unwrap().lines() {
        let line_str = line.to_string();
        println!("{}", line_str);

        let words: Vec<&str> = line_str.split(" ").collect();

        if words.len() == 2 {
            if words[1] == "map:" {
                whichMap = words[0].to_string();
                println!("Parsing map {}", words[0]);
            }
        }
        if words.len() == 3 {
            let start: i64 = words[1].parse().unwrap();
            let range:i64 = words[2].parse().unwrap();
            let end:i64 = start + range;
            let destination:i64 = words[0].parse().unwrap(); 
            let offset:i64 = destination - start;
            if whichMap == "seed-to-soil" {
               seed_to_soil.push((start,end,offset));
            }
            if whichMap == "soil-to-fertilizer" {
               soil_to_fertilizer.push((start,end,offset));
            }
            if whichMap == "fertilizer-to-water" {
               fertilizer_to_water.push((start,end,offset));
            }
            if whichMap == "water-to-light" {
               water_to_light.push((start,end,offset));
            }
            if whichMap == "light-to-temperature" {
               light_to_temperature.push((start,end,offset));
            }
            if whichMap == "temperature-to-humidity" {
               temperature_to_humidity.push((start,end,offset));
            }
            if whichMap == "humidity-to-location" {
               humidity_to_location.push((start,end,offset));
            }
            println!("{:?}", seed_to_soil);


        }

    }

    let mut min_location = std::i64::MAX;
    for seed in seeds.iter() {
      println!("Checking seed {}", seed);
      let soil =  convert(&seed_to_soil, *seed);
      let fertilizer = convert(&soil_to_fertilizer, soil);
      let water = convert(&fertilizer_to_water, fertilizer);
      let light = convert(&water_to_light, water);
      let temperature = convert(&light_to_temperature, light);
      let humidity = convert(&temperature_to_humidity, temperature);
      let location = convert(&humidity_to_location, humidity);

      min_location = cmp::min(min_location, location);
    }
    println!("min location {}", min_location);
Ok(())
}

fn convert(vec: &Vec<(i64,i64,i64)>, i: i64) -> i64 {

    for x in vec.iter() {
        if i>= x.0 && i< x.1 {
            return i+x.2
        } 
    }
    i
}
