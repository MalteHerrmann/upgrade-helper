use crate::network::Network;
use chrono::{Datelike, Duration, Timelike, TimeZone};
use inquire::{
    Select,
    DateSelect,
};
use std::process;

/// Prompts the user to select the network type used.
pub fn get_used_network() -> Network {
    let used_network: Network;

    let network_options = vec![
        "Local Node",
        "Testnet",
        "Mainnet",
    ];

    // Prompt the user to select the network
    let chosen_network = Select::new(
        "Select network", network_options,
    ).prompt();

    match chosen_network {
        Ok(choice) => {
            match choice {
                "Local Node" => used_network = Network::LocalNode,
                "Testnet" => used_network = Network::Testnet,
                "Mainnet" => used_network = Network::Mainnet,
                &_ => {
                    println!("Invalid network selected: {:?}", choice);
                    process::exit(1); // TODO: return error here instead of exiting in here
                },
            }
        }
        Err(e) => { 
            println!("Error selecting network: {}", e); 
            process::exit(1);
        }
    }

    used_network
}

/// Prompts the user to input the target version to upgrade to.
pub fn get_text(prompt: &str) -> String {
    let target_version: String;
    // Prompt the user to input the desired target version
    let result = inquire::Text::new(prompt)
        .prompt();
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
pub fn get_upgrade_date(voting_period: Duration, utc_time: chrono::DateTime<chrono::Utc>) -> String {
    let default_date = calculate_planned_date(voting_period, utc_time);

    let planned_date: String;
    // Prompt the user to input the desired upgrade date
    let result = DateSelect::new("Select date for the planned upgrade")
        .with_min_date(utc_time.date_naive())
        .with_default(default_date.date_naive())
        .prompt();
    match result {
        Ok(date) => {
            planned_date = date.to_string();
        }
        Err(e) => {
            println!("Error selecting planned date: {}", e);
            process::exit(1);
        }
    }

    planned_date
}

/// Calculates the date for the planned upgrade given the current time and the voting period duration.
/// Per default, 4 pm UTC is used as a reference time.
/// If the passed UTC time is after 2 pm UTC, the planned date will be shifted to the next day.
fn calculate_planned_date(voting_period: Duration, utc_time: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<chrono::Utc> {
    let mut end_of_voting = utc_time.add(voting_period);

    // NOTE: if using the tool after 2pm UTC, the upgrade should happen on the next day
    if utc_time.hour() > 14 {
        end_of_voting = end_of_voting.add(Duration::days(1));
    }

    chrono::Utc.with_ymd_and_hms(
        end_of_voting.year(),
        end_of_voting.month(),
        end_of_voting.day(),
        16,
        0,
        0,
    ).unwrap()
}

#[cfg(test)]
mod tests{
    use super::*;
    use rstest::{
        fixture,
        rstest
    };
    use chrono::{Duration, DateTime, Utc};

    #[fixture]
    fn monday_morning() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 10, 23, 11, 0, 0).unwrap()
    }

    #[fixture]
    fn monday_evening() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 10, 23, 20, 0, 0).unwrap()
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
            Utc.with_ymd_and_hms(2023, 10, 29, 16, 0, 0).unwrap(),
            "expected different date for mainnet upgrade when calling on monday evening",
        );
    }
}