use crate::{
    scans::manual_scan_page::scan_profiles_dialog::scan_profile_store::{
        BuiltinProfile, ProfileOrigin, ProfileStoreFailure, ScanProfile,
    },
    translated_messages,
};

pub fn builtin_profile_name(builtin_profile: BuiltinProfile) -> String {
    match builtin_profile {
        BuiltinProfile::QuickScan => translated_messages::scans::quick_scan(),
        BuiltinProfile::FullScan => translated_messages::scans::full_scan(),
        BuiltinProfile::HomeFolder => translated_messages::scans::home_folder(),
    }
}

pub fn builtin_profile_description(builtin_profile: BuiltinProfile) -> String {
    match builtin_profile {
        BuiltinProfile::QuickScan => translated_messages::scans::fast_scan_downloads_folder(),
        BuiltinProfile::FullScan => translated_messages::scans::comprehensive_system_wide_scan(),
        BuiltinProfile::HomeFolder => translated_messages::scans::user_home_directory_personal(),
    }
}

pub fn displayed_profile_name(scan_profile: &ScanProfile) -> String {
    match scan_profile.profile_origin {
        ProfileOrigin::Builtin(builtin_profile) => builtin_profile_name(builtin_profile),
        ProfileOrigin::Custom { .. } => scan_profile.profile_draft.profile_name.clone(),
    }
}

pub fn displayed_profile_description(scan_profile: &ScanProfile) -> String {
    match scan_profile.profile_origin {
        ProfileOrigin::Builtin(builtin_profile) => builtin_profile_description(builtin_profile),
        ProfileOrigin::Custom { .. } => scan_profile.profile_draft.profile_description.clone(),
    }
}

pub fn describe_profile_store_failure(store_failure: &ProfileStoreFailure) -> String {
    match store_failure {
        ProfileStoreFailure::ProfileFileUnavailable(access_failure) => {
            translated_messages::scans::profile_file_not_accessed(access_failure)
        }
        ProfileStoreFailure::ProfileDataInvalid(parse_failure) => {
            translated_messages::scans::profile_data_not_valid(parse_failure)
        }
        ProfileStoreFailure::ProfileFileNotReplaced(persist_failure) => {
            translated_messages::scans::profile_file_not_replaced(persist_failure)
        }
        ProfileStoreFailure::ProfileTaskInterrupted(interrupted_task) => {
            translated_messages::scans::profile_task_interrupted_error(interrupted_task)
        }
        ProfileStoreFailure::ProfileNameMissing => {
            translated_messages::scans::profile_name_required()
        }
        ProfileStoreFailure::ProfileNameTooLong => {
            translated_messages::scans::name_50_characters_less()
        }
        ProfileStoreFailure::ProfileNameTaken => {
            translated_messages::scans::another_profile_already_uses()
        }
        ProfileStoreFailure::ProfileTargetsMissing => {
            translated_messages::scans::add_least_one_target()
        }
    }
}
