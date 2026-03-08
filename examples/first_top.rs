use std::ops::Range;

use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use chrono_tz::Tz;
use csv_db::Database;
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

const DATABASE_PATH: &str = ".";
const DATABASE_COLLECTION: &str = "first_results";
const MAX_RESULTS: usize = 10;
const CUTOFF_US: i64 = 1000000;
const RAND_OPEN_HOUR: Range<u32> = 5..12;
const RAND_OPEN_MIN: Range<u32> = 0..59;

#[derive(Debug, Deserialize, Serialize)]
struct FirstResult {
    nick: String,
    channel: String,
    datetime: DateTime<Utc>,
    timezone: String,
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let db = Database::new(DATABASE_PATH, None);

    let first_results = db
        .find(DATABASE_COLLECTION, |_: &FirstResult| true)
        .await
        .map_err(|_| "Could not find data")?;

    let rank = rank(&first_results, MAX_RESULTS)?;

    for (pos, (date, x)) in rank.iter().enumerate() {
        println!(
            "{}. {:?} {} {} ms",
            pos + 1,
            date,
            x.get(0).ok_or("Could not get data")?.1,
            x.get(0).ok_or("Could not get data")?.0 / 1000
        );
    }

    Ok(())
}

/// Compute the top MAX_RESULTS earliest !1st submissions for each nick.
///
/// 1. Group entries by date (each different day of the year is a key for the group).
/// 2. For each date:
///    - Compute each player's "delta" (how close they were to the opening time).
///    - Keep only deltas of interest (positive and below cutoff).
///    - Pick the earliest valid one for that day.
/// 3. Globally sort all days by delta time (earliest !1st).
/// 4. Ensure unique entries by nick.
/// 5. Return the top MAX_RESULTS.
fn rank(
    first_results: &[FirstResult],
    max_results: usize,
) -> Result<Vec<(NaiveDate, Vec<(i64, String)>)>, &'static str> {
    // Group entries by date (each different day of the year is a key for the group).
    // Chain date_naive() to get rid of the time and return a date as key to chunk_by.
    let groups = first_results.iter().chunk_by(|r| {
        let tz: Tz = r
            .timezone
            .parse()
            .expect("Timezone should be in Continent/Capital format");

        r.datetime.with_timezone(&tz).date_naive()
    });

    // For each group (one per date), determine the best player and time delta.
    // The outer filter_map itereates through each date and selects where the best delta is between 0 and CUTOFF_US.
    // Then sorts the groups by the lowest delta, makes results unique by nick and takes max_results.
    let rank: Vec<(NaiveDate, Vec<(i64, String)>)> = groups
        .into_iter()
        .filter_map(|(day, group)| {
            // The inner filter_map calculates for each date the deltas, sorts by lowest and takes only one.
            // filter_map maps to Vec<(i64, String)>, a vector of tuples representing delta and nick.
            let delta_results: Vec<(i64, String)> = group
                .filter_map(|r| delta(day, r).ok())
                .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
                .take(1)
                .collect();
            // End of inner filter_map.

            if let Some((micros, _nick)) = delta_results.get(0) {
                if *micros > 0 && *micros <= CUTOFF_US {
                    return Some((day, delta_results));
                }
            }
            None
        })
        .sorted_by(|a, b| Ord::cmp(&a.1[0].0, &b.1[0].0))
        .unique_by(|r| r.1[0].1.clone())
        .take(max_results)
        .collect();
    // End of outer filter_map.

    Ok(rank)
}

/// Calculate the delta in microseconds between the time when the user played !1st and the opening time.
fn delta(day: NaiveDate, r: &FirstResult) -> Result<(i64, String), &'static str> {
    // Convert the player time to the player timezone.
    let tz: Tz = r.timezone.parse().map_err(|_| "Bad timezone")?;
    let local_player_time = r.datetime.with_timezone(&tz);

    let month_day = day.day();

    // Use the same seed as the bot uses (day of the month) to get the same opening hour.
    let mut rng = StdRng::seed_from_u64(month_day as u64);
    let open_hour = rng.random_range(RAND_OPEN_HOUR);

    // Use the same seed as the bot uses (day of the month) to get the same opening minute.
    let mut rng = StdRng::seed_from_u64(month_day as u64);
    let open_min = rng.random_range(RAND_OPEN_MIN);

    // To build the local opening time we use a little trick.
    // We already calculated the opening hour and minute above, but we are working with DateTime.
    // So we make the local opening time equal to the local player time to get the correct date.
    // Then we simply set the opening hour and minute with the values above.
    // Finally we zero out the other components of the DateTime.
    let local_opening_time = local_player_time
        .with_hour(open_hour)
        .and_then(|t| t.with_minute(open_min))
        .and_then(|t| t.with_second(0))
        .and_then(|t| t.with_nanosecond(0))
        .ok_or("Bad time format")?;

    // Finally subtract the local opening time from the local player time.
    let delta = local_player_time - local_opening_time;

    Ok((
        delta
            .num_microseconds()
            .ok_or("Could not get microseconds")?,
        r.nick.clone(),
    ))
}
