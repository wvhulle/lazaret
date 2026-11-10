use crate::{
    scans::manual_scan_page::scan_profiles_dialog::scan_profile_store::{
        BuiltinProfile, ProfileOrigin, ProfileStoreFailure, ScanProfile,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub fn builtin_profile_name(
    message_catalog: &MessageCatalog,
    builtin_profile: BuiltinProfile,
) -> String {
    match builtin_profile {
        BuiltinProfile::QuickScan => translated_messages::scans::quick_scan(message_catalog),
        BuiltinProfile::FullScan => translated_messages::scans::full_scan(message_catalog),
        BuiltinProfile::HomeFolder => translated_messages::scans::home_folder(message_catalog),
    }
}

pub fn builtin_profile_description(
    message_catalog: &MessageCatalog,
    builtin_profile: BuiltinProfile,
) -> String {
    match builtin_profile {
        BuiltinProfile::QuickScan => {
            translated_messages::scans::fast_scan_downloads_folder(message_catalog)
        }
        BuiltinProfile::FullScan => {
            translated_messages::scans::comprehensive_system_wide_scan(message_catalog)
        }
        BuiltinProfile::HomeFolder => {
            translated_messages::scans::user_home_directory_personal(message_catalog)
        }
    }
}

pub fn displayed_profile_name(
    message_catalog: &MessageCatalog,
    scan_profile: &ScanProfile,
) -> String {
    match scan_profile.profile_origin {
        ProfileOrigin::Builtin(builtin_profile) => {
            builtin_profile_name(message_catalog, builtin_profile)
        }
        ProfileOrigin::Custom { .. } => scan_profile.profile_draft.profile_name.clone(),
    }
}

pub fn displayed_profile_description(
    message_catalog: &MessageCatalog,
    scan_profile: &ScanProfile,
) -> String {
    match scan_profile.profile_origin {
        ProfileOrigin::Builtin(builtin_profile) => {
            builtin_profile_description(message_catalog, builtin_profile)
        }
        ProfileOrigin::Custom { .. } => scan_profile.profile_draft.profile_description.clone(),
    }
}

pub fn describe_profile_store_failure(
    message_catalog: &MessageCatalog,
    store_failure: &ProfileStoreFailure,
) -> String {
    match store_failure {
        ProfileStoreFailure::ProfileFileUnavailable(access_failure) => {
            translated_messages::scans::profile_file_unavailable(message_catalog, access_failure)
        }
        ProfileStoreFailure::ProfileDataInvalid(parse_failure) => {
            translated_messages::scans::profile_data_invalid(message_catalog, parse_failure)
        }
        ProfileStoreFailure::ProfileFileNotReplaced(persist_failure) => {
            translated_messages::scans::profile_file_not_replaced(message_catalog, persist_failure)
        }
        ProfileStoreFailure::ProfileTaskInterrupted(interrupted_task) => {
            translated_messages::scans::profile_task_interrupted(message_catalog, interrupted_task)
        }
        ProfileStoreFailure::ProfileNameMissing => {
            translated_messages::scans::profile_name_missing(message_catalog)
        }
        ProfileStoreFailure::ProfileNameTooLong => {
            translated_messages::scans::profile_name_too_long(message_catalog)
        }
        ProfileStoreFailure::ProfileNameTaken => {
            translated_messages::scans::profile_name_taken(message_catalog)
        }
        ProfileStoreFailure::ProfileTargetsMissing => {
            translated_messages::scans::profile_targets_missing(message_catalog)
        }
    }
}
