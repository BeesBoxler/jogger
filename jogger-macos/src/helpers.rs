use cocoa::appkit::NSTextField;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use objc::runtime::Class;
use objc::{msg_send, sel, sel_impl};
use dispatch::Queue;

pub fn activate_app() {
    unsafe {
        let app: id = msg_send![Class::get("NSApplication").unwrap(), sharedApplication];
        let _: () = msg_send![app, activateIgnoringOtherApps: true];
    }
}

pub fn show_alert_on_main_thread(title: String, message: String) {
    Queue::main().exec_async(move || {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let app: id = msg_send![Class::get("NSApplication").unwrap(), sharedApplication];
            let _: () = msg_send![app, activateIgnoringOtherApps: true];

            let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
            let alert: id = msg_send![alert, init];
            let _: () = msg_send![alert, setAlertStyle: 1];

            let title_ns = NSString::alloc(nil).init_str(&title);
            let message_ns = NSString::alloc(nil).init_str(&message);
            let _: () = msg_send![alert, setMessageText: title_ns];
            let _: () = msg_send![alert, setInformativeText: message_ns];
            let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("OK")];

            let _: isize = msg_send![alert, runModal];
        }
    });
}

pub fn show_alert(title: &str, message: &str) {
    activate_app();

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];
        let _: () = msg_send![alert, setAlertStyle: 1];

        let title_ns = NSString::alloc(nil).init_str(title);
        let message_ns = NSString::alloc(nil).init_str(message);
        let _: () = msg_send![alert, setMessageText: title_ns];
        let _: () = msg_send![alert, setInformativeText: message_ns];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("OK")];

        let _: isize = msg_send![alert, runModal];
    }
}

pub fn show_single_input_alert(title: &str, label: &str, placeholder: &str) -> Option<String> {
    show_multi_input_alert(title, &[(label, placeholder)]).and_then(|v| v.into_iter().next())
}

pub fn show_multi_input_alert(title: &str, fields: &[(&str, &str)]) -> Option<Vec<String>> {
    activate_app();

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![Class::get("NSAlert").unwrap(), alloc];
        let alert: id = msg_send![alert, init];
        let _: () = msg_send![alert, setAlertStyle: 1];

        let title_ns = NSString::alloc(nil).init_str(title);
        let _: () = msg_send![alert, setMessageText: title_ns];

        let height = (fields.len() as f64 * 50.0) + 20.0;
        let container: id = msg_send![Class::get("NSView").unwrap(), alloc];
        let container: id = msg_send![container, initWithFrame: NSRect::new(
            NSPoint::new(0., 0.),
            NSSize::new(350., height)
        )];

        let mut text_fields = Vec::new();

        for (i, (label, placeholder)) in fields.iter().enumerate() {
            let y = height - ((i + 1) as f64 * 50.0);

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
        let _: () = msg_send![alert, layout];

        let response: isize = msg_send![alert, runModal];

        if response == 1000 {
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
