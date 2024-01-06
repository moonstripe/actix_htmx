use std::process::Command;

fn main() {
    let _ = Command::new("npx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("./static/main.css")
        .arg("-o")
        .arg("./static/tailwind.css")
        .output();
}
