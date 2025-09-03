use gtk4::{prelude::*, Application, ApplicationWindow, Box as GtkBox, DrawingArea, Orientation, Overlay, Label, Stack, CssProvider, glib};
use gtk4::gdk::Display;
use gtk4_layer_shell::{LayerShell, Layer, Edge};
use std::cell::RefCell;
use std::rc::Rc;
use std::process::{Command, exit};

fn main() {
    let app = Application::builder()
        .application_id("ekah.scu.capsulator")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

// fn typing_effect(label: &Label, text: &str, delay_ms: u64) {
//     let label = label.clone();
//     let chars: Vec<char> = text.chars().collect();
//     let index = Rc::new(RefCell::new(0));
//     let chars_rc = Rc::new(chars);

//     glib::timeout_add_local(std::time::Duration::from_millis(delay_ms), move || {
//         let i = *index.borrow();
//         if i < chars_rc.len() {
//             let current_text = chars_rc.iter().take(i + 1).collect::<String>();
//             label.set_markup(&current_text);
//             *index.borrow_mut() += 1;
//             glib::ControlFlow::Continue
//         } else {
//             glib::ControlFlow::Break
//         }
//     });
// }

// fn typing_effect_normal(label: &Label, text: &str, delay_ms: u64) {
//     let label = label.clone();
//     let chars: Vec<char> = text.chars().collect();
//     let index = Rc::new(RefCell::new(0));
//     let chars_rc = Rc::new(chars);

//     glib::timeout_add_local(std::time::Duration::from_millis(delay_ms), move || {
//         let i = *index.borrow();
//         if i < chars_rc.len() {
//             let current_text = chars_rc.iter().take(i + 1).collect::<String>();
//             label.set_text(&current_text);
//             *index.borrow_mut() += 1;
//             glib::ControlFlow::Continue
//         } else {
//             glib::ControlFlow::Break
//         }
//     });
// }

fn scan_networks() -> Vec<String> {
    let output = Command::new("nmcli")
        .args(&["-t", "-f", "SSID", "device", "wifi", "list"])
        .output()
        .expect("failed to execute nmcli");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    stdout.lines()
        .skip(1)
        .filter_map(|line| line.split_whitespace().next())
        .map(|ssid| ssid.to_string())
        .collect()
}


fn draw_circle_progress(cr: &gtk4::cairo::Context, percent: f64) {
    let w = 40.0;
    let h = 40.0;
    let radius = 10.0;
    let center_x = w / 2.0;
    let center_y = h / 2.0;

    // Background circle
    cr.set_source_rgba(0.2, 0.2, 0.2, 0.4);
    cr.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);
    cr.set_line_width(4.0);
    let _ = cr.stroke();

    // Progress arc
    cr.set_source_rgba(255.0/255.0, 71.0/255.0, 71.0/255.0, 1.0);
    let angle = percent / 100.0 * 2.0 * std::f64::consts::PI;
    cr.arc(center_x, center_y, radius, -std::f64::consts::PI / 2.0, angle - std::f64::consts::PI / 2.0);
    let _ = cr.stroke();

    // Percentage text
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.select_font_face("Sans", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Bold);
    cr.set_font_size(12.0);
    let text = format!("{:.0}%", percent);
    let extents = cr.text_extents(&text).unwrap();
    cr.move_to(
        center_x - extents.width() / 2.0,
        center_y + extents.height() / 2.0,
    );
}



fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.init_layer_shell();
    window.set_layer(Layer::Background);
    window.auto_exclusive_zone_enable();
    window.fullscreen();
    window.set_decorated(false);
    window.set_namespace(Some("welp"));

    for (edge, anchor) in [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, true),
    ] {
        window.set_anchor(edge, anchor);
    }

    let css = CssProvider::new();
    css.load_from_data(
        "
        window {
            background-color: rgba(0, 0, 0, 0);
        }

        #cynide {
            border-radius: 50px;
            border: 1px solid rgba(255, 255, 255, 0.12);
            background-color: rgba(19, 19, 19, 0.2);
            padding: 10px 30px 30px 30px;
            opacity: 1.0;
        }

        @keyframes retro-glow-in {
            0% {
                opacity: 0;
                text-shadow:
                    0 0 8px #11eeee,
                    0 0 20px #11bbbb,
                    0 0 36px #11aaaa,
                    0 0 64px #55ffff,
                    0 0 96px #aaffff;
            }
            50% {
                opacity: 0.7;
                text-shadow:
                    0 0 16px #55ffff,
                    0 0 36px #11eeee,
                    0 0 56px #aaffff,
                    0 0 80px #55ffff;
            }
            80% {
                opacity: 1;
                text-shadow:
                    0 0 28px #aaffff,
                    0 0 48px #55ffff,
                    0 0 68px #22dddd,
                    0 0 96px #11aaaa;
            }
            100% {
                opacity: 1;
                text-shadow:
                    0 0 4px  #aaffff,
                    0 0 8px  #55ffff,
                    0 0 12px #22dddd,
                    0 0 24px #11aaaa,
                    0 0 36px #11bbbb,
                    0 0 48px #11eeee;
            }
        }

        @keyframes retro-glow-loop {
            0% {
                opacity: 1;
                text-shadow:
                    0 0 4px  #aaffff,
                    0 0 8px  #55ffff,
                    0 0 12px #22dddd,
                    0 0 24px #11aaaa,
                    0 0 36px #11bbbb,
                    0 0 48px #11eeee;
            }
            50% {
                opacity: 1;
                text-shadow:
                    0 0 28px #aaffff,
                    0 0 48px #55ffff,
                    0 0 68px #22dddd,
                    0 0 96px #11aaaa;
            }
            80% {
                opacity: 0.7;
                text-shadow:
                    0 0 16px #55ffff,
                    0 0 36px #11eeee,
                    0 0 56px #aaffff,
                    0 0 80px #55ffff;
            }
            100% {
                opacity: 0;
                text-shadow:
                    0 0 8px #11eeee,
                    0 0 20px #11bbbb,
                    0 0 36px #11aaaa,
                    0 0 64px #55ffff,
                    0 0 96px #aaffff;
            }
        }

        label.eye {
            letter-spacing: 1px;
            line-height: 0.5;
            font-size: 12px;
            color: #aaffff;
            font-weight: bold;
            text-shadow:
                0 0 4px  #aaffff,
                0 0 8px  #55ffff,
                0 0 12px #22dddd,
                0 0 24px #11aaaa,
                0 0 36px #11bbbb,
                0 0 48px #11eeee;
            background: transparent;
            padding: 2px 16px;
            border-radius: 9px;
            font-family: 'Orbitron', 'Segoe UI', monospace;
            opacity: 0;
            animation: 
                retro-glow-in 2s cubic-bezier(0.6,0,0.4,1) forwards;
        }

        label.aftereye {
            letter-spacing: 1px;
            line-height: 0.5;
            font-size: 12px;
            color: #aaffff;
            font-weight: bold;
            text-shadow:
                0 0 4px  #aaffff,
                0 0 8px  #55ffff,
                0 0 12px #22dddd,
                0 0 24px #11aaaa,
                0 0 36px #11bbbb,
                0 0 48px #11eeee;
            background: transparent;
            padding: 2px 16px;
            border-radius: 9px;
            font-family: 'Orbitron', 'Segoe UI', monospace;
            opacity: 1;
            animation:             
                retro-glow-loop 2s cubic-bezier(0.6,0,0.4,1) forwards;
        }

        #heading {
            font-size: 28px;
            font-weight: 900;
        }

        #mainbox {
            border-radius: 25px;
            border: 1px solid rgba(255, 255, 255, 0.16);
            background-color: rgba(40, 54, 45, 0.32);
            padding: 30px 30px 30px 30px;
        }

        #inbox {
            border-radius: 25px;
            border: 1px solid rgba(255, 255, 255, 0.16);
            background-color: rgba(49, 49, 49, 0.56);
            padding: 30px 30px 30px 30px;
        }

        #progressbox{
            border-radius: 50px;
            border: 1px solid rgba(255, 255, 255, 0.16);
            background-color: rgba(255, 255, 255, 0.32);
            padding: 0px 0px 0px 10px;
            box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
        }

        ",
    );

    gtk4::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.set_hexpand(true);
    main_box.set_vexpand(true);
    main_box.set_widget_name("mainbox");
    main_box.set_valign(gtk4::Align::Fill);
    main_box.set_halign(gtk4::Align::Fill);
    main_box.set_margin_bottom(20);
    main_box.set_margin_end(20);
    main_box.set_margin_start(20);
    main_box.set_margin_top(20);
    main_box.set_visible(false);

    let stack = Stack::builder()
        .hexpand(true)
        .vexpand(true)
        .transition_type(gtk4::StackTransitionType::Crossfade)
        .build();
    // let click = Rc::new(Cell::new(0));
    let stack_box = GtkBox::new(Orientation::Horizontal, 5);
    stack_box.set_hexpand(true);
    stack_box.set_vexpand(true);

    let wifibox = GtkBox::new(Orientation::Vertical, 5);
    wifibox.set_widget_name("inbox");
    wifibox.set_vexpand(false);
    wifibox.set_hexpand(false);
    wifibox.set_size_request(500, 500);
    wifibox.set_valign(gtk4::Align::Center);
    wifibox.set_halign(gtk4::Align::Center);
    wifibox.append(&Label::builder()
        .name("heading")
        .label("Internet is Required")
        .justify(gtk4::Justification::Left)
        .build()
    );
    stack.add_named(&wifibox, Some("wifi"));
    stack.set_visible_child_name("wifi");

    let networks = scan_networks();
    for ssid in networks {
        wifibox.append(&Label::new(Some(&ssid)));
    }
    
    stack_box.append(&stack);
    main_box.append(&stack_box);
    
    let progressbox = GtkBox::new(Orientation::Horizontal, 5);
    progressbox.set_widget_name("progressbox");
    progressbox.set_vexpand(true);
    progressbox.set_hexpand(true);
    progressbox.set_valign(gtk4::Align::End);
    progressbox.set_halign(gtk4::Align::Center);
    progressbox.append(&Label::new(Some("Till cynageOS")));

    let drawing_area = DrawingArea::builder()
        .content_width(40)
        .content_height(40)
        .build();

    drawing_area.set_draw_func(move |_, cr, _, _| {
        draw_circle_progress(cr, 0.0);
    });

    progressbox.append(&drawing_area);
    main_box.append(&progressbox);

    let overlay = Overlay::new();

    let eye = Label::new(Some("
  ⢠⣿⣯⠦⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠈⠂⠀⠀⠀⠀⠀⠀⠀⠑⠐⠀⠀⠀⠀⠀⠀⠸⡀ \n
  ⡿⣭⡦⠗⠁⠄⠂⠀⠀⠀⠀⠀⡠⣰⢀⠀⠀⠀⢰⠋⡆⢀⢠⠀⠀⠀⠀⠀⠐⢆⠀⢂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠁⠀\n
 ⠘⣼⣎⠕⠊⠁⠀⠀⠀⢢⠆⡀⠬⡑⢿⣻⡆⠀⡀⡄⠄⣧⢸⡈⢀⠀⡆⢠⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⡇⠸⡩⡠⡔⢱⢀⠰⣄⠔⠁⣻⣢⢙⣿⣼⣿⣷⠴⠿⣿⡗⣟⣿⡿⣷⣾⣤⣼⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠁⠈⠔⢱⢌⢿⢢⠑⠻⣗⠎⣀⣿⣟⢛⣍⣯⣿⣧⣤⣿⣧⣿⣿⣵⣾⣿⣎⡹⠿⣿⣶⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⠀⠀⢾⢱⣷⣷⡢⢾⣷⢯⣽⣽⣿⣿⠿⣿⣛⡿⠯⠿⠿⠿⡿⠿⣿⣿⣿⣿⣿⣿⣽⣟⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⢀⠑⢬⣧⣻⣽⣽⣿⣿⣿⣿⢟⣻⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠿⣽⢿⡙⢿⣿⣿⣇⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⠈⠳⢜⢿⣿⣿⢿⣿⣿⡿⣩⠋⠄⠀⠀⠀⠀⠀⣀⣠⣤⣤⣤⣤⣄⡀⠀⠀⠈⠻⣮⡟⠙⠹⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⢀⢀⡀⠉⢟⡻⢛⣿⠿⡷⠁⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠹⣿⣷⣦⣱⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⠀⠉⠚⣋⠶⣋⡵⢏⣰⠁⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠀⠈⢿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀\n
⠀⠀⠀⢬⣷⣶⣽⣿⣦⡉⢡⠀⠀⠀⠀⠀⣾⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣷⢄⡀⠀⠀\n
⠀⠀⠀⡨⠟⠉⠉⣉⠻⣿⡌⢆⠀⠀⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⢔⣿⣽⣿⣿⣿⣿⣿⣤⠑⢶⡄\n
⠀⠀⠐⠁⠀⢠⡪⠒⣚⣻⣶⣄⠳⣠⠀⠀⠈⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠐⣾⣿⣿⣿⣿⣿⣿⣿⣿⣱⣼⣯\n
⠀⠀⠀⢀⣔⢡⡴⢛⣳⡼⠿⢿⣧⣬⣑⠤⣀⡀⠉⠻⢿⣿⣿⣿⣿⣿⠟⠋⠀⠀⠀⣀⣀⣤⣾⣿⣿⣿⡿⣿⣿⠿⠿⢿⢿⡿⠠\n
⠀⠀⠀⠉⠊⡝⠨⠋⠀⢀⡤⣾⣟⡻⣿⢷⣶⣬⣭⣐⣤⣄⢀⣈⣀⠀⡠⢄⡦⣤⡛⠩⣿⢛⣻⢿⢛⡼⠾⠝⡅⠭⠪⠴⠋⠀⠀\n
⠀⠀⠀⠀⠀⠀⠀⢠⠖⢛⢜⡩⠔⠋⣉⢔⠟⢪⡿⣫⠛⢿⣿⣿⡧⠉⣿⠎⠺⣾⠁⠃⣻⠑⠠⠂⠑⢒⢁⠤⠐⡄⠉⠀⠀⠀⠀\n
⠀⠀⠀⠀⠀⠀⠐⠁⠀⠉⠁⠀⠀⣪⠼⠃⢠⠿⠈⢼⢀⣾⠯⢿⠂⢑⢸⠢⠂⠃⠀⠀⠐⡘⠄⢠⠔⠓⢙⡥⠋⠀⠀⠀⠀⠀⠀\n
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠉⠀⠀⠇⠀⠃⠀⠘⢀⢉⠁⢀⢀⠀⡀⠀⠀⢔⠺⢽⠪⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⡀⢀⡅⠤⠀⠈⢤⠐⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡐⠈⠰⠓⢀⠄⠇⠺⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠁⠀⠀⠀⠴⠠⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"));
    eye.set_css_classes(&["eye"]);
    eye.is_selectable();
    overlay.add_overlay(&main_box);
    overlay.set_child(Some(&eye));
    eye.set_margin_top(50);

    eye.set_halign(gtk4::Align::Center);
    eye.set_valign(gtk4::Align::Center);

    let main_box_clone = main_box.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(4), move || {
        eye.remove_css_class("eye");
        eye.add_css_class("aftereye");
        let main_box_clone_inner = main_box_clone.clone();
        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            main_box_clone_inner.set_visible(true);
            glib::ControlFlow::Break
        });
        glib::ControlFlow::Break
    });

    window.set_child(Some(&overlay));
    window.show();

}