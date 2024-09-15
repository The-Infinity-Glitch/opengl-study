extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

// Window settings.
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(init_error_callback).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGl", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    while !window.should_close() {
        process_events(&mut window, &events);
        
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        window.swap_buffers();
        glfw.poll_events()
    }
}

fn init_error_callback(err: glfw::Error, description: String) {
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