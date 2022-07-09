use std::cell::RefCell;
use std::rc::Rc;

use gl33::global_loader::*;
use gl33::gl_enumerations::*;

pub struct Shader {
    name: Rc<RefCell<String>>,
    program: Rc<RefCell<u32>>
}

impl Shader {
    pub fn null_shader() -> Shader {
        Shader {name: Rc::new(RefCell::new(String::from("NULL"))), program: Rc::new(RefCell::new(0)) }
    }

    pub unsafe fn load_shader_program(name: &str, vert: &str, frag: &str) -> Shader {
        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
                                    
        glShaderSource(
            vertex_shader,
            1,
            &(vert.as_bytes().as_ptr().cast()),
            &(vert.len().try_into().unwrap()),
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
            &(frag.as_bytes().as_ptr().cast()),
            &(frag.len().try_into().unwrap()),
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

        let shader_program = glCreateProgram();

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

        Shader { name: Rc::new(RefCell::new(String::from(name))), program: Rc::new(RefCell::new(shader_program)) }
    }

    pub unsafe fn delete_shader(&self) {
        glDeleteProgram(*self.program.borrow());
        *self.name.borrow_mut() = String::from("NULL");
        *self.program.borrow_mut() = 0;
    }

    pub fn get_name(&self) -> String {
        self.name.borrow().clone()
    }

    pub fn get_program(&self) -> u32 {
        *self.program.borrow()
    }
}

impl Clone for Shader {
    fn clone(&self) -> Shader {
        Shader { name: Rc::clone(&self.name), program: Rc::clone(&self.program) }
    }
}