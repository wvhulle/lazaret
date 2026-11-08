use std::path::Path;

use bytesize::ByteSize;
use relm4::gtk::glib;

use crate::{
    host_services::clamav_scanner::scanner_output_parser::sanitize_scanner_line,
    scans::removable_drives_page::{
        drive_mount_checks::{
            DeviceAutoScanSettings, DriveScanDecision, DriveSizeLimit, DriveSkipReason,
        },
        removable_drives_state::DeviceScanStatus,
    },
    shared_widgets::scan_runs::scan_results_section::scan_results_state::ScanResultsStatus,
    translated_messages,
};

pub fn drive_size_text(drive_size: ByteSize) -> String {
    glib::format_size(drive_size.as_u64()).into()
}

pub fn shown_drive_name(drive_name: &str, mount_path: &Path) -> String {
    let fallback_name = mount_path
        .file_name()
        .unwrap_or(mount_path.as_os_str())
        .to_string_lossy();
    sanitize_scanner_line(if drive_name.trim().is_empty() {
        &fallback_name
    } else {
        drive_name
    })
}

pub fn auto_scan_state_text(auto_scan_settings: DeviceAutoScanSettings) -> String {
    match (
        auto_scan_settings.auto_scan_enabled,
        auto_scan_settings.drive_size_limit(),
    ) {
        (false, _) => translated_messages::scans::off_label(),
        (true, DriveSizeLimit::NoSizeLimit) => translated_messages::scans::every_drive(),
        (true, DriveSizeLimit::LimitedTo(size_limit)) => {
            translated_messages::scans::drives_up_size(drive_size_text(size_limit))
        }
    }
}

fn skip_reason_text(skip_reason: DriveSkipReason) -> String {
    match skip_reason {
        DriveSkipReason::SkippedLargerThanLimit {
            drive_size,
            size_limit,
        } => translated_messages::scans::skipped_larger_than_limit(
            drive_size_text(drive_size),
            drive_size_text(size_limit),
        ),
        DriveSkipReason::SkippedOnBattery => translated_messages::scans::skipped_battery(),
        DriveSkipReason::SkippedAutoScanOff => translated_messages::scans::skipped_auto_scan_off(),
        DriveSkipReason::SkippedWhileScanning => {
            translated_messages::scans::skipped_while_scanning()
        }
        DriveSkipReason::SizeUnreadable => translated_messages::scans::size_unreadable(),
    }
}

pub fn drive_decision_text(drive_decision: DriveScanDecision) -> String {
    match drive_decision {
        DriveScanDecision::DriveSettling => translated_messages::scans::drive_settling(),
        DriveScanDecision::DriveScanning => translated_messages::scans::drive_scanning(),
        DriveScanDecision::ScannedDrive { scan_result } => match scan_result {
            ScanResultsStatus::ThreatsDetected => translated_messages::scans::threats_detected(),
            ScanResultsStatus::NoThreatsDetected => {
                translated_messages::scans::no_threats_detected()
            }
            ScanResultsStatus::FinishedWithProblems => {
                translated_messages::scans::finished_problems()
            }
            ScanResultsStatus::ScanCancelled => translated_messages::scans::scan_cancelled(),
        },
        DriveScanDecision::DriveSkipped(skip_reason) => skip_reason_text(skip_reason),
    }
}

pub const fn drive_decision_icon(drive_decision: DriveScanDecision) -> &'static str {
    match drive_decision {
        DriveScanDecision::DriveSettling | DriveScanDecision::DriveScanning => {
            "content-loading-symbolic"
        }
        DriveScanDecision::ScannedDrive {
            scan_result: ScanResultsStatus::NoThreatsDetected,
        } => "object-select-symbolic",
        DriveScanDecision::ScannedDrive {
            scan_result:
                ScanResultsStatus::ThreatsDetected | ScanResultsStatus::FinishedWithProblems,
        } => "dialog-warning-symbolic",
        DriveScanDecision::ScannedDrive {
            scan_result: ScanResultsStatus::ScanCancelled,
        }
        | DriveScanDecision::DriveSkipped(_) => "action-unavailable-symbolic",
    }
}

pub fn device_scan_title(device_scan_status: &DeviceScanStatus) -> String {
    match device_scan_status {
        DeviceScanStatus::DeviceScanRunning { drive_name } => {
            translated_messages::scans::device_scan_running(drive_name)
        }
        DeviceScanStatus::NoDeviceScanYet | DeviceScanStatus::DeviceScanEnded { .. } => {
            translated_messages::scans::drive_scan()
        }
    }
}

#[cfg(test)]
mod removable_drives_presentation_tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn each_skip_reason_says_why_the_drive_was_not_scanned() {
        assert_eq!(
            drive_decision_text(DriveScanDecision::DriveSkipped(
                DriveSkipReason::SkippedLargerThanLimit {
                    drive_size: ByteSize::gb(64),
                    size_limit: ByteSize::gb(32),
                }
            )),
            "Not scanned: 64.0\u{a0}GB is larger than the 32.0\u{a0}GB limit"
        );
        assert_eq!(
            drive_decision_text(DriveScanDecision::DriveSkipped(
                DriveSkipReason::SizeUnreadable
            )),
            "Not scanned: the size of the drive could not be read"
        );
        assert_eq!(
            drive_decision_text(DriveScanDecision::ScannedDrive {
                scan_result: ScanResultsStatus::ThreatsDetected
            }),
            "Scanned, threats found"
        );
    }

    #[test]
    fn the_auto_scan_state_names_the_size_limit() {
        let enabled_settings = DeviceAutoScanSettings {
            auto_scan_enabled: true,
            maximum_gigabytes: 32,
            skip_on_battery: true,
        };
        assert_eq!(
            auto_scan_state_text(enabled_settings),
            "Scans drives up to 32.0\u{a0}GB"
        );
        assert_eq!(
            auto_scan_state_text(DeviceAutoScanSettings {
                maximum_gigabytes: 0,
                ..enabled_settings
            }),
            "Scans every drive"
        );
        assert_eq!(
            auto_scan_state_text(DeviceAutoScanSettings {
                auto_scan_enabled: false,
                ..enabled_settings
            }),
            "Off"
        );
    }

    #[test]
    fn a_drive_without_a_name_is_shown_by_its_mount_folder() {
        let mount_path = PathBuf::from("/run/media/user/3A1F-0C2B");
        assert_eq!(shown_drive_name("", &mount_path), "3A1F-0C2B");
        assert_eq!(shown_drive_name("USB STICK", &mount_path), "USB STICK");
    }
}
