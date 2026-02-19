fn main() {
    // This relative path is correct based on your repo structure
    slint_build::compile("../native/ui/app-window.slint").expect("Slint build failed");
}