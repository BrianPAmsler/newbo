extern crate glfw;

use glfw::{Action, Context, Key};

use gl33::global_loader::*;
use gl33::gl_enumerations::*;

use core::panic;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;

use std::sync::atomic::{Ordering, AtomicI64};

use libc::strlen;

use core::{
    convert::TryInto,
    mem::{size_of, size_of_val},
  };

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

// Include shaders
const VERT_SHADER: &'static str = include_str!("shaders/default_shader.vert");
const FRAG_SHADER: &'static str = include_str!("shaders/default_shader.frag");

static mut WINDOW: MaybeUninit<glfw::Window> = MaybeUninit::uninit();
static mut GLFW: MaybeUninit<glfw::Glfw> = MaybeUninit::uninit();
static mut EVENTS: MaybeUninit<Receiver<(f64, glfw::WindowEvent)>> = MaybeUninit::uninit();
static mut GL_PROGRAM: MaybeUninit<u32> = MaybeUninit::uninit();

static GL_INITIALIZED: AtomicBool = AtomicBool::new(false);
static IS_RUNNING: AtomicBool = AtomicBool::new(false);

static FIXED_TICK_DURATION: AtomicI64 = AtomicI64::new(1000000000i64 / 60i64);

fn convert_string<'a>(s: *const u8) -> &'a str {
    unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(s, strlen(s as *const i8))) }
}

pub fn init_gl() {
    if GL_INITIALIZED.load(Ordering::Relaxed) {
        // Maybe this should panic, I'm not really sure
        eprintln!("GL Already Initialized!");
        return;
    }

    unsafe {GLFW.write(glfw::init(glfw::FAIL_ON_ERRORS).unwrap())};
    let glfw = unsafe {GLFW.assume_init_mut()};
 
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(800, 600, "Wob", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Do gl stuff
    unsafe {
        WINDOW.write(window);
        load_global_gl(&|fn_name: *const u8| WINDOW.assume_init_mut().get_proc_address(convert_string(fn_name)));

        glClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        
        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        
        // Send verticies to gpu
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);

        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
                                    
        glShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
            );
            
        glCompileShader(vertex_shader);

        let mut success = 0;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
                );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
                
        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);

        glShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
            );
        glCompileShader(fragment_shader);
        
        let mut success = 0;
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        GL_PROGRAM.write(glCreateProgram());
        let shader_program = GL_PROGRAM.assume_init();

        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);

        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
        
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        glUseProgram(shader_program);
    }

    GL_INITIALIZED.store(true, Ordering::Relaxed);
    unsafe {EVENTS.write(events);}
}

pub fn start_game_loop() {
    if !GL_INITIALIZED.load(Ordering::Relaxed) {
        panic!("GL not initialized!");
    }
    
    if IS_RUNNING.load(Ordering::Relaxed) {
        panic!("Game loop already running in another thread!");
    }
    IS_RUNNING.store(true, Ordering::Relaxed);

    let window = unsafe { WINDOW.assume_init_mut() };
    let events = unsafe { EVENTS.assume_init_mut() };
    let glfw = unsafe { GLFW.assume_init_mut() };

    static mut LAST_TICK: i64 = 0;
    static mut LAST_FIXED_TICK: i64 = 0;
    unsafe {
        glGetInteger64v(GL_TIMESTAMP, &mut LAST_TICK);
        LAST_FIXED_TICK = LAST_TICK;
    }
    // Loop until the user closes the window
    while !window.should_close() {
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }

        // Game tick
        unsafe {
            let mut current_time: i64 = 0;
            glGetInteger64v(GL_TIMESTAMP, &mut current_time);

            game_tick((current_time - LAST_TICK) as f32 / 1000000000f32);
            LAST_TICK = current_time;

            if current_time - LAST_FIXED_TICK >= FIXED_TICK_DURATION.load(Ordering::Relaxed) {
                fixed_game_tick((current_time - LAST_FIXED_TICK) as f32 / 1000000000f32);
                LAST_FIXED_TICK = current_time;
            }

            // Render
            render();
        }
        // Swap front and back buffers
        window.swap_buffers();
    }

    IS_RUNNING.store(false, Ordering::Relaxed);
}

// Default value is 60 tps, so this may never need to be used.
#[allow(dead_code)]
pub fn set_fixed_tick_rate(tickrate:  u32) {
    FIXED_TICK_DURATION.store(1000000000i64 / tickrate as i64, Ordering::Relaxed);
}

static mut OFFSET1: f32 = 0.0;
static mut OFFSET2: f32 = 0.0;

unsafe fn render() {
    glClear(GL_COLOR_BUFFER_BIT);
    
    let program = GL_PROGRAM.assume_init();
    let loc: i32 = glGetUniformLocation(program, b"offset\0" as *const u8);

    if loc >= 0 {
        glUniform3f(loc, OFFSET1, 0.0, 0.0);
    }

    glDrawArrays(GL_TRIANGLES, 0, 3);

    if loc >= 0 {
        glUniform3f(loc, OFFSET2, 0.0, 0.0);
    }

    glDrawArrays(GL_TRIANGLES, 0, 3);
}

fn game_tick(delta_time: f32) {
    unsafe {
        OFFSET1 += 0.1 * delta_time;
    }
}

fn fixed_game_tick(delta_time: f32) {
    unsafe {
        OFFSET2 -= 0.1 * delta_time;
    }
}