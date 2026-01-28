use cocoa::appkit::NSTextField;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use jogger_core::{submit_timelog, Preferences, TimeLog};
use objc::runtime::Class;
use objc::{msg_send, sel, sel_impl};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};
use winit::event_loop::{ControlFlow, EventLoop};

// Helper to activate app and bring to front
fn activate_app() {
    unsafe {
        let app: id = msg_send![Class::get("NSApplication").unwrap(), sharedApplication];
        let _: () = msg_send![app, activateIgnoringOtherApps: true];
    }
}

// Helper to show native macOS alert with multiple text inputs
fn show_multi_input_alert(title: &str, fields: &[(&str, &str)]) -> Option<Vec<String>> {
    activate_app(); // Bring app to front!

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];

        let title_ns = NSString::alloc(nil).init_str(title);
        let _: () = msg_send![alert, setMessageText: title_ns];

        // Create a container view for all fields
        let container: id = msg_send![Class::get("NSView").unwrap(), alloc];
        let height = (fields.len() as f64) * 50.0;
        let container: id = msg_send![container, initWithFrame: NSRect::new(
            NSPoint::new(0., 0.),
            NSSize::new(350., height)
        )];

        let mut text_fields = Vec::new();

        for (i, (label, placeholder)) in fields.iter().enumerate() {
            let y = height - ((i + 1) as f64 * 50.0);

            // Label
            let label_view: id = msg_send![Class::get("NSTextField").unwrap(), alloc];
            let label_view: id = msg_send![label_view, initWithFrame: NSRect::new(
                NSPoint::new(0., y + 20.),
                NSSize::new(350., 20.)
            )];
            let label_ns = NSString::alloc(nil).init_str(label);
            let _: () = msg_send![label_view, setStringValue: label_ns];
            let _: () = msg_send![label_view, setBezeled: false];
            let _: () = msg_send![label_view, setDrawsBackground: false];
            let _: () = msg_send![label_view, setEditable: false];
            let _: () = msg_send![label_view, setSelectable: false];
            let _: () = msg_send![container, addSubview: label_view];

            // Input field
            let text_field = NSTextField::alloc(nil);
            let text_field: id = msg_send![text_field, initWithFrame: NSRect::new(
                NSPoint::new(0., y),
                NSSize::new(350., 24.)
            )];
            let placeholder_ns = NSString::alloc(nil).init_str(placeholder);
            let _: () = msg_send![text_field, setPlaceholderString: placeholder_ns];
            let _: () = msg_send![container, addSubview: text_field];

            text_fields.push(text_field);
        }

        let _: () = msg_send![alert, setAccessoryView: container];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Submit")];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Cancel")];

        // Remove the icon and make it appear on top
        let _: () = msg_send![alert, setIcon: nil];
        let _: () = msg_send![alert, layout];

        let response: isize = msg_send![alert, runModal];

        if response == 1000 {
            // NSAlertFirstButtonReturn
            let mut results = Vec::new();
            for text_field in text_fields {
                let value: id = msg_send![text_field, stringValue];
                let bytes: *const u8 = msg_send![value, UTF8String];
                let len: usize = msg_send![value, lengthOfBytesUsingEncoding: 4];

                if !bytes.is_null() {
                    let slice = std::slice::from_raw_parts(bytes, len);
                    results.push(String::from_utf8_lossy(slice).to_string());
                } else {
                    results.push(String::new());
                }
            }
            return Some(results);
        }

        None
    }
}

fn show_alert(title: &str, message: &str) {
    activate_app(); // Bring app to front!

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];

        let title_ns = NSString::alloc(nil).init_str(title);
        let message_ns = NSString::alloc(nil).init_str(message);
        let _: () = msg_send![alert, setMessageText: title_ns];
        let _: () = msg_send![alert, setInformativeText: message_ns];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("OK")];

        // Remove the icon
        let _: () = msg_send![alert, setIcon: nil];

        let _: isize = msg_send![alert, runModal];
    }
}

fn create_template_icon() -> tray_icon::Icon {
    // Create a 22x22 template icon with a THICCC runner (note the 3 Cs)
    // Template icons should be black with transparency - macOS will colorize
    let size = 22;
    let mut rgba = vec![0u8; size * size * 4];

    // Draw a THICCC runner stick figure - MAXIMUM VISIBILITY!
    // Now with respectful leg positioning (no manspreading!)
    let runner_pixels = vec![
        // Head (THICCC circle)
        (9, 2),
        (10, 2),
        (11, 2),
        (12, 2),
        (13, 2),
        (8, 3),
        (9, 3),
        (10, 3),
        (11, 3),
        (12, 3),
        (13, 3),
        (14, 3),
        (8, 4),
        (9, 4),
        (10, 4),
        (11, 4),
        (12, 4),
        (13, 4),
        (14, 4),
        (8, 5),
        (9, 5),
        (10, 5),
        (11, 5),
        (12, 5),
        (13, 5),
        (14, 5),
        (9, 6),
        (10, 6),
        (11, 6),
        (12, 6),
        (13, 6),
        // Body (THICCC trunk)
        (9, 7),
        (10, 7),
        (11, 7),
        (12, 7),
        (13, 7),
        (9, 8),
        (10, 8),
        (11, 8),
        (12, 8),
        (13, 8),
        (9, 9),
        (10, 9),
        (11, 9),
        (12, 9),
        (13, 9),
        (9, 10),
        (10, 10),
        (11, 10),
        (12, 10),
        (13, 10),
        (9, 11),
        (10, 11),
        (11, 11),
        (12, 11),
        (13, 11),
        // Arms (THICCC running pose)
        // Front arm (forward)
        (6, 9),
        (7, 9),
        (8, 9),
        (5, 10),
        (6, 10),
        (7, 10),
        (8, 10),
        (5, 11),
        (6, 11),
        (7, 11),
        // Back arm (back)
        (14, 10),
        (15, 10),
        (16, 10),
        (14, 11),
        (15, 11),
        (16, 11),
        (17, 11),
        (15, 12),
        (16, 12),
        (17, 12),
        // Legs (THICCC but RESPECTFUL - closer together!)
        // Front leg (forward, slightly left of center)
        (8, 12),
        (9, 12),
        (10, 12),
        (7, 13),
        (8, 13),
        (9, 13),
        (6, 14),
        (7, 14),
        (8, 14),
        (5, 15),
        (6, 15),
        (7, 15),
        (5, 16),
        (6, 16),
        (7, 16),
        (5, 17),
        (6, 17),
        // Back leg (back, slightly right of center)
        (11, 12),
        (12, 12),
        (13, 12),
        (12, 13),
        (13, 13),
        (14, 13),
        (13, 14),
        (14, 14),
        (15, 14),
        (14, 15),
        (15, 15),
        (16, 15),
        (14, 16),
        (15, 16),
        (16, 16),
        (15, 17),
        (16, 17),
    ];

    for (x, y) in runner_pixels {
        if x < size && y < size {
            let idx = (y * size + x) * 4;
            rgba[idx] = 0; // R
            rgba[idx + 1] = 0; // G
            rgba[idx + 2] = 0; // B
            rgba[idx + 3] = 255; // A (fully opaque)
        }
    }

    tray_icon::Icon::from_rgba(rgba, size as u32, size as u32).unwrap()
}

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
        let _: () = msg_send![alert, setIcon: nil];

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
    activate_app(); // Bring app to front!

    let current = prefs.lock().unwrap().clone();

    let fields = vec![
        ("Name:", &current.name as &str),
        ("Email:", &current.email),
        ("API Key:", &current.api_key),
        ("Jira URL:", &current.jira_url),
    ];

    // Create dialog with pre-filled values
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];

        let title_ns = NSString::alloc(nil).init_str("Preferences ⚙️");
        let _: () = msg_send![alert, setMessageText: title_ns];

        let container: id = msg_send![Class::get("NSView").unwrap(), alloc];
        let height = (fields.len() as f64) * 50.0;
        let container: id = msg_send![container, initWithFrame: NSRect::new(
            NSPoint::new(0., 0.),
            NSSize::new(400., height)
        )];

        let mut text_fields = Vec::new();

        for (i, (label, value)) in fields.iter().enumerate() {
            let y = height - ((i + 1) as f64 * 50.0);

            // Label
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

            // Input field with current value
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

        let _: () = msg_send![alert, setAccessoryView: container];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Save")];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("Cancel")];

        // Remove the icon
        let _: () = msg_send![alert, setIcon: nil];

        let response: isize = msg_send![alert, runModal];

        if response == 1000 {
            // Save clicked
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

            // Save to file
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

    let event_loop = EventLoop::new().unwrap();

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

    let _ = event_loop.run(move |_event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

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
