use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let mut passports: Vec<Vec<String>> = Vec::new();
        let mut passport: Vec<String> = Vec::new();
        for line in f.lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    passports.push(passport);
                    passport = Vec::new();
                } else {
                    for field in line.split_whitespace() {
                        passport.push(field.to_string());
                    }
                }
            }
        }
        if !passport.is_empty() {
            passports.push(passport);
        }
        let valid_fields_count = passports
            .iter()
            .filter(|passport| contains_all_fields(passport))
            .count();
        println!("{}", valid_fields_count);
        let valid_data_count = passports
            .iter()
            .filter(|passport| contains_valid_data(passport).is_ok())
            .count();
        println!("{}", valid_data_count);
    }
}

use std::collections::HashSet;
fn contains_all_fields(passport: &Vec<String>) -> bool {
    let mut set = HashSet::new();
    for field in passport {
        let split = field.split(':').collect::<Vec<&str>>()[0];
        set.insert(split.to_string());
    }
    let required_fields = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
        // "cid",
    ];
    for req in required_fields {
        if !set.contains(req) {
            return false;
        }
    }
    true
}

fn validate_year(
    passport: &HashMap<String, String>,
    field: &str,
    min: i32,
    max: i32,
) -> Result<(), ()> {
    let year = passport.get(field).ok_or(())?;
    let year = year.parse::<i32>().ok().ok_or(())?;
    Some(()).filter(|_| year >= min && year <= max).ok_or(())
}

fn validate_height(passport: &HashMap<String, String>) -> Result<(), ()> {
    let height = passport.get("hgt").ok_or(())?;
    let mut height = height.to_string();
    let unit_start = height.find(|c: char| !c.is_digit(10)).ok_or(())?;
    let units = height.split_off(unit_start);
    let height = height.parse::<i32>().ok().ok_or(())?;
    Some(())
        .filter(|_| match &units[..] {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false,
        })
        .ok_or(())
}

fn validate_hair_color(passport: &HashMap<String, String>) -> Result<(), ()> {
    let hair_color = passport.get("hcl").ok_or(())?;
    Some(())
        .filter(|_| {
            hair_color.len() == 7
                && hair_color.chars().enumerate().all(|(i, c)| {
                    if i == 0 {
                        c == '#'
                    } else {
                        c.is_digit(16)
                    }
                })
        })
        .ok_or(())
}

fn validate_eye_color(passport: &HashMap<String, String>) -> Result<(), ()> {
    let eye_color = passport.get("ecl").ok_or(())?;
    let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    Some(())
        .filter(|_| valid_colors.into_iter().any(|color| eye_color == color))
        .ok_or(())
}

fn validate_passport_id(passport: &HashMap<String, String>) -> Result<(), ()> {
    let id = passport.get("pid").ok_or(())?;
    Some(())
        .filter(|_| id.len() == 9 && id.chars().all(|c| c.is_digit(10)))
        .ok_or(())
}

use std::collections::HashMap;
fn contains_valid_data(fields: &Vec<String>) -> Result<(), ()> {
    let mut passport: HashMap<String, String> = HashMap::new();
    for field in fields {
        let split: Vec<&str> = field.split(':').collect();
        passport.insert(split[0].to_string(), split[1].to_string());
    }
    validate_year(&passport, "byr", 1920, 2002)?;
    validate_year(&passport, "iyr", 2010, 2020)?;
    validate_year(&passport, "eyr", 2020, 2030)?;
    validate_height(&passport)?;
    validate_hair_color(&passport)?;
    validate_eye_color(&passport)?;
    validate_passport_id(&passport)?;
    Ok(())
}
