extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGTH: u32 = 600;

fn main() {
    // glfw: initialize and configure.
    let mut glfw = glfw::init(error_callback).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    // glfw window creation
    let (mut window, events) = glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGTH, "LearnOpenGl", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    
    // gl: load all OpenGl functions pointers.
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    // render loop
    while !window.should_close() {
        // events
        process_events(&mut window, &events);
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn error_callback(err: glfw::Error, description: String) {
    println!("GLFW error {}: {}.", err, description);
}

fn process_events(window: &mut glfw::Window, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, heigth) => {
                // make sure the viewport matches the new window dimensions; note that width and height will be significantly larger than specified on retina displays
                unsafe { gl::Viewport(0, 0, width, heigth)}
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
