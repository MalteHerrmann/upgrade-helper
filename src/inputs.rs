use crate::network::Network;
use chrono::{
    DateTime, Datelike, Duration, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc, Weekday,
};
use inquire::{DateSelect, Select};
use std::path::PathBuf;
use std::{fs, ops::Add, process};

const MONTHS: [&str; 13] = [
    "", "January", "February", "March", "April", "May", "June", "July", "August", "Septemer",
    "October", "November", "December",
];

/// Scans the current folder for existing proposal configurations (stored as JSON)
/// and lets the user choose the desired configuration file to use.
pub fn choose_config() -> Result<PathBuf, String> {
    let current_dir = std::env::current_dir().unwrap();

    // Get all files in the current directory
    let paths = fs::read_dir(&current_dir).unwrap();

    // Filter for JSON files
    let json_files = paths.filter(|path| {
        path.as_ref()
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .ends_with(".json")
    });

    // Collect the file names
    let mut config_files: Vec<String> = Vec::new();
    for file in json_files {
        let file_name = file.unwrap().path().to_str().unwrap().to_string();
        config_files.push(file_name);
    }

    if config_files.is_empty() {
        return Err("No configuration files found in current directory".to_string());
    }

    // Prompt the user to select the configuration file
    let config_file_name = match Select::new("Select configuration file", config_files).prompt() {
        Ok(choice) => choice,
        Err(e) => {
            return Err(format!("Error selecting configuration file: {}", e));
        }
    };

    Ok(current_dir.join(config_file_name))
}

/// Prompts the user to select the network type used.
pub fn get_used_network() -> Result<Network, String> {
    let used_network: Network;

    let network_options = vec!["Local Node", "Testnet", "Mainnet"];

    // Prompt the user to select the network
    let chosen_network = Select::new("Select network", network_options).prompt();

    match chosen_network {
        Ok(choice) => match choice {
            "Local Node" => used_network = Network::LocalNode,
            "Testnet" => used_network = Network::Testnet,
            "Mainnet" => used_network = Network::Mainnet,
            &_ => {
                return Err(format!("Invalid network selected: {:?}", choice));
            }
        },
        Err(e) => {
            return Err(format!("Error selecting network: {}", e.to_string()));
        }
    }

    Ok(used_network)
}

/// Prompts the user to input the target version to upgrade to.
pub fn get_text(prompt: &str) -> String {
    let target_version: String;
    // Prompt the user to input the desired target version
    let result = inquire::Text::new(prompt).prompt();
    match result {
        Ok(version) => {
            target_version = version;
        }
        Err(e) => {
            println!("Error selecting target version: {}", e);
            process::exit(1);
        }
    }

    target_version
}

/// Prompts the user to input the date for the planned upgrade.
/// The date is calculated based on the current time and the voting period duration.
pub fn get_upgrade_date(
    voting_period: Duration,
    utc_time: DateTime<Utc>,
) -> Result<DateTime<Utc>, String> {
    let default_date = calculate_planned_date(voting_period, utc_time);

    // Prompt the user to input the desired upgrade date
    let result = DateSelect::new("Select date for the planned upgrade")
        .with_min_date(utc_time.date_naive())
        .with_default(default_date.date_naive())
        .with_week_start(Weekday::Mon)
        .prompt();
    match result {
        Ok(date) => {
            let time = NaiveTime::from_hms_opt(16, 0, 0).unwrap();
            let planned_naive_date_time = NaiveDateTime::new(date, time);
            Ok(Utc.from_local_datetime(&planned_naive_date_time).unwrap())
        }
        Err(e) => Err(format!("Error selecting planned date: {}", e.to_string())),
    }
}

/// Calculates the date for the planned upgrade given the current time and the voting period duration.
/// Per default, 4 pm UTC is used as a reference time.
/// If the passed UTC time is after 2 pm UTC, the planned date will be shifted to the next day.
fn calculate_planned_date(voting_period: Duration, utc_time: DateTime<Utc>) -> DateTime<Utc> {
    let mut end_of_voting = utc_time.add(voting_period);

    // NOTE: if using the tool after 2pm UTC or the end of voting would be at or after 2 PM, the upgrade should happen on the next day
    if utc_time.hour() > 14 || end_of_voting.hour() >= 16 {
        end_of_voting = end_of_voting.add(Duration::days(1));
    }

    // NOTE: we don't want to upgrade on a weekend, so we shift the upgrade to the next monday
    if end_of_voting.weekday() == Weekday::Sat {
        end_of_voting = end_of_voting.add(Duration::days(2));
    } else if end_of_voting.weekday() == Weekday::Sun {
        end_of_voting = end_of_voting.add(Duration::days(1));
    }

    Utc.with_ymd_and_hms(
        end_of_voting.year(),
        end_of_voting.month(),
        end_of_voting.day(),
        16,
        0,
        0,
    )
    .unwrap()
}

/// Checks if the passed upgrade time is valid.
/// The upgrade time cannot be on a weekend.
pub fn is_valid_upgrade_time(upgrade_time: DateTime<Utc>) -> bool {
    if upgrade_time.weekday() == Weekday::Sat || upgrade_time.weekday() == Weekday::Sun {
        return false;
    }

    true
}

/// Returns a string representation of the upgrade time.
pub fn get_time_string(time: DateTime<Utc>) -> String {
    let (is_pm, hour) = time.hour12();
    format!(
        "{}{} {} on {}., {} {}., {}",
        hour,
        if is_pm { "PM" } else { "AM" },
        time.timezone(),
        time.weekday(),
        MONTHS[time.month() as usize],
        time.day(),
        time.year(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Duration, Utc};
    use rstest::{fixture, rstest};

    #[fixture]
    fn monday_morning() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 10, 23, 11, 0, 0).unwrap()
    }

    #[fixture]
    fn monday_evening() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 10, 23, 20, 0, 0).unwrap()
    }

    #[fixture]
    fn friday_morning() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 10, 27, 11, 0, 0).unwrap()
    }

    #[fixture]
    fn testnet_voting_period() -> Duration {
        Duration::hours(12)
    }

    #[fixture]
    fn mainnet_voting_period() -> Duration {
        Duration::hours(120)
    }

    #[rstest]
    fn test_calculate_planned_date_monday_morning_testnet(
        monday_morning: DateTime<Utc>,
        testnet_voting_period: Duration,
    ) {
        assert_eq!(
            calculate_planned_date(testnet_voting_period, monday_morning),
            Utc.with_ymd_and_hms(2023, 10, 24, 16, 0, 0).unwrap(),
            "expected different date for testnet upgrade when calling on monday morning",
        );
    }

    #[rstest]
    fn test_calculate_planned_date_monday_morning_mainnet(
        monday_morning: DateTime<Utc>,
        mainnet_voting_period: Duration,
    ) {
        assert_eq!(
            calculate_planned_date(mainnet_voting_period, monday_morning),
            // NOTE: the upgrade should happen on the next monday 4PM, not on saturday which would be t+120h
            Utc.with_ymd_and_hms(2023, 10, 30, 16, 0, 0).unwrap(),
            "expected different date for mainnet upgrade when calling on monday morning",
        );
    }

    #[rstest]
    fn test_calculate_planned_date_monday_evening_testnet(
        monday_evening: DateTime<Utc>,
        testnet_voting_period: Duration,
    ) {
        assert_eq!(
            calculate_planned_date(testnet_voting_period, monday_evening),
            Utc.with_ymd_and_hms(2023, 10, 25, 16, 0, 0).unwrap(),
            "expected different date for testnet upgrade when calling on monday evening",
        );
    }

    #[rstest]
    fn test_calculate_planned_date_monday_evening_mainnet(
        monday_evening: DateTime<Utc>,
        mainnet_voting_period: Duration,
    ) {
        assert_eq!(
            calculate_planned_date(mainnet_voting_period, monday_evening),
            // NOTE: the upgrade should happen on the next monday 4PM, not on saturday which would be t+120h
            Utc.with_ymd_and_hms(2023, 10, 30, 16, 0, 0).unwrap(),
            "expected different date for mainnet upgrade when calling on monday evening",
        );
    }

    #[rstest]
    fn test_calculate_planned_date_friday_morning_testnet(
        friday_morning: DateTime<Utc>,
        testnet_voting_period: Duration,
    ) {
        assert_eq!(
            calculate_planned_date(testnet_voting_period, friday_morning),
            // NOTE: the upgrade should happen on the next monday 4PM, not on saturday which would be t+12h
            Utc.with_ymd_and_hms(2023, 10, 30, 16, 0, 0).unwrap(),
            "expected different date for testnet upgrade when calling on thursday morning",
        );
    }

    #[rstest]
    fn test_calculate_planned_date_friday_morning_mainnet(
        friday_morning: DateTime<Utc>,
        mainnet_voting_period: Duration,
    ) {
        assert_eq!(
            calculate_planned_date(mainnet_voting_period, friday_morning),
            // NOTE: the upgrade should happen on the next wednesday 4PM
            Utc.with_ymd_and_hms(2023, 11, 1, 16, 0, 0).unwrap(),
            "expected different date for mainnet upgrade when calling on thursday morning",
        );
    }

    #[test]
    fn test_get_time_string_october_morning() {
        let time = Utc.with_ymd_and_hms(2023, 10, 23, 4, 0, 0).unwrap();
        assert_eq!(
            get_time_string(time),
            "4AM UTC on Mon., October 23., 2023",
            "expected different time string",
        );
    }

    #[test]
    fn test_get_time_string_february_evening() {
        let time = Utc.with_ymd_and_hms(2023, 2, 1, 16, 0, 0).unwrap();
        assert_eq!(
            get_time_string(time),
            "4PM UTC on Wed., February 1., 2023",
            "expected different time string",
        );
    }
}
