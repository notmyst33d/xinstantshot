use arboard::Clipboard;
use arboard::ImageData;
use chrono::Datelike;
use chrono::Local;
use chrono::Timelike;
use home::home_dir;
use image::DynamicImage;
use image::RgbImage;
use notify_rust::Notification;
use std::path::Path;
use x11_screenshot::Screen;

enum Mode {
    FILE,
    CLIPBOARD,
}

fn send_notification(text: String) {
    Notification::new()
        .summary("xinstantshot")
        .body(&text)
        .show()
        .unwrap();
}

fn save_file(image: RgbImage, base_path: &Path) {
    let time = Local::now();
    let mut collision_counter = 0;
    let get_path = |collision_counter| {
        format!(
            "{}/Pictures/Screenshots/{}-{}-{}_{}-{}-{}_{}.png",
            base_path.to_str().unwrap(),
            time.day(),
            time.month(),
            time.year(),
            time.hour(),
            time.minute(),
            time.second(),
            collision_counter,
        )
    };

    let mut path = get_path(collision_counter);
    while Path::new(&path).exists() {
        collision_counter += 1;
        path = get_path(collision_counter);
    }

    image.save(&path).unwrap();

    send_notification(format!("Screenshot saved to {}", path));
}

fn save_clipboard(image: RgbImage) {
    Clipboard::new()
        .unwrap()
        .set_image(ImageData {
            width: image.width() as usize,
            height: image.height() as usize,
            bytes: DynamicImage::ImageRgb8(image)
                .into_rgba8()
                .into_raw()
                .into(),
        })
        .unwrap();
    send_notification("Screenshot saved to clipboard".into());
}

fn main() {
    let image = Screen::open().unwrap().capture().unwrap();
    let mut mode = Mode::FILE;
    let home = home_dir().unwrap();
    let mut path = home.as_path();

    let args = std::env::args().collect::<Vec<String>>();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-p" => {
                let value = iter.next().unwrap();
                if !value.starts_with("-") {
                    path = Path::new(value);
                }
            }
            "-c" => mode = Mode::CLIPBOARD,
            _ => {}
        }
    }

    match mode {
        Mode::FILE => save_file(image, path),
        Mode::CLIPBOARD => save_clipboard(image),
    };
}
