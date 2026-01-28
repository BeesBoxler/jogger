pub fn create_template_icon() -> tray_icon::Icon {
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
        // Legs (THICCC but RESPECTFUL)
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
            rgba[idx] = 0;
            rgba[idx + 1] = 0;
            rgba[idx + 2] = 0;
            rgba[idx + 3] = 255;
        }
    }

    tray_icon::Icon::from_rgba(rgba, size as u32, size as u32).unwrap()
}
