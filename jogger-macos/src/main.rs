#![allow(unexpected_cfgs)]

mod helpers;
mod icon;

use cocoa::appkit::NSTextField;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use helpers::{activate_app, show_alert, show_multi_input_alert, show_single_input_alert};
use icon::create_template_icon;
use jogger_core::{submit_timelog, time::string_to_seconds, Preferences, TimeLog};
use objc::runtime::Class;
use objc::{msg_send, sel, sel_impl};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use time::OffsetDateTime;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Debug, Clone)]
enum UserEvent {
    ReminderTick,
}

// Helper to create empty icon for alerts

// Helper to activate app and bring to front

// Show reminder dialog with 4 options
fn show_reminder_dialog(prefs: Arc<Mutex<Preferences>>) {
    activate_app();

    let mut prefs_lock = prefs.lock().unwrap();

    // Check if we should reset
    if prefs_lock.should_reset_timer() {
        prefs_lock.timer_state.accumulated_seconds = 0;
        prefs_lock.timer_state.last_log_time = None;
    }

    let elapsed = prefs_lock.get_elapsed_seconds();
    let minutes = elapsed / 60;

    let message = if prefs_lock.timer_state.accumulated_seconds > 0 {
        format!(
            "⏰ Time to log!\n\n{} minutes elapsed\n(+{} minutes accumulated)",
            minutes,
            prefs_lock.timer_state.accumulated_seconds / 60
        )
    } else {
        format!("⏰ Time to log!\n\n{} minutes elapsed", minutes)
    };

    drop(prefs_lock);

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];
        let _: () = msg_send![alert, setAlertStyle: 1]; // NSAlertStyleInformational

        let msg_ns = NSString::alloc(nil).init_str(&message);
        let _: () = msg_send![alert, setMessageText: msg_ns];

        let _: () =
            msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Log to Ticket")];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Log to Distraction")];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Continue")];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Cancel")];

        let response: isize = msg_send![alert, runModal];

        match response {
            1000 => {
                // Log to Ticket
                let _ = alert;
                if let Some(values) = show_multi_input_alert(
                    "Log Time to Ticket",
                    &[("Ticket:", "PROJ-123"), ("Time:", &format!("{}m", minutes))],
                ) {
                    if values.len() == 2 {
                        let prefs_lock = prefs.lock().unwrap();
                        let prefs_ref = Rc::new(RefCell::new(prefs_lock.clone()));
                        let timelog = TimeLog {
                            ticket_number: values[0].clone(),
                            time_spent_seconds: string_to_seconds(&values[1])
                                .unwrap_or(elapsed as usize),
                            comment: String::new(),
                            prefs: prefs_ref,
                        };
                        drop(prefs_lock);

                        match submit_timelog(&timelog) {
                            Ok(_) => {
                                let mut prefs_lock = prefs.lock().unwrap();
                                prefs_lock.update_timer_state(&values[0]);
                                let _ = prefs_lock.save();
                                show_alert("Success! ✅", "Time logged successfully!");
                            }
                            Err(e) => {
                                show_alert("Error ❌", &format!("Failed to log time:\n{}", e.msg()))
                            }
                        }
                    }
                }
            }
            1001 => {
                // Log to Distraction
                let _ = alert;
                // Find first PersonalDistraction ticket
                let prefs_lock = prefs.lock().unwrap();
                let distraction = prefs_lock
                    .custom_meetings
                    .iter()
                    .flat_map(|p| &p.meetings)
                    .find(|m| {
                        matches!(
                            m.0,
                            jogger_core::meeting_types::MeetingType::PersonalDistraction
                        )
                    })
                    .map(|m| m.1.clone());
                drop(prefs_lock);

                if let Some(distraction_ticket) = distraction {
                    if let Some(time_str) = show_single_input_alert(
                        "Log Personal Distraction",
                        "Time:",
                        &format!("{}m", minutes),
                    ) {
                        let prefs_lock = prefs.lock().unwrap();
                        let prefs_ref = Rc::new(RefCell::new(prefs_lock.clone()));
                        let timelog = TimeLog {
                            ticket_number: distraction_ticket.clone(),
                            time_spent_seconds: string_to_seconds(&time_str)
                                .unwrap_or(elapsed as usize),
                            comment: String::new(),
                            prefs: prefs_ref,
                        };
                        drop(prefs_lock);

                        match submit_timelog(&timelog) {
                            Ok(_) => {
                                let mut prefs_lock = prefs.lock().unwrap();
                                prefs_lock.update_timer_state(&distraction_ticket);
                                let _ = prefs_lock.save();
                                show_alert("Success! ✅", "Time logged successfully!");
                            }
                            Err(e) => {
                                show_alert("Error ❌", &format!("Failed to log time:\n{}", e.msg()))
                            }
                        }
                    }
                } else {
                    show_alert("Error ❌", "No personal distraction ticket configured!");
                }
            }
            1002 => {
                // Continue
                let _ = alert;
                let prefs_lock = prefs.lock().unwrap();
                if let Some(last_ticket) = prefs_lock.timer_state.last_ticket.clone() {
                    let prefs_ref = Rc::new(RefCell::new(prefs_lock.clone()));
                    let timelog = TimeLog {
                        ticket_number: last_ticket.clone(),
                        time_spent_seconds: elapsed as usize,
                        comment: String::new(),
                        prefs: prefs_ref,
                    };
                    drop(prefs_lock);

                    match submit_timelog(&timelog) {
                        Ok(_) => {
                            let mut prefs_lock = prefs.lock().unwrap();
                            prefs_lock.update_timer_state(&last_ticket);
                            let _ = prefs_lock.save();
                            show_alert("Success! ✅", "Time logged successfully!");
                        }
                        Err(e) => {
                            show_alert("Error ❌", &format!("Failed to log time:\n{:?}", e.msg()))
                        }
                    }
                } else {
                    show_alert("Error ❌", "No previous ticket to continue with!");
                }
            }
            _ => {
                // Cancel - accumulate time
                let _ = alert;
                let mut prefs_lock = prefs.lock().unwrap();
                prefs_lock.timer_state.accumulated_seconds += elapsed;
                prefs_lock.timer_state.last_log_time =
                    Some(OffsetDateTime::now_utc().unix_timestamp());
                let _ = prefs_lock.save();
            }
        }
    }
}

// Helper for single input

// Helper to show native macOS alert with multiple text inputs

fn show_ticket_dialog(prefs: Arc<Mutex<Preferences>>) {
    let fields = vec![
        ("Ticket Number:", "e.g., PROJ-123"),
        ("Time Spent:", "e.g., 1h30m, 1.5h, 90m"),
        ("Comment (optional):", "What did you work on?"),
    ];

    if let Some(values) = show_multi_input_alert("Log Time to Ticket", &fields) {
        if values.len() >= 3 && !values[0].is_empty() && !values[1].is_empty() {
            let ticket = values[0].clone();
            let time_str = values[1].clone();
            let comment = values[2].clone();

            submit_time_log(prefs, ticket, time_str, comment);
        }
    }
}

fn submit_time_log(
    prefs: Arc<Mutex<Preferences>>,
    ticket: String,
    time_str: String,
    comment: String,
) {
    match jogger_core::string_to_seconds(&time_str) {
        Ok(seconds) => {
            let prefs_clone = prefs.lock().unwrap().clone();
            let ticket_clone = ticket.clone();

            std::thread::spawn(move || {
                let prefs_rc = Rc::new(RefCell::new(prefs_clone));

                let log = TimeLog {
                    time_spent_seconds: seconds,
                    comment,
                    ticket_number: ticket_clone.clone(),
                    prefs: prefs_rc,
                };

                match submit_timelog(&log) {
                    Ok(_) => {
                        println!("✅ Time logged successfully to {}!", ticket_clone);
                        show_alert(
                            "Success! ✅",
                            &format!("Logged {} to {}", time_str, ticket_clone),
                        );
                    }
                    Err(e) => {
                        eprintln!("❌ Error: {}", e.msg());
                        show_alert("Error ❌", &format!("Failed to log time:\n{}", e.msg()));
                    }
                }
            });
        }
        Err(e) => {
            show_alert(
                "Invalid Time ⚠️",
                &format!("Could not parse time:\n{}", e.msg()),
            );
        }
    }
}

// Helper to show project/meeting selector with dropdowns
fn show_meeting_selector_dropdown(prefs: Arc<Mutex<Preferences>>) -> Option<String> {
    activate_app(); // Bring app to front!

    let projects = prefs.lock().unwrap().custom_meetings.clone();

    if projects.is_empty() {
        return None;
    }

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];
        let _: () = msg_send![alert, setAlertStyle: 1]; // NSAlertStyleInformational

        let title_ns = NSString::alloc(nil).init_str("Log Personal Distraction");
        let _: () = msg_send![alert, setMessageText: title_ns];

        // Create container
        let container: id = msg_send![Class::get("NSView").unwrap(), alloc];
        let container: id = msg_send![container, initWithFrame: NSRect::new(
            NSPoint::new(0., 0.),
            NSSize::new(400., 180.)
        )];

        // Project dropdown label
        let project_label: id = msg_send![Class::get("NSTextField").unwrap(), alloc];
        let project_label: id = msg_send![project_label, initWithFrame: NSRect::new(
            NSPoint::new(0., 150.),
            NSSize::new(400., 20.)
        )];
        let _: () =
            msg_send![project_label, setStringValue: NSString::alloc(nil).init_str("Project:")];
        let _: () = msg_send![project_label, setBezeled: false];
        let _: () = msg_send![project_label, setDrawsBackground: false];
        let _: () = msg_send![project_label, setEditable: false];
        let _: () = msg_send![container, addSubview: project_label];

        // Project dropdown
        let project_popup: id = msg_send![Class::get("NSPopUpButton").unwrap(), alloc];
        let project_popup: id = msg_send![project_popup, initWithFrame: NSRect::new(
            NSPoint::new(0., 120.),
            NSSize::new(400., 25.)
        ) pullsDown: false];

        for project in &projects {
            let item_title = NSString::alloc(nil).init_str(&project.name);
            let _: () = msg_send![project_popup, addItemWithTitle: item_title];
        }

        let _: () = msg_send![container, addSubview: project_popup];

        // Ticket dropdown label
        let ticket_label: id = msg_send![Class::get("NSTextField").unwrap(), alloc];
        let ticket_label: id = msg_send![ticket_label, initWithFrame: NSRect::new(
            NSPoint::new(0., 90.),
            NSSize::new(400., 20.)
        )];
        let _: () =
            msg_send![ticket_label, setStringValue: NSString::alloc(nil).init_str("Ticket:")];
        let _: () = msg_send![ticket_label, setBezeled: false];
        let _: () = msg_send![ticket_label, setDrawsBackground: false];
        let _: () = msg_send![ticket_label, setEditable: false];
        let _: () = msg_send![container, addSubview: ticket_label];

        // Ticket dropdown
        let ticket_popup: id = msg_send![Class::get("NSPopUpButton").unwrap(), alloc];
        let ticket_popup: id = msg_send![ticket_popup, initWithFrame: NSRect::new(
            NSPoint::new(0., 60.),
            NSSize::new(400., 25.)
        ) pullsDown: false];

        // Populate with first project's meetings
        if let Some(first_project) = projects.first() {
            for meeting in &first_project.meetings {
                let item_title =
                    NSString::alloc(nil).init_str(&format!("{} - {}", meeting.0, meeting.1));
                let _: () = msg_send![ticket_popup, addItemWithTitle: item_title];
            }
        }

        let _: () = msg_send![container, addSubview: ticket_popup];

        // Set up project dropdown to update ticket dropdown
        // Note: This is simplified - in a full implementation we'd use proper target/action

        let _: () = msg_send![alert, setAccessoryView: container];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Continue")];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Cancel")];

        // Remove the icon

        let response: isize = msg_send![alert, runModal];

        if response == 1000 {
            let project_idx: isize = msg_send![project_popup, indexOfSelectedItem];
            let ticket_idx: isize = msg_send![ticket_popup, indexOfSelectedItem];

            if project_idx >= 0 && ticket_idx >= 0 {
                if let Some(project) = projects.get(project_idx as usize) {
                    if let Some(meeting) = project.meetings.get(ticket_idx as usize) {
                        return Some(meeting.1.clone());
                    }
                }
            }
        }

        None
    }
}

fn show_distraction_dialog(prefs: Arc<Mutex<Preferences>>) {
    let projects = prefs.lock().unwrap().custom_meetings.clone();

    if projects.is_empty() {
        show_alert(
            "No Projects",
            "No distraction tickets configured.\nEdit preferences to add them.",
        );
        return;
    }

    // Show dropdown selector
    if let Some(ticket) = show_meeting_selector_dropdown(Arc::clone(&prefs)) {
        let fields = vec![
            ("Time Spent:", "e.g., 1h30m, 1.5h, 90m"),
            ("Comment (optional):", "What did you work on?"),
        ];

        if let Some(values) =
            show_multi_input_alert(&format!("Log Distraction: {}", ticket), &fields)
        {
            if values.len() >= 2 && !values[0].is_empty() {
                submit_time_log(prefs, ticket, values[0].clone(), values[1].clone());
            }
        }
    }
}

fn show_preferences_dialog(prefs: Arc<Mutex<Preferences>>) {
    activate_app();

    let current = prefs.lock().unwrap().clone();

    let fields = [
        ("Name:", &current.name as &str),
        ("Email:", &current.email),
        ("API Key:", &current.api_key),
        ("Jira URL:", &current.jira_url),
    ];

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];
        let _: () = msg_send![alert, setAlertStyle: 1]; // NSAlertStyleInformational

        let title_ns = NSString::alloc(nil).init_str("Preferences ⚙️");
        let _: () = msg_send![alert, setMessageText: title_ns];

        let container: id = msg_send![Class::get("NSView").unwrap(), alloc];
        let height = (fields.len() as f64) * 50.0 + 120.0; // Extra space for reminder settings
        let container: id = msg_send![container, initWithFrame: NSRect::new(
            NSPoint::new(0., 0.),
            NSSize::new(400., height)
        )];

        let mut text_fields = Vec::new();

        for (i, (label, value)) in fields.iter().enumerate() {
            let y = height - ((i + 1) as f64 * 50.0);

            let label_view: id = msg_send![Class::get("NSTextField").unwrap(), alloc];
            let label_view: id = msg_send![label_view, initWithFrame: NSRect::new(
                NSPoint::new(0., y + 20.),
                NSSize::new(400., 20.)
            )];
            let label_ns = NSString::alloc(nil).init_str(label);
            let _: () = msg_send![label_view, setStringValue: label_ns];
            let _: () = msg_send![label_view, setBezeled: false];
            let _: () = msg_send![label_view, setDrawsBackground: false];
            let _: () = msg_send![label_view, setEditable: false];
            let _: () = msg_send![label_view, setSelectable: false];
            let _: () = msg_send![container, addSubview: label_view];

            let text_field = NSTextField::alloc(nil);
            let text_field: id = msg_send![text_field, initWithFrame: NSRect::new(
                NSPoint::new(0., y),
                NSSize::new(400., 24.)
            )];
            let value_ns = NSString::alloc(nil).init_str(value);
            let _: () = msg_send![text_field, setStringValue: value_ns];
            let _: () = msg_send![container, addSubview: text_field];

            text_fields.push(text_field);
        }

        // Reminder settings section - below all text fields
        let reminder_y = 60.0;

        // Checkbox
        let checkbox: id = msg_send![Class::get("NSButton").unwrap(), alloc];
        let checkbox: id = msg_send![checkbox, initWithFrame: NSRect::new(
            NSPoint::new(0., reminder_y + 30.),
            NSSize::new(400., 20.)
        )];
        let _: () = msg_send![checkbox, setButtonType: 3]; // Switch button
        let _: () = msg_send![checkbox, setTitle: NSString::alloc(nil).init_str("Enable ADHD-Friendly Reminders ⏰")];
        let _: () =
            msg_send![checkbox, setState: if current.reminder_settings.enabled { 1 } else { 0 }];
        let _: () = msg_send![container, addSubview: checkbox];

        // Interval dropdown
        let popup: id = msg_send![Class::get("NSPopUpButton").unwrap(), alloc];
        let popup: id = msg_send![popup, initWithFrame: NSRect::new(
            NSPoint::new(0., reminder_y - 5.),
            NSSize::new(200., 30.)
        )];
        let _: () =
            msg_send![popup, addItemWithTitle: NSString::alloc(nil).init_str("Every 15 minutes")];
        let _: () =
            msg_send![popup, addItemWithTitle: NSString::alloc(nil).init_str("Every 30 minutes")];
        let _: () =
            msg_send![popup, addItemWithTitle: NSString::alloc(nil).init_str("Every 60 minutes")];

        let selected_idx = match current.reminder_settings.interval_minutes {
            15 => 0,
            30 => 1,
            _ => 2,
        };
        let _: () = msg_send![popup, selectItemAtIndex: selected_idx];
        let _: () = msg_send![container, addSubview: popup];

        let _: () = msg_send![alert, setAccessoryView: container];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Save")];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Cancel")];

        let response: isize = msg_send![alert, runModal];

        if response == 1000 {
            let mut new_prefs = current;

            for (i, text_field) in text_fields.iter().enumerate() {
                let value: id = msg_send![*text_field, stringValue];
                let bytes: *const u8 = msg_send![value, UTF8String];
                let len: usize = msg_send![value, lengthOfBytesUsingEncoding: 4];

                if !bytes.is_null() {
                    let slice = std::slice::from_raw_parts(bytes, len);
                    let string_value = String::from_utf8_lossy(slice).to_string();

                    match i {
                        0 => new_prefs.name = string_value,
                        1 => new_prefs.email = string_value,
                        2 => new_prefs.api_key = string_value,
                        3 => new_prefs.jira_url = string_value,
                        _ => {}
                    }
                }
            }

            // Get reminder settings
            let checkbox_state: isize = msg_send![checkbox, state];
            let was_enabled = new_prefs.reminder_settings.enabled;
            new_prefs.reminder_settings.enabled = checkbox_state == 1;

            // Initialize timer when first enabled
            if !was_enabled && new_prefs.reminder_settings.enabled && new_prefs.timer_state.last_log_time.is_none() {
                new_prefs.timer_state.last_log_time = Some(OffsetDateTime::now_utc().unix_timestamp());
                new_prefs.timer_state.last_log_date = Some(format!("{}-{:02}-{:02}", 
                    OffsetDateTime::now_utc().year(),
                    OffsetDateTime::now_utc().month() as u8,
                    OffsetDateTime::now_utc().day()
                ));
            }

            let selected: isize = msg_send![popup, indexOfSelectedItem];
            new_prefs.reminder_settings.interval_minutes = match selected {
                0 => 15,
                1 => 30,
                _ => 60,
            };

            match new_prefs.save() {
                Ok(_) => {
                    *prefs.lock().unwrap() = new_prefs;
                    show_alert("Success! ✅", "Preferences saved successfully!");
                }
                Err(e) => {
                    show_alert("Error ❌", &format!("Failed to save preferences:\n{}", e));
                }
            }
        }
    }
}

fn main() {
    println!("🏃🏼‍♀️ Jogger - Menu Bar App");
    println!("✨ Look for Gerald the Gentleman Runner in your menu bar!");

    let prefs = Arc::new(Mutex::new(Preferences::load().unwrap_or_default()));

    let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event().build().unwrap();

    // Create menu
    let menu = Menu::new();
    let log_ticket = MenuItem::new("📝 Log Time to Ticket", true, None);
    let log_distraction = MenuItem::new("☕ Log Personal Distraction", true, None);
    let preferences = MenuItem::new("⚙️  Preferences", true, None);
    let about_gerald = MenuItem::new("About Gerald...", true, None); // Easter egg!
    let quit = MenuItem::new("Quit", true, None);

    menu.append(&log_ticket).unwrap();
    menu.append(&log_distraction).unwrap();
    menu.append(&preferences).unwrap();
    menu.append(&about_gerald).unwrap();
    menu.append(&quit).unwrap();

    // Create tray icon with template image
    let icon = create_template_icon();
    let _tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Jogger - Jira Time Logger 🏃🏼‍♀️")
        .with_icon(icon)
        .with_icon_as_template(true) // This makes it adapt to light/dark mode!
        .build()
        .unwrap();

    let menu_channel = MenuEvent::receiver();

    // Spawn background timer thread
    let proxy = event_loop.create_proxy();
    let prefs_timer = Arc::clone(&prefs);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(60)); // Check every minute

        let prefs = prefs_timer.lock().unwrap();
        if prefs.reminder_settings.enabled {
            let elapsed = prefs.get_elapsed_seconds();
            let interval_seconds = prefs.reminder_settings.interval_minutes * 60;

            if elapsed >= interval_seconds {
                let _ = proxy.send_event(UserEvent::ReminderTick);
            }
        }
    });

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        // Handle timer events
        if let winit::event::Event::UserEvent(UserEvent::ReminderTick) = event {
            show_reminder_dialog(Arc::clone(&prefs));
            return;
        }

        if let Ok(event) = menu_channel.try_recv() {
            let prefs = Arc::clone(&prefs);

            if event.id == log_ticket.id() {
                show_ticket_dialog(prefs);
            } else if event.id == log_distraction.id() {
                show_distraction_dialog(prefs);
            } else if event.id == preferences.id() {
                show_preferences_dialog(prefs);
            } else if event.id == about_gerald.id() {
                // Gerald Easter Egg!
                show_alert(
                    "Meet Gerald! 🏃‍♂️",
                    "Gerald the Gentleman Runner\n\n\
                    Gerald is a THICCC (note the 3 Cs) stick figure who believes in:\n\
                    • Respectful leg positioning (no manspreading!)\n\
                    • Proper time logging\n\
                    • Bringing good vibes to your menu bar\n\n\
                    He's been running since 2026 and shows no signs of stopping.\n\n\
                    Fun fact: Gerald adapts to light and dark mode!\n\
                    He's always dressed appropriately for the occasion. 🎩",
                );
            } else if event.id == quit.id() {
                println!("👋 Gerald says goodbye!");
                elwt.exit();
            }
        }
    });
}
