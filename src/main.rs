use std::{fs, thread, time::Duration, process::Command, path::Path }; 
fn main() {
    let dir = std::env::current_exe()
        .expect("Failed to get exe")
        .parent()
        .expect("failed to get path")
        .to_path_buf();
    println!("{}",dir.display());

    let icon = dir.join("lowBattery.jpg");
    let sound = dir.join("Google-notification.mp3");

    // Temporary
    //let status_path = dir.join("status");
    //let capacity_path = dir.join("capacity"); 

    let mut notified_low = false; 
    let mut notified_critical = false; 
    let mut notified_full = false; 

    loop {
        let status = fs::read_to_string("/sys/class/power_supply/BAT1/status")
            .unwrap_or_default()
            .trim()
            .to_string();
        let capacity: u32 = fs::read_to_string("/sys/class/power_supply/BAT1/capacity")
            .unwrap_or_default()
            .trim()
            .parse()
            .unwrap_or(0);

        match (status.as_str(), capacity) {
            ("Discharging", 10..=20) => {
                if !notified_low {
                    notify("~Oni-Chan!", &format!("Your battery is low at {}", capacity), &icon, &sound);
                    notified_low = true;
                    notified_critical = false;
                    notified_full = false; 
                }
            }
            ("Discharging", 0..=9) => {
                if !notified_critical {
                    notify("~Oni-chan! I'm really hungry!", "Please feed me!",&icon, &sound);
                    notified_low = false; 
                    notified_critical = true; 
                    notified_full = false; 
                }
            }
            ("Discharging", 95..) => {
                if !notified_full {
                    notify("~Oni-chan!", "I'm filled!", &icon, &sound);
                    notified_low = false; 
                    notified_critical = false; 
                    notified_full = true; 
                }
            }

            _ => {
                // Reset flags
                notified_low = false; 
                notified_critical = false; 
                notified_full = false; 
            }
        }
            thread::sleep(Duration::from_secs(1));
    }
}
                 


fn notify(summary: &str, body: &str, icon: &Path, sound: &Path) { 
    Command::new("notify-send")
        .arg(summary)
        .arg(body)
        .arg("--icon")
        .arg(icon)
        .spawn();

    Command::new("ffplay")
        .args(["-nodisp", "-autoexit", "-volume", "20", "-loglevel", "quiet"])
        .arg(sound)
        .spawn();


}
