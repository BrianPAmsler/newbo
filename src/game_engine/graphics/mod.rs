extern crate glfw;

use glfw::{FlushedMessages, WindowEvent, Context};

use gl33::global_loader::*;
use gl33::gl_enumerations::*;

use core::panic;
use std::mem::MaybeUninit;
use std::sync::mpsc::Receiver;

use std::sync::atomic::{Ordering, AtomicBool};

use core::mem::{size_of};

use libc::strlen;

mod shader;
use shader::*;

static mut GLFW: MaybeUninit<glfw::Glfw> = MaybeUninit::uninit();

static GL_INITIALIZED: AtomicBool = AtomicBool::new(false);

// Include shaders
const VERT_SHADER: &'static str = include_str!("shaders/terrain_shader.vert");
const FRAG_SHADER: &'static str = include_str!("shaders/terrain_shader.frag");

pub type TerrainVertex = [f32; 6];

pub struct Graphics {
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    terrain_shader: Shader,
    sprite_shader: Shader,
    sprite_vbo: u32,
    sprite_vao: u32,
    terrain_vbo: u32,
    terrain_vao: u32
}

fn convert_string<'a>(s: *const u8) -> &'a str {
    unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(s, strlen(s as *const i8))) }
}

impl Graphics {
    pub fn init_gl() {
        if GL_INITIALIZED.load(Ordering::Relaxed) {
            // Maybe this should panic, I'm not really sure
            eprintln!("GL Already Initialized!");
            return;
        }
    
        unsafe {GLFW.write(glfw::init(glfw::FAIL_ON_ERRORS).unwrap())};
    
        GL_INITIALIZED.store(true, Ordering::Relaxed);
    }

    pub fn gl_initialized() -> bool {
        GL_INITIALIZED.load(Ordering::Relaxed)
    }

    pub fn get_gl_time(time: &mut i64) {
        unsafe { glGetInteger64v(GL_TIMESTAMP, time) };
    }

    pub fn create_window() -> Graphics {
        if !Graphics::gl_initialized() {
            panic!("GL not initialized!");
        }

        let glfw = unsafe {GLFW.assume_init_mut()};
     
        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw.create_window(800, 600, "Wob", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
    
        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);

        let mut gfx = Graphics {window: window,
            events: events,
            terrain_shader: Shader::null_shader(),
            sprite_shader: Shader::null_shader(),
            sprite_vbo: 0,
            sprite_vao: 0,
            terrain_vbo: 0,
            terrain_vao: 0
        };
    
        // The get_proc_address shouldn't be mutable as far as I can tell, but for some reason it is. There's an unsafe block anyway so why not.
        let win_ptr = &mut gfx.window as *mut glfw::Window;
        // Do gl stuff
        unsafe {
            load_global_gl(&|fn_name: *const u8| (*win_ptr).get_proc_address(convert_string(fn_name)));
            gfx.terrain_shader = Shader::load_shader_program("Terrain Shader", VERT_SHADER, FRAG_SHADER);
    
            glClearColor(0.2, 0.3, 0.3, 1.0);
    
            gfx.terrain_shader = Shader::load_shader_program("Default Shader", VERT_SHADER, FRAG_SHADER);
    
            let mut vao: u32 = 0;
            glGenVertexArrays(1, &mut vao);
            if (vao as i32) < 0 {
                panic!("Error creaing VAO!");
            }
            
            glBindVertexArray(vao);
    
            let mut vbo: u32 = 0;
            glGenBuffers(1, &mut vbo);
            if (vbo as i32) < 0 {
                panic!("Error creaing VBO!");
            }
            
            gfx.terrain_vao = vao;
            gfx.terrain_vbo = vbo;
    
            glBindBuffer(GL_ARRAY_BUFFER, vbo as u32);
    
            // Enable pos attribute pointer
            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                0,
                24,
                0 as *const _,
            );
            glEnableVertexAttribArray(0);
    
            // Enable color attribute pointer
            glVertexAttribPointer(
                1,
                3,
                GL_FLOAT,
                0,
                24,
                12 as *const _,
            );
            glEnableVertexAttribArray(1);
    
            // Create Sprite VAO
            let mut vao: u32 = 0;
            glGenVertexArrays(1, &mut vao);
            if (vao as i32) < 0 {
                panic!("Error creaing VAO!");
            }
            
            glBindVertexArray(vao);
    
            let mut vbo: u32 = 0;
            glGenBuffers(1, &mut vbo);
            if (vbo as i32) < 0 {
                panic!("Error creaing VBO!");
            }
    
            gfx.sprite_vao = vao;
            gfx.sprite_vbo = vbo;
            
            glBindBuffer(GL_ARRAY_BUFFER, vbo as u32);
    
            // Enable pos attribute pointer
            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                0,
                20,
                0 as *const _,
            );
            glEnableVertexAttribArray(0);
    
            // Enable uv attribute pointer
            glVertexAttribPointer(
                1,
                3,
                GL_FLOAT,
                0,
                20,
                12 as *const _,
            );
            glEnableVertexAttribArray(1);
    
            glBindBuffer(GL_ARRAY_BUFFER, 0);
            glBindVertexArray(0);
        }
    
        gfx
    }
    
    pub fn buffer_verticies(&mut self, verticies: &[TerrainVertex]) {
        unsafe {
            glBindVertexArray(self.terrain_vao);
            glBindBuffer(GL_ARRAY_BUFFER, self.terrain_vbo);
            
            println!("Buffering: vao = {}; vbo = {}", self.terrain_vao, self.terrain_vbo);

            // Send verticies to gpu
            glBufferData(
                GL_ARRAY_BUFFER,
                (verticies.len() * size_of::<TerrainVertex>()) as isize,
                verticies.as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            glBindBuffer(GL_ARRAY_BUFFER, 0);
            glBindVertexArray(0);
        }
    }

    pub fn render(&self, offset1: f32, offset2: f32) {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);

            glBindVertexArray(self.terrain_vao);
            glBindBuffer(GL_ARRAY_BUFFER, self.terrain_vbo);
            
            let program = self.terrain_shader.get_program();
            glUseProgram(program);
            let loc: i32 = glGetUniformLocation(program, b"offset\0" as *const u8);
        
            if loc >= 0 {
                glUniform3f(loc, offset1, 0.0, 0.0);
            }
        
            glDrawArrays(GL_TRIANGLES, 0, 3);
        
            if loc >= 0 {
                glUniform3f(loc, offset2, 0.0, 0.0);
            }
        
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
    }

    pub fn window_alive(&self) -> bool {
        !self.window.should_close()
    }

    pub fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn get_window_events(&self) -> FlushedMessages<'_, (f64, WindowEvent)> {
        let glfw = unsafe {GLFW.assume_init_mut()};

        glfw.poll_events();
        glfw::flush_messages(&self.events)
    }

    pub fn swap_window_buffers(&mut self) {
        self.window.swap_buffers();
    }
}