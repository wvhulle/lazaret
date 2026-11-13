use unix_mode::{Access, Accessor};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessAudience {
    Everyone,
    OwnerAndGroup,
    OnlyOwner,
    OnlyAdministrator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramRunning {
    NotRunnable,
    Runnable(AccessAudience),
    RunsWithOwnerRights,
    RunsWithGroupRights,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileAccessSummary {
    pub opening: AccessAudience,
    pub changing: AccessAudience,
    pub running: ProgramRunning,
}

fn access_audience(permission_mode: u32, requested_access: Access) -> AccessAudience {
    if unix_mode::is_allowed(Accessor::Other, requested_access, permission_mode) {
        AccessAudience::Everyone
    } else if unix_mode::is_allowed(Accessor::Group, requested_access, permission_mode) {
        AccessAudience::OwnerAndGroup
    } else if unix_mode::is_allowed(Accessor::User, requested_access, permission_mode) {
        AccessAudience::OnlyOwner
    } else {
        AccessAudience::OnlyAdministrator
    }
}

fn program_running(permission_mode: u32) -> ProgramRunning {
    let owner_can_run = unix_mode::is_allowed(Accessor::User, Access::Execute, permission_mode);
    let group_can_run = unix_mode::is_allowed(Accessor::Group, Access::Execute, permission_mode);
    if unix_mode::is_setuid(permission_mode) && owner_can_run {
        return ProgramRunning::RunsWithOwnerRights;
    }
    if unix_mode::is_setgid(permission_mode) && group_can_run {
        return ProgramRunning::RunsWithGroupRights;
    }
    match access_audience(permission_mode, Access::Execute) {
        AccessAudience::OnlyAdministrator => ProgramRunning::NotRunnable,
        running_audience => ProgramRunning::Runnable(running_audience),
    }
}

pub fn file_access_summary(permission_mode: u32) -> FileAccessSummary {
    FileAccessSummary {
        opening: access_audience(permission_mode, Access::Read),
        changing: access_audience(permission_mode, Access::Write),
        running: program_running(permission_mode),
    }
}

#[cfg(test)]
mod file_access_summary_tests {
    use super::*;

    #[test]
    fn raw_modes_still_read_like_ls() {
        assert_eq!(unix_mode::to_string(0o100_644), "-rw-r--r--");
        assert_eq!(unix_mode::to_string(0o104_755), "-rwsr-xr-x");
        assert_eq!(unix_mode::to_string(0o120_777), "lrwxrwxrwx");
    }

    #[test]
    fn access_is_summarised_by_the_widest_audience() {
        assert_eq!(
            file_access_summary(0o100_644),
            FileAccessSummary {
                opening: AccessAudience::Everyone,
                changing: AccessAudience::OnlyOwner,
                running: ProgramRunning::NotRunnable,
            }
        );
        assert_eq!(
            file_access_summary(0o100_640).opening,
            AccessAudience::OwnerAndGroup
        );
        assert_eq!(
            file_access_summary(0o100_600).opening,
            AccessAudience::OnlyOwner
        );
        assert_eq!(
            file_access_summary(0o100_000),
            FileAccessSummary {
                opening: AccessAudience::OnlyAdministrator,
                changing: AccessAudience::OnlyAdministrator,
                running: ProgramRunning::NotRunnable,
            }
        );
        assert_eq!(
            file_access_summary(0o100_755).running,
            ProgramRunning::Runnable(AccessAudience::Everyone)
        );
        assert_eq!(
            file_access_summary(0o104_755).running,
            ProgramRunning::RunsWithOwnerRights
        );
        assert_eq!(
            file_access_summary(0o102_750).running,
            ProgramRunning::RunsWithGroupRights
        );
    }
}
