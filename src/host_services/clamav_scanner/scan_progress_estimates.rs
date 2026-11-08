use std::time::{Duration, Instant};

use crate::translated_messages;

const ESTIMATE_MINIMUM_FILES: u64 = 50;
const ESTIMATE_MINIMUM_ELAPSED: Duration = Duration::from_secs(3);

#[derive(Debug)]
pub struct RefreshThrottle {
    refresh_interval: Duration,
    last_refresh: Instant,
}

impl RefreshThrottle {
    pub fn starting_at(refresh_interval: Duration, started_at: Instant) -> Self {
        Self {
            refresh_interval,
            last_refresh: started_at
                .checked_sub(refresh_interval)
                .unwrap_or(started_at),
        }
    }

    pub fn allows_refresh(&mut self, checked_at: Instant) -> bool {
        if checked_at.saturating_duration_since(self.last_refresh) < self.refresh_interval {
            return false;
        }
        self.last_refresh = checked_at;
        true
    }
}

pub fn file_progress_text(inspected_file_count: u64, planned_file_count: u64) -> String {
    translated_messages::host::file_progress(planned_file_count, inspected_file_count)
}

pub fn remaining_time_text(
    elapsed_time: Duration,
    inspected_file_count: u64,
    planned_file_count: u64,
) -> Option<String> {
    if inspected_file_count < ESTIMATE_MINIMUM_FILES
        || elapsed_time < ESTIMATE_MINIMUM_ELAPSED
        || inspected_file_count >= planned_file_count
    {
        return None;
    }
    let remaining_file_count = planned_file_count - inspected_file_count;
    let remaining_seconds = (elapsed_time.as_secs_f64() * remaining_file_count as f64
        / inspected_file_count as f64)
        .round() as u64;
    Some(describe_remaining_seconds(remaining_seconds))
}

fn describe_remaining_seconds(remaining_seconds: u64) -> String {
    match remaining_seconds {
        0..60 => {
            let remaining_amount = remaining_seconds.max(1);
            translated_messages::host::count_second_left(remaining_amount)
        }
        60..3600 => {
            let remaining_amount = remaining_seconds.div_ceil(60);
            translated_messages::host::count_minute_left(remaining_amount)
        }
        _long_scan => {
            let remaining_amount = remaining_seconds.div_ceil(3600);
            translated_messages::host::count_hour_left(remaining_amount)
        }
    }
}

pub fn elapsed_duration_text(elapsed_time: Duration) -> String {
    let elapsed_seconds = elapsed_time.as_secs();
    let (hours, minutes, seconds) = (
        elapsed_seconds / 3600,
        elapsed_seconds % 3600 / 60,
        elapsed_seconds % 60,
    );
    match (hours, minutes) {
        (0, 0) => translated_messages::host::seconds_s(seconds),
        (0, _) => translated_messages::host::minutes_min_seconds_s(minutes, seconds),
        _long_scan => translated_messages::host::hours_h_minutes_min(hours, minutes),
    }
}

#[cfg(test)]
mod scan_progress_estimates_tests {
    use super::*;

    #[test]
    fn refreshes_are_spaced_by_the_interval() {
        let started_at = Instant::now();
        let mut refresh_throttle =
            RefreshThrottle::starting_at(Duration::from_millis(100), started_at);
        assert!(refresh_throttle.allows_refresh(started_at));
        assert!(!refresh_throttle.allows_refresh(started_at + Duration::from_millis(40)));
        assert!(refresh_throttle.allows_refresh(started_at + Duration::from_millis(120)));
    }

    #[test]
    fn remaining_time_waits_for_enough_progress() {
        assert_eq!(remaining_time_text(Duration::from_secs(10), 10, 1000), None);
        assert_eq!(remaining_time_text(Duration::from_secs(1), 500, 1000), None);
        assert_eq!(
            remaining_time_text(Duration::from_secs(30), 100, 300),
            Some("About 1 minute left".to_owned())
        );
        assert_eq!(
            remaining_time_text(Duration::from_secs(10), 100, 200),
            Some("About 10 seconds left".to_owned())
        );
        assert_eq!(
            remaining_time_text(Duration::from_secs(3600), 100, 400),
            Some("About 3 hours left".to_owned())
        );
    }

    #[test]
    fn progress_text_counts_done_of_total() {
        assert_eq!(file_progress_text(187, 5000), "187 of 5000 files");
    }

    #[test]
    fn elapsed_time_uses_the_two_largest_units() {
        assert_eq!(elapsed_duration_text(Duration::from_secs(42)), "42 s");
        assert_eq!(
            elapsed_duration_text(Duration::from_secs(193)),
            "3 min 13 s"
        );
        assert_eq!(
            elapsed_duration_text(Duration::from_secs(7500)),
            "2 h 5 min"
        );
    }
}
