use std::{env, fs, process};

mod common;
mod day01;
mod day02;

use common::Day;

fn main() {
    let mut challenges: Vec<Day> = Vec::new();
    challenges.push(Day {
        part_1: day01::part_1,
        part_2: day01::part_2,
        day_number: 1,
    });

    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!("Error: args is empty.");
        process::exit(exitcode::OSERR);
    } else if args.len() == 1 {
        println!("Running all the days:");
        challenges.iter().for_each(|day| {
            println!(
                "Day {} - Part 1: {}",
                day.day_number,
                (day.part_1)(
                    &fs::read_to_string(format!("inputs/{}", day.day_number))
                        .expect("Error reading input file.")
                )
            );
            println!(
                "Day {} - Part 2: {}",
                day.day_number,
                (day.part_2)(
                    &fs::read_to_string(format!("inputs/{}", day.day_number))
                        .expect("Error reading input file.")
                )
            );
        });
    } else if args.len() == 2 {
        if let Ok(day_number) = args[1].parse::<usize>() {
            println!("Running day {}", day_number);
            challenges.retain(|d| d.day_number == day_number);

            challenges.iter().for_each(|d| {
                println!(
                    "Day {} - Part 1: {}",
                    day_number,
                    (d.part_1)(
                        &fs::read_to_string(format!("inputs/{}", day_number))
                            .expect("Error reading input file.")
                    )
                );
                println!(
                    "Day {} - Part 2: {}",
                    day_number,
                    (d.part_2)(
                        &fs::read_to_string(format!("inputs/{}", day_number))
                            .expect("Error reading input file.")
                    )
                );
            });
        } else {
            eprintln!("Error: '{}' is not a valid day number.", args[1]);
            process::exit(exitcode::USAGE);
        }
    } else {
        println!("Day specification [] is not supported yet.");
        process::exit(exitcode::USAGE);
    }

    process::exit(exitcode::OK);
}
