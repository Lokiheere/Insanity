mod resources;

fn main() {
    let mut window = resources::win::Window::new(1920, 1080, "My Window"); // replace with your desired width, height, and title
    window.run();
}