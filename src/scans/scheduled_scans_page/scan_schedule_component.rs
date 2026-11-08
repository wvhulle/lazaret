use relm4::{Component, ComponentParts, ComponentSender, adw, adw::prelude::*, gtk, gtk::gio};

use crate::{
    host_services::{
        desktop::{
            locale_formatting::exact_local_time_text, stored_settings::application_settings,
            stored_settings_watch::forward_setting_changes,
        },
        systemd_bus::host_systemd_units::{
            SystemdBusFailure, TimerUnitSnapshot, UnitTimestamp, timer_unit_snapshot,
        },
    },
    scans::scheduled_scans_page::{
        scan_schedule_settings::{
            SCHEDULE_SETTING_KEYS, StoredScheduleSettings, frequency_at, frequency_position,
            joined_scan_targets, listed_scan_targets,
        },
        scan_schedule_units::{
            ScheduleApplyFailure, ScheduleTimerState, disable_scan_schedule, enable_scan_schedule,
            read_schedule_timer_state, systemd_user_unit_directory,
        },
    },
    shared_widgets::listed_spin_row::listed_spin_row,
    translated_messages,
};

const SYSTEM_SCAN_TIMER: &str = "clamdscan.timer";

pub struct ScanScheduleSection {
    stored_settings: gio::Settings,
    schedule_settings: StoredScheduleSettings,
    scan_targets_text: String,
    entry_texts_replaced: bool,
    user_timer_view: UserTimerView,
    system_schedule_owner: SystemScheduleOwner,
    page_notice: ScheduleSectionNotice,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserTimerView {
    ReadingUserTimer,
    UserTimerRead(ScheduleTimerState),
}

#[derive(Debug)]
pub enum ScheduleSectionNotice {
    NoNoticeShown,
    ScanTimeUnreadable,
    ScheduleNotApplied(ScheduleApplyFailure),
}

#[derive(Debug)]
pub enum ScanScheduleSectionInput {
    ScheduleToggled(bool),
    FrequencyChosen(u32),
    TimeTextChanged(String),
    WeekdayChosen(u32),
    MonthDayChanged(f64),
    TargetsTextChanged(String),
    BatterySkipToggled(bool),
    AutoQuarantineToggled(bool),
    StoredSettingChanged,
    ApplyRequested,
}

#[derive(Debug)]
pub enum ScanScheduleSectionCommand {
    TimerStateLoaded(ScheduleTimerState),
    SystemTimerInspected(SystemScheduleOwner),
    ScheduleApplied(Result<(), ScheduleApplyFailure>),
}

fn describe_timer_state(timer_state: &ScheduleTimerState) -> String {
    match timer_state {
        ScheduleTimerState::TimerInactive => translated_messages::scans::timer_inactive(),
        ScheduleTimerState::TimerActive {
            next_run: UnitTimestamp::RecordedAt { recorded_at },
        } => translated_messages::scans::next_run_time(exact_local_time_text(*recorded_at)),
        ScheduleTimerState::TimerActive {
            next_run: UnitTimestamp::NeverRecorded,
        } => translated_messages::scans::scheduled_time_next_run(),
        ScheduleTimerState::SystemdUnavailable => translated_messages::scans::systemd_unavailable(),
    }
}

fn describe_system_schedule(next_run: UnitTimestamp) -> String {
    match next_run {
        UnitTimestamp::RecordedAt { recorded_at } => {
            translated_messages::scans::system_already_runs_scheduled(exact_local_time_text(
                recorded_at,
            ))
        }
        UnitTimestamp::NeverRecorded => translated_messages::scans::system_scan_timer_settings(),
    }
}

fn describe_user_timer(user_timer_view: &UserTimerView) -> String {
    match user_timer_view {
        UserTimerView::ReadingUserTimer => translated_messages::scans::reading_user_timer(),
        UserTimerView::UserTimerRead(timer_state) => describe_timer_state(timer_state),
    }
}

fn describe_schedule_group(
    system_schedule_owner: &SystemScheduleOwner,
    user_timer_view: &UserTimerView,
) -> String {
    match system_schedule_owner {
        SystemScheduleOwner::InspectingSystemTimer => {
            translated_messages::scans::inspecting_system_timer()
        }
        SystemScheduleOwner::NoSystemTimer => describe_user_timer(user_timer_view),
        SystemScheduleOwner::SystemTimerExists { next_run } => describe_system_schedule(*next_run),
        SystemScheduleOwner::SystemTimerUnknown(inspection_failure) => {
            translated_messages::scans::system_timer_unknown(inspection_failure)
        }
    }
}

fn describe_page_notice(page_notice: &ScheduleSectionNotice) -> String {
    match page_notice {
        ScheduleSectionNotice::NoNoticeShown => String::new(),
        ScheduleSectionNotice::ScanTimeUnreadable => {
            translated_messages::scans::scan_time_unreadable()
        }
        ScheduleSectionNotice::ScheduleNotApplied(apply_failure) => {
            describe_apply_failure(apply_failure)
        }
    }
}

const fn shows_page_notice(page_notice: &ScheduleSectionNotice) -> bool {
    !matches!(page_notice, ScheduleSectionNotice::NoNoticeShown)
}

fn describe_apply_failure(apply_failure: &ScheduleApplyFailure) -> String {
    match apply_failure {
        ScheduleApplyFailure::UnitFilesUnwritable(unit_failure) => {
            translated_messages::scans::unit_files_unwritable(unit_failure)
        }
        ScheduleApplyFailure::UserManagerFailed(manager_failure) => {
            translated_messages::scans::timer_not_applied_error(manager_failure)
        }
    }
}

fn frequency_labels() -> gtk::StringList {
    let labels = [
        translated_messages::scans::hourly_label(),
        translated_messages::scans::daily_label(),
        translated_messages::scans::weekly_label(),
        translated_messages::scans::monthly_label(),
    ];
    gtk::StringList::new(&labels.iter().map(String::as_str).collect::<Vec<_>>())
}

fn weekday_labels() -> gtk::StringList {
    let labels = [
        translated_messages::scans::monday_label(),
        translated_messages::scans::tuesday_label(),
        translated_messages::scans::wednesday_label(),
        translated_messages::scans::thursday_label(),
        translated_messages::scans::friday_label(),
        translated_messages::scans::saturday_label(),
        translated_messages::scans::sunday_label(),
    ];
    gtk::StringList::new(&labels.iter().map(String::as_str).collect::<Vec<_>>())
}

#[derive(Clone, Debug, PartialEq)]
pub enum SystemScheduleOwner {
    InspectingSystemTimer,
    NoSystemTimer,
    SystemTimerExists { next_run: UnitTimestamp },
    SystemTimerUnknown(SystemdBusFailure),
}

pub fn system_schedule_owner(timer_snapshot: TimerUnitSnapshot) -> SystemScheduleOwner {
    match timer_snapshot {
        TimerUnitSnapshot::LoadedTimer { next_run, .. } => {
            SystemScheduleOwner::SystemTimerExists { next_run }
        }
        TimerUnitSnapshot::TimerNotFound => SystemScheduleOwner::NoSystemTimer,
        TimerUnitSnapshot::SystemdUnreachable(inspection_failure) => {
            SystemScheduleOwner::SystemTimerUnknown(inspection_failure)
        }
    }
}

pub const fn allows_applying_schedule(system_schedule_owner: &SystemScheduleOwner) -> bool {
    matches!(system_schedule_owner, SystemScheduleOwner::NoSystemTimer)
}

impl ScanScheduleSection {
    fn inspect_timers(sender: &ComponentSender<Self>) {
        sender.oneshot_command(async {
            ScanScheduleSectionCommand::SystemTimerInspected(system_schedule_owner(
                timer_unit_snapshot(SYSTEM_SCAN_TIMER).await,
            ))
        });
        sender.oneshot_command(async {
            ScanScheduleSectionCommand::TimerStateLoaded(read_schedule_timer_state().await)
        });
    }

    fn change_schedule_settings(&mut self, apply_change: impl FnOnce(&mut StoredScheduleSettings)) {
        apply_change(&mut self.schedule_settings);
        self.schedule_settings.write_to(&self.stored_settings);
    }

    fn show_stored_settings(&mut self) {
        let stored_schedule = StoredScheduleSettings::read_from(&self.stored_settings);
        if stored_schedule == self.schedule_settings {
            return;
        }
        if stored_schedule.schedule_time_text != self.schedule_settings.schedule_time_text
            || stored_schedule.scan_targets != listed_scan_targets(&self.scan_targets_text)
        {
            self.scan_targets_text = joined_scan_targets(&stored_schedule.scan_targets);
            self.entry_texts_replaced = true;
        }
        self.schedule_settings = stored_schedule;
    }
}

#[relm4::component(pub)]
impl Component for ScanScheduleSection {
    type Init = ();
    type Input = ScanScheduleSectionInput;
    type Output = ();
    type CommandOutput = ScanScheduleSectionCommand;

    view! {
        adw::PreferencesGroup {

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,

            adw::PreferencesGroup {
                set_title: &translated_messages::scans::schedule_title(),
                #[watch]
                set_sensitive: allows_applying_schedule(&model.system_schedule_owner),
                #[watch]
                set_description: Some(&describe_schedule_group(
                    &model.system_schedule_owner,
                    &model.user_timer_view,
                )),
                #[wrap(Some)]
                set_header_suffix = &gtk::Button {
                    set_label: &translated_messages::scans::apply_label(),
                    set_valign: gtk::Align::Center,
                    add_css_class: "suggested-action",
                    connect_clicked => ScanScheduleSectionInput::ApplyRequested,
                },

                adw::SwitchRow {
                    set_title: &translated_messages::scans::enable_scheduled_scans_title(),
                    #[watch]
                    #[block_signal(schedule_toggled_handler)]
                    set_active: model.schedule_settings.schedule_enabled,
                    connect_active_notify[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::ScheduleToggled(changed_row.is_active()));
                    } @schedule_toggled_handler,
                },

                adw::ComboRow {
                    set_title: &translated_messages::scans::frequency_title(),
                    set_model: Some(&frequency_labels()),
                    #[watch]
                    #[block_signal(frequency_chosen_handler)]
                    set_selected: frequency_position(model.schedule_settings.frequency_choice),
                    connect_selected_notify[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::FrequencyChosen(changed_row.selected()));
                    } @frequency_chosen_handler,
                },

                adw::EntryRow {
                    set_title: &translated_messages::scans::time_24_hour_title(),
                    #[track(model.entry_texts_replaced)]
                    #[block_signal(time_changed_handler)]
                    set_text: &model.schedule_settings.schedule_time_text,
                    connect_changed[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::TimeTextChanged(changed_row.text().to_string()));
                    } @time_changed_handler,
                },

                adw::ComboRow {
                    set_title: &translated_messages::scans::day_week_title(),
                    set_subtitle: &translated_messages::scans::used_weekly_scans_subtitle(),
                    set_model: Some(&weekday_labels()),
                    #[watch]
                    #[block_signal(weekday_chosen_handler)]
                    set_selected: model.schedule_settings.weekday_position,
                    connect_selected_notify[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::WeekdayChosen(changed_row.selected()));
                    } @weekday_chosen_handler,
                },

                listed_spin_row(&translated_messages::scans::day_month_title()) -> adw::SpinRow {
                    set_subtitle: &translated_messages::scans::used_monthly_scans_subtitle(),
                    set_adjustment: Some(&gtk::Adjustment::new(1.0, 1.0, 28.0, 1.0, 7.0, 0.0)),
                    #[watch]
                    #[block_signal(month_day_handler)]
                    set_value: f64::from(model.schedule_settings.month_day),
                    connect_value_notify[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::MonthDayChanged(changed_row.value()));
                    } @month_day_handler,
                },

                adw::EntryRow {
                    set_title: &translated_messages::scans::targets_separated_commas_title(),
                    #[track(model.entry_texts_replaced)]
                    #[block_signal(targets_changed_handler)]
                    set_text: &model.scan_targets_text,
                    connect_changed[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::TargetsTextChanged(changed_row.text().to_string()));
                    } @targets_changed_handler,
                },

                adw::SwitchRow {
                    set_title: &translated_messages::scans::skip_battery_schedule_title(),
                    set_subtitle: &translated_messages::scans::not_start_scheduled_subtitle(),
                    #[watch]
                    #[block_signal(battery_skip_handler)]
                    set_active: model.schedule_settings.skip_on_battery,
                    connect_active_notify[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::BatterySkipToggled(changed_row.is_active()));
                    } @battery_skip_handler,
                },

                adw::SwitchRow {
                    set_title: &translated_messages::scans::auto_quarantine_title(),
                    set_subtitle: &translated_messages::scans::automatically_quarantine_threats_subtitle(),
                    #[watch]
                    #[block_signal(auto_quarantine_handler)]
                    set_active: model.schedule_settings.auto_quarantine,
                    connect_active_notify[sender] => move |changed_row| {
                        sender.input(ScanScheduleSectionInput::AutoQuarantineToggled(changed_row.is_active()));
                    } @auto_quarantine_handler,
                },
            },

            gtk::Label {
                set_xalign: 0.0,
                set_wrap: true,
                add_css_class: "error",
                #[watch]
                set_visible: shows_page_notice(&model.page_notice),
                #[watch]
                set_label: &describe_page_notice(&model.page_notice),
            },
            },
        }
    }

    fn init((): (), root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let stored_settings = application_settings();
        forward_setting_changes(
            &stored_settings,
            &SCHEDULE_SETTING_KEYS,
            sender.input_sender(),
            |_changed_settings, _changed_key| ScanScheduleSectionInput::StoredSettingChanged,
        );
        let schedule_settings = StoredScheduleSettings::read_from(&stored_settings);
        let model = Self {
            scan_targets_text: joined_scan_targets(&schedule_settings.scan_targets),
            entry_texts_replaced: true,
            schedule_settings,
            stored_settings,
            user_timer_view: UserTimerView::ReadingUserTimer,
            system_schedule_owner: SystemScheduleOwner::InspectingSystemTimer,
            page_notice: ScheduleSectionNotice::NoNoticeShown,
        };
        let widgets = view_output!();
        Self::inspect_timers(&sender);
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        page_input: ScanScheduleSectionInput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        self.entry_texts_replaced = false;
        match page_input {
            ScanScheduleSectionInput::ScheduleToggled(schedule_enabled) => {
                self.change_schedule_settings(|schedule_settings| {
                    schedule_settings.schedule_enabled = schedule_enabled;
                });
            }
            ScanScheduleSectionInput::FrequencyChosen(chosen_position) => {
                if let Some(schedule_frequency) = frequency_at(chosen_position) {
                    self.change_schedule_settings(|schedule_settings| {
                        schedule_settings.frequency_choice = schedule_frequency;
                    });
                }
            }
            ScanScheduleSectionInput::TimeTextChanged(schedule_time_text) => {
                self.change_schedule_settings(|schedule_settings| {
                    schedule_settings.schedule_time_text = schedule_time_text;
                });
            }
            ScanScheduleSectionInput::WeekdayChosen(weekday_position) => {
                self.change_schedule_settings(|schedule_settings| {
                    schedule_settings.weekday_position = weekday_position;
                });
            }
            ScanScheduleSectionInput::MonthDayChanged(spin_value) => {
                let month_day = spin_value.clamp(1.0, 28.0) as u32;
                self.change_schedule_settings(|schedule_settings| {
                    schedule_settings.month_day = month_day;
                });
            }
            ScanScheduleSectionInput::TargetsTextChanged(scan_targets_text) => {
                let scan_targets = listed_scan_targets(&scan_targets_text);
                self.scan_targets_text = scan_targets_text;
                self.change_schedule_settings(|schedule_settings| {
                    schedule_settings.scan_targets = scan_targets;
                });
            }
            ScanScheduleSectionInput::BatterySkipToggled(skip_on_battery) => {
                self.change_schedule_settings(|schedule_settings| {
                    schedule_settings.skip_on_battery = skip_on_battery;
                });
            }
            ScanScheduleSectionInput::AutoQuarantineToggled(auto_quarantine) => {
                self.change_schedule_settings(|schedule_settings| {
                    schedule_settings.auto_quarantine = auto_quarantine;
                });
            }
            ScanScheduleSectionInput::StoredSettingChanged => self.show_stored_settings(),
            ScanScheduleSectionInput::ApplyRequested => {
                if !allows_applying_schedule(&self.system_schedule_owner) {
                    return;
                }
                self.page_notice = ScheduleSectionNotice::NoNoticeShown;
                if !self.schedule_settings.schedule_enabled {
                    sender.oneshot_command(async {
                        ScanScheduleSectionCommand::ScheduleApplied(
                            disable_scan_schedule(systemd_user_unit_directory()).await,
                        )
                    });
                    return;
                }
                let Some(scan_schedule) = self.schedule_settings.scan_schedule() else {
                    self.page_notice = ScheduleSectionNotice::ScanTimeUnreadable;
                    return;
                };
                sender.oneshot_command(async move {
                    ScanScheduleSectionCommand::ScheduleApplied(
                        enable_scan_schedule(systemd_user_unit_directory(), scan_schedule).await,
                    )
                });
            }
        }
    }

    fn update_cmd(
        &mut self,
        page_command: ScanScheduleSectionCommand,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        self.entry_texts_replaced = false;
        match page_command {
            ScanScheduleSectionCommand::TimerStateLoaded(timer_state) => {
                self.user_timer_view = UserTimerView::UserTimerRead(timer_state);
            }
            ScanScheduleSectionCommand::SystemTimerInspected(system_schedule_owner) => {
                self.system_schedule_owner = system_schedule_owner;
            }
            ScanScheduleSectionCommand::ScheduleApplied(apply_outcome) => {
                if let Err(apply_failure) = apply_outcome {
                    self.page_notice = ScheduleSectionNotice::ScheduleNotApplied(apply_failure);
                }
                self.user_timer_view = UserTimerView::ReadingUserTimer;
                Self::inspect_timers(&sender);
            }
        }
    }
}

#[cfg(test)]
mod scan_schedule_component_tests {
    use super::*;
    use crate::host_services::systemd_bus::host_systemd_units::UnitActivity;

    #[test]
    fn an_installed_system_timer_locks_the_schedule_even_when_stopped() {
        [UnitActivity::UnitActive, UnitActivity::UnitInactive]
            .into_iter()
            .for_each(|unit_activity| {
                assert!(matches!(
                    system_schedule_owner(TimerUnitSnapshot::LoadedTimer {
                        unit_activity,
                        next_run: UnitTimestamp::NeverRecorded,
                        last_trigger: UnitTimestamp::NeverRecorded,
                    }),
                    SystemScheduleOwner::SystemTimerExists { .. }
                ));
            });
    }

    #[test]
    fn hosts_without_a_system_timer_keep_their_own_schedule() {
        let system_schedule_owner = system_schedule_owner(TimerUnitSnapshot::TimerNotFound);
        assert_eq!(system_schedule_owner, SystemScheduleOwner::NoSystemTimer);
        assert!(allows_applying_schedule(&system_schedule_owner));
    }

    #[test]
    fn the_schedule_cannot_be_applied_while_inspecting_or_without_systemd() {
        [
            SystemScheduleOwner::InspectingSystemTimer,
            system_schedule_owner(TimerUnitSnapshot::SystemdUnreachable(
                SystemdBusFailure::UnrecognizedLoadState,
            )),
            system_schedule_owner(TimerUnitSnapshot::SystemdUnreachable(
                SystemdBusFailure::BusCallFailed(zbus::Error::InvalidReply),
            )),
            SystemScheduleOwner::SystemTimerExists {
                next_run: UnitTimestamp::NeverRecorded,
            },
        ]
        .iter()
        .for_each(|system_schedule_owner| {
            assert!(!allows_applying_schedule(system_schedule_owner));
        });
    }
}
