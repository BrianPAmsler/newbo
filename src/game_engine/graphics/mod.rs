extern crate glfw;

use glfw::{FlushedMessages, WindowEvent, Context};

use gl33::global_loader::*;
use gl33::gl_enumerations::*;
use missing_gl_enums::*;

use std::fs::File;
use std::io::BufReader;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::sync::mpsc::Receiver;

use std::sync::atomic::{Ordering, AtomicBool};

use core::mem::{size_of};

use libc::strlen;

mod shader;
use shader::*;

use super::Sprite;
use super::err::{EngineError, EngineErrorTrait};

mod missing_gl_enums;

static mut GLFW: MaybeUninit<glfw::Glfw> = MaybeUninit::uninit();

static GL_INITIALIZED: AtomicBool = AtomicBool::new(false);

// Include shaders
const VERT_SHADER: &'static str = include_str!("shaders/terrain_shader.vert");
const FRAG_SHADER: &'static str = include_str!("shaders/terrain_shader.frag");

const SPRITE_VERT_SHADER: &'static str = include_str!("shaders/sprite_shader.vert");
const SPRITE_FRAG_SHADER: &'static str = include_str!("shaders/sprite_shader.frag");

const INSTANCES: usize = 500;

pub struct TerrainVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub struct SpriteVertex {
    pub x: f32,
    pub y: f32, 
    pub z: f32,
    pub u: f32,
    pub v: f32
}

pub struct SpriteInfo {
    pub u: f32,
    pub v: f32,
    pub w: f32,
    pub h: f32
}

const TEST_SPRITE_INFOS: [SpriteInfo; 17] = [
    SpriteInfo { u: 0.00, v: 0.00, w: 0.00, h: 0.00},
    SpriteInfo { u: 0.00, v: 0.00, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.25, v: 0.00, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.50, v: 0.00, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.75, v: 0.00, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.00, v: 0.25, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.25, v: 0.25, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.50, v: 0.25, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.75, v: 0.25, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.00, v: 0.50, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.25, v: 0.50, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.50, v: 0.50, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.75, v: 0.50, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.00, v: 0.75, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.25, v: 0.75, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.50, v: 0.75, w: 0.25, h: 0.25},
    SpriteInfo { u: 0.75, v: 0.75, w: 0.25, h: 0.25},
];

const SPRITE_VERTICIES: [SpriteVertex; 6] = [
    SpriteVertex { x: -1.0, y: -1.0, z: 0.0, u: 0.0, v: 1.0 }, // Bottom left
    SpriteVertex { x:  1.0, y: -1.0, z: 0.0, u: 1.0, v: 1.0 }, // Bottom right
    SpriteVertex { x:  1.0, y:  1.0, z: 0.0, u: 1.0, v: 0.0 }, // Top right
    SpriteVertex { x: -1.0, y:  1.0, z: 0.0, u: 0.0, v: 0.0 }, // Top left
    SpriteVertex { x: -1.0, y: -1.0, z: 0.0, u: 0.0, v: 1.0 }, // Bottom left
    SpriteVertex { x:  1.0, y:  1.0, z: 0.0, u: 1.0, v: 0.0 }, // Top right
];

impl EngineErrorTrait for glfw::InitError {
    fn get_error_message(&self) -> &str {
        match &self {
            glfw::InitError::AlreadyInitialized => "AlreadyInitialized",
            glfw::InitError::Internal => "Internal",
        }
    }
}

pub struct Graphics {
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    terrain_shader: Shader,
    sprite_shader: Shader,
    sprite_vbo: u32,
    sprite_vao: u32,
    terrain_vbo: u32,
    terrain_vao: u32,
    sprites: [f32; INSTANCES * 4],
    sprite_ids: [i32; INSTANCES]
}

fn get_proc_address(window: &glfw::Window, fn_name: *const u8) -> *const c_void {
    // The fact that I need to do ths is bullshit, but I don't see any way around it.
    let w = unsafe {&mut *(window as *const glfw::Window as *mut glfw::Window)};
    let fn_name_slice = std::str::from_utf8(unsafe {std::slice::from_raw_parts(fn_name, strlen(fn_name as *const i8))}).unwrap();

    // I'm pretty sure this function shouldn't atually be mutating anything idk why the library wants a &mut self
    w.get_proc_address(fn_name_slice)
}

impl Graphics {
    pub fn init_gl() -> Result<(), EngineError> {
        if GL_INITIALIZED.load(Ordering::Relaxed) {
            return Err("GL Already Initialized!".into());
        }
        
        let init = glfw::init(glfw::FAIL_ON_ERRORS);
        
        if init.is_err() {
            let err_str = format!("GL Init Error: {:?}", init.err().unwrap());
            return Err(err_str.into());
        }

        unsafe {GLFW.write(init.unwrap())};
    
        GL_INITIALIZED.store(true, Ordering::Relaxed);
        Ok(())
    }

    pub fn gl_initialized() -> bool {
        GL_INITIALIZED.load(Ordering::Relaxed)
    }

    pub fn get_glfw_time() -> f64 {

        let glfw = unsafe { GLFW.assume_init_ref() };
        glfw.get_time()
    }

    pub fn create_window() -> Result<Graphics, EngineError> {
        if !Graphics::gl_initialized() {
            return Err("GL not initialized!".into());
        }

        let glfw = unsafe {GLFW.assume_init_mut()};
     
        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = {
            let op = glfw.create_window(800, 600, "Wob", glfw::WindowMode::Windowed);

            if op.is_none() {
                return Err("Failed to create GLFW window.".into());
            }

            op.unwrap()
        };
    
        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);

        let mut gfx = Graphics {window: window,
            events,
            terrain_shader: Shader::null_shader(),
            sprite_shader: Shader::null_shader(),
            sprite_vbo: 0,
            sprite_vao: 0,
            terrain_vbo: 0,
            terrain_vao: 0,
            sprites: [0.0; INSTANCES * 4],
            sprite_ids: [0; INSTANCES]
        };

        // Do gl stuff
        unsafe {
            load_global_gl(&|fn_name| get_proc_address(&gfx.window, fn_name));
            gfx.terrain_shader = Shader::load_shader_program("Terrain Shader", VERT_SHADER, FRAG_SHADER, &[]);
    
            glClearColor(0.2, 0.3, 0.3, 1.0);
    
            gfx.sprite_shader = Shader::load_shader_program("Sprite Shader", SPRITE_VERT_SHADER, SPRITE_FRAG_SHADER, &[ShaderArg("$sheet_size", "17"), ShaderArg("$instance_count", &INSTANCES.to_string())]);
    
            let mut vao: u32 = 0;
            glGenVertexArrays(1, &mut vao);
            if (vao as i32) < 0 {
                return Err("Error creaing VAO!".into());
            }
            
            glBindVertexArray(vao);
    
            let mut vbo: u32 = 0;
            glGenBuffers(1, &mut vbo);
            if (vbo as i32) < 0 {
                return Err("Error creaing VBO!".into());
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
                return Err("Error creaing VAO!".into());
            }
            
            glBindVertexArray(vao);
    
            let mut vbo: u32 = 0;
            glGenBuffers(1, &mut vbo);
            if (vbo as i32) < 0 {
                return Err("Error creaing VBO!".into());
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
            
            let f = File::open("test_spritesheet.png").unwrap();
            
            let tex = gfx.sprite_shader.load_texture(f);

            let loc = glGetUniformLocation(gfx.sprite_shader.get_program(), b"MainTex\0" as *const u8);

            if loc >= 0 {
                glUniform1i(loc, tex as i32);
            }

            // Send verticies to gpu
            glBufferData(
                GL_ARRAY_BUFFER,
                (SPRITE_VERTICIES.len() * size_of::<SpriteVertex>()) as isize,
                SPRITE_VERTICIES.as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);  
    
            glBindBuffer(GL_ARRAY_BUFFER, 0);
            glBindVertexArray(0);
        }
    
        Ok(gfx)
    }
    
    pub fn buffer_terrain_verticies(&mut self, verticies: &[TerrainVertex]) {
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

            glBindVertexArray(self.sprite_vao);
            glBindBuffer(GL_ARRAY_BUFFER, self.sprite_vbo);
            
            let program = self.sprite_shader.get_program();
            glUseProgram(program);
            
            let loc: i32 = glGetUniformLocation(self.sprite_shader.get_program(), b"sprite_info\0" as *const u8);
        
            if loc >= 0 {
                glUniform4fv(loc, 17, TEST_SPRITE_INFOS.as_ptr().cast());
            }

            let loc: i32 = glGetUniformLocation(program, b"sprites\0" as *const u8);
            if loc >= 0 {
                glUniform4fv(loc, INSTANCES as i32, self.sprites.as_ptr().cast());
            }

            let loc: i32 = glGetUniformLocation(program, b"sprite_id\0" as *const u8);
            if loc >= 0 {
                glUniform1iv(loc, INSTANCES as i32, self.sprite_ids.as_ptr().cast());
            }

            glDrawArraysInstanced(GL_TRIANGLES, 0, 6, INSTANCES as i32);
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

    pub fn update_sprite(&mut self, sprite: Sprite, idx: usize) {
        self.sprites[idx * 4] = sprite.x;
        self.sprites[idx * 4 + 1] = sprite.y;
        self.sprites[idx * 4 + 2] = sprite.w;
        self.sprites[idx * 4 + 3] = sprite.h;

        self.sprite_ids[idx] = sprite.sprite_id;
    }
}