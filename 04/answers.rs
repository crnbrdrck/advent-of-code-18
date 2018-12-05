use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Given a list of records found about guard sleeping patterns, order them by the time and then figure out which guard was asleep at a given minute the most times
    // Print out the value of the guard id multiplied by the minute at which they were asleep most times
    let test = "[1518-11-01 00:00] Guard #10 begins shift
    [1518-11-01 00:05] falls asleep
    [1518-11-01 00:25] wakes up
    [1518-11-01 00:30] falls asleep
    [1518-11-01 00:55] wakes up
    [1518-11-01 23:58] Guard #99 begins shift
    [1518-11-02 00:40] falls asleep
    [1518-11-02 00:50] wakes up
    [1518-11-03 00:05] Guard #10 begins shift
    [1518-11-03 00:24] falls asleep
    [1518-11-03 00:29] wakes up
    [1518-11-04 00:02] Guard #99 begins shift
    [1518-11-04 00:36] falls asleep
    [1518-11-04 00:46] wakes up
    [1518-11-05 00:03] Guard #99 begins shift
    [1518-11-05 00:45] falls asleep
    [1518-11-05 00:55] wakes up";
    let (test_ans1, test_ans2) = calc(test);
    println!("({}, {}) should be (240, 4455)", test_ans1, test_ans2);

    // Now do with the actual file
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    // Trim the file to avoid having that error again
    contents = contents.trim().to_string();
    let (ans1, ans2) = calc(&contents);
    println!("Puzzle Answers: {}, {}", ans1, ans2);
}

fn calc(input: &str) -> (u32, u32) {
    // First, split input on newlines and sort by the time in the square brackets
    let mut lines: Vec<&str> = input.split("\n").map(|s| s.trim()).collect();
    lines.sort_unstable_by(|a, b| date_time_sort(a, b));

    // Now we need to iterate through the lines and calculate for each guard the minutes at which they were asleep and how many nights
    // NOTE: All asleep / awake times are during midnight hour so we only need the minute segment
    let mut sleep_times: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut i = 0;
    while i < lines.len() {
        // Assert that the current line contains the word "Guard" (for my own sanity)
        assert!(lines[i].contains(&"Guard"));
        let mut j = i;
        while (j + 1) < lines.len() && !lines[j+1].contains("Guard") {
            j += 1;
        }
        // We have details for a guard from line i to line j so parse the times for that guard and then update i
        parse_times(lines[i..=j].to_vec(), &mut sleep_times);
        i = j + 1;
    }

    // Now we search through the hashes to find the guard with the most time spent asleep, and get the minute they were asleep the most at
    let mut max_id = 0;
    let mut max_sleep_count = 0;

    for (id, times) in &sleep_times {
        // Sum up the amount of minutes the guard was asleep for
        let sleep_count = times.values().into_iter().sum();
        if sleep_count > max_sleep_count {
            max_id = *id;
            max_sleep_count = sleep_count;
        }
    }

    // Having found the guard who was asleep most often, get the minute they were asleep most at
    let mut max_minute = 0;
    max_sleep_count = 0;
    for (min, count) in &sleep_times[&max_id] {
        if count > &max_sleep_count {
            max_sleep_count = *count;
            max_minute = *min;
        }
    }

    // The funny thing about part 2 is I did that first :eyes:
    let mut pt_2_max_id = 0;
    let mut pt_2_max_minute = 0;
    let mut pt_2_max_count= 0;

    for (id, times) in &sleep_times {
        for (min, sleep_count) in times {
            if sleep_count > &pt_2_max_count {
                pt_2_max_count = *sleep_count;
                pt_2_max_minute = *min;
                pt_2_max_id = *id;
            }
        }
    }

    return ((max_id * max_minute) as u32, (pt_2_max_id * pt_2_max_minute) as u32);
}

fn parse_times(lines: Vec<&str>, sleep_times: &mut HashMap<u32, HashMap<u32, u32>>) {
    // Parse the lines in the vector which contains sleeping data on a Guard, and update the sleep times hash
    // The first line will contain the Guard's ID if you split by spaces, grab element 3 and remove the #
    let id = lines[0].split(" ").nth(3).unwrap().replace("#", "").parse::<u32>().unwrap();
    if !sleep_times.contains_key(&id) {
        sleep_times.insert(id, HashMap::new());
    }

    // Iterate through the other elements, every pair should be falls asleep and wakes up, so get the difference and update the hash accordingly
    for i in (1..lines.len()).step_by(2) {
        // lines[i] is sleep, lines[i + 1] is wake (but assert so I don't fuck it up)
        assert!(lines[i].contains("asleep"));
        assert!(lines[i + 1].contains("wakes"));

        // Get the minute difference between the two lines
        let sleep_min = lines[i].split(" ").nth(1).unwrap()[3..5].parse::<u32>().unwrap();
        let wake_min = lines[i + 1].split(" ").nth(1).unwrap()[3..5].parse::<u32>().unwrap();

        for min in sleep_min..wake_min {
            if !sleep_times[&id].contains_key(&min) {
                sleep_times.get_mut(&id).unwrap().insert(min, 1);
            }
            else {
                *sleep_times.get_mut(&id).unwrap().get_mut(&min).unwrap() += 1;
            }
        }
    }
}

fn date_time_sort(a: &str, b: &str) -> Ordering {
    // Given two strings, sort them by the date and time stored within the square brackets
    // Get the strings inside the square brackets, parse the datetimes and compare them
    // /shrug Do this with no external crates because why not
    let a_date_str = &a.split("]").next().unwrap()[1..];
    let b_date_str = &b.split("]").next().unwrap()[1..];

    // Gonna do this with no external crates -> Compare Years, then months, days, hours and finally minutes

    // Years
    let a_year = a_date_str[..4].parse::<u32>().expect("year was not an int");
    let b_year = b_date_str[..4].parse::<u32>().expect("year was not an int");
    if a_year != b_year {
        return a_year.cmp(&b_year);
    }

    // Months (if years are equal)
    let a_month = a_date_str[5..7].parse::<u32>().expect("month was not an int");
    let b_month = b_date_str[5..7].parse::<u32>().expect("month was not an int");
    if a_month != b_month {
        return a_month.cmp(&b_month);
    }

    // Days (if months are equal)
    let a_day = a_date_str[8..10].parse::<u32>().expect("day was not an int");
    let b_day = b_date_str[8..10].parse::<u32>().expect("day was not an int");
    if a_day != b_day {
        return a_day.cmp(&b_day);
    }

    // Hours (if days are equal)
    let a_hour = a_date_str[11..13].parse::<u32>().expect("hour was not an int");
    let b_hour = b_date_str[11..13].parse::<u32>().expect("hour was not an int");
    if a_hour != b_hour {
        return a_hour.cmp(&b_hour);
    }

    // Minutes (if hours are equal)
    let a_min = a_date_str[14..].parse::<u32>().expect("min was not an int");
    let b_min = b_date_str[14..].parse::<u32>().expect("min was not an int");
    // If these are equal then the datetime strings are entirely equal so it's okay to return equal here
    return a_min.cmp(&b_min);
}
