use crate::{
    host_services::clamav_scanner::scanner_exit_classification::AbnormalScannerExit,
    translated_messages,
};

pub fn abnormal_scanner_exit_text(abnormal_exit: &AbnormalScannerExit) -> String {
    match abnormal_exit {
        AbnormalScannerExit::ScannerError { exit_code } => {
            translated_messages::widgets::scanner_stopped_error_code(exit_code)
        }
        AbnormalScannerExit::KilledBySignal { signal_number } => {
            translated_messages::widgets::scanner_stopped_signal(signal_number)
        }
        AbnormalScannerExit::UnrecognizedWaitStatus { raw_wait_status } => {
            translated_messages::widgets::scanner_ended_way_lazaret(raw_wait_status)
        }
        AbnormalScannerExit::WaitFailed { wait_failure_kind } => {
            translated_messages::widgets::lazaret_lost_track_scanner(wait_failure_kind)
        }
    }
}
