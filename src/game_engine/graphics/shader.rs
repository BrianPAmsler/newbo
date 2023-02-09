use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::rc::Rc;

use gl33::global_loader::*;
use gl33::gl_enumerations::*;

#[derive(Clone)]
pub struct ShaderArg<'a>(pub &'a str, pub &'a str);

pub struct Shader {
    name: Rc<RefCell<String>>,
    program: Rc<RefCell<u32>>,
    textures: Rc<RefCell<Vec<u32>>>
}

fn next_non_ws(i: &mut usize, s: &str) {
    while *i < s.len() && char::is_whitespace(s.chars().nth(*i).unwrap()) {
        *i += 1;
    }
}

fn next_ws(i: &mut usize, s: &str) {
    while *i < s.len() && !char::is_whitespace(s.chars().nth(*i).unwrap()) {
        *i += 1;
    }
}

fn next_of_char(i: &mut usize, c: char,s: &str) {
    while *i < s.len() && s.chars().nth(*i) != Some(c) {
        *i += 1;
    }
}

fn insert_shader_args(shader: &str, args: &[ShaderArg]) -> String {
    if args.len() == 0 {
        return shader.to_owned();
    }

    let args: HashMap<String, ShaderArg> = args.into_iter().map(|x| (x.0.to_owned(), x.to_owned())).collect();
    let mut out = String::new();

    let lines = shader.split('\n');
    for line in lines {
        let mut i = 0;
        next_non_ws(&mut i, line);

        if line.chars().nth(i) == Some('#') {
            next_of_char(&mut i, '$', line);

            if i < line.len() {
                let start = i;
                next_ws(&mut i, line);
                let end = i;

                let val = args.get(&line[start..end].to_owned()).expect("Shader arg does not exist!").1;

                out.push_str(&line[..start]);
                out.push_str(&val);

                if end < line.len() {
                    out.push_str(&line[end..]);
                }
            } else {    
                out.push_str(&line);
            }
        } else {
            out.push_str(&line);
        }

        out.push('\n');
    }

    out
}

impl Shader {
    pub fn null_shader() -> Shader {
        Shader {name: Rc::new(RefCell::new(String::from("NULL"))), program: Rc::new(RefCell::new(0)), textures: Rc::new(RefCell::new(Vec::new())) }
    }

    pub unsafe fn load_shader_program(name: &str, vert: &str, frag: &str, args: &[ShaderArg]) -> Shader {
        let vert = insert_shader_args(vert, args);
        let frag = insert_shader_args(frag, args);

        println!("{}", vert);

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

        Shader { name: Rc::new(RefCell::new(String::from(name))), program: Rc::new(RefCell::new(shader_program)), textures: Rc::new(RefCell::new(Vec::new())) }
    }

    pub unsafe fn delete_shader(&self) {
        glDeleteProgram(*self.program.borrow());
        *self.name.borrow_mut() = String::from("NULL");
        *self.program.borrow_mut() = 0;
    }

    pub unsafe fn load_texture<R: Read + Seek>(&mut self, readable: R) -> u32 {
        let buf = BufReader::new(readable);
        let img = image::io::Reader::new(buf).with_guessed_format().unwrap().decode().unwrap();
        let img = img.to_rgba8();
        let ptr = img.as_ptr();
        let mut texture: u32 = 0;

        // Create Texture
        glGenTextures(1, &mut texture);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_LINEAR.0 as i32); //scale linearly when image bigger than texture
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_LINEAR.0 as i32); //scale linearly when image smalled than texture
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA8.0 as i32, img.width() as i32, img.height() as i32, 0,
        GL_RGBA, GL_UNSIGNED_BYTE, ptr.cast());
        glEnable(GL_TEXTURE_2D);

        self.textures.borrow_mut().push(texture);

        texture
    }

    pub fn get_textures(&self) -> Ref<[u32]> {
        Ref::map(self.textures.borrow(), |rf| {
            &rf[..]
        })
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
        Shader { name: self.name.clone(), program: self.program.clone(), textures: self.textures.clone() }
    }
}