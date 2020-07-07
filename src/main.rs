extern crate gl;
extern crate glfw;

use glfw::{ Action, Context, Key, WindowHint, OpenGlProfileHint };

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn main() {
    let mut _glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // set context version
    _glfw.window_hint(WindowHint::ContextVersion(3, 3));
    _glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // create window
    let (mut window, events) = _glfw.create_window(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, "Learn OpenGL", glfw::WindowMode::Windowed)
        .expect("failed to create window");

    // load gl function pointers
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    // make window's context the current one
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    unsafe { gl::Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT); }

    // main program loop
    while !window.should_close() {
        window.swap_buffers();
        _glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height); }
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }

    unsafe { glfw::ffi::glfwTerminate(); }
}
