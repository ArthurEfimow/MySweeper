#![allow(non_snake_case)]
#[macro_use]

extern crate serde_derive;

extern crate sdl2;
extern crate gl;
extern crate xml;
extern crate serde;
extern crate serde_json;
extern crate stb_image;
extern crate rand;

pub mod render_gl;

pub mod mesh;

pub mod form;

pub mod gamefield;

use std::ffi::{CString};


fn main() {

    //export MESA_GL_VERSION_OVERRIDE=4.30

    let args: Vec<String> = std::env::args().collect();
    let x : u8 = args[1].parse::<u8>().unwrap();
    let y : u8 = args[2].parse::<u8>().unwrap();
    let m : u8 = args[3].parse::<u8>().unwrap();

    let mut gamefield = gamefield::GameField::create(x,y,m);

    let background: form::Background = serde_json::from_str(&std::fs::read_to_string("./Data/Screen1.xml").unwrap()).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let width = 1000;
    let height = 1000;
    let _window = video_subsystem
        .window("MySweeper", width, height)
	.opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = _window.gl_create_context().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();
    let mut state;

    let mut open_field:  bool = false;
    let mut set_flag:  bool = false;
    let mut mouse_pos_x : f32;
    let mut mouse_pos_y : f32;

    unsafe {
	gl::Viewport(0, 0, width as i32, height as i32);
        gl::ClearColor(background.r, background.g, background.b, background.a);
    }

    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("./shaders/triangle.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("./shaders/triangle.frag")).unwrap()).unwrap();
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

   
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
    	gl::GenBuffers(1, &mut vbo);
    }    
    let mut MeshMap = std::collections::HashMap::new();

    let mut cells = vec![];
    let const1 = 1.0/(x as f32);
    let const2 = 1.0/(y as f32);
    let cell_length = 1.0/((x as f32)/2.0);
    let cell_height = 1.0/((y as f32)/2.0);
    for i in 0..x {
	for j in 0..y {
	    cells.push(form::Form::createQua( -1.0+const1+(i as f32)/((x as f32)/2.0), -1.0+const2+(j as f32)/((y as f32)/2.0),cell_length,cell_height,0.3));
	}
    }

    MeshMap.entry("Cell".to_string()).or_insert(mesh::Mesh::create("./Data/Img/cell.png".to_string()));
    MeshMap.entry("OCell".to_string()).or_insert(mesh::Mesh::create("./Data/Img/open_cell.png".to_string()));
    MeshMap.entry("MCell".to_string()).or_insert(mesh::Mesh::create("./Data/Img/mine_cell.png".to_string()));
    MeshMap.entry("M2Cell".to_string()).or_insert(mesh::Mesh::create("./Data/Img/hidden_mine_cell.png".to_string()));
    MeshMap.entry("FCell".to_string()).or_insert(mesh::Mesh::create("./Data/Img/flagged_cell.png".to_string()));
    MeshMap.entry("DCell".to_string()).or_insert(mesh::Mesh::create("./Data/Img/done_cell.png".to_string()));
    MeshMap.entry(1.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num1.png".to_string()));
    MeshMap.entry(2.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num2.png".to_string()));
    MeshMap.entry(3.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num3.png".to_string()));
    MeshMap.entry(4.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num4.png".to_string()));
    MeshMap.entry(5.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num5.png".to_string()));
    MeshMap.entry(6.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num6.png".to_string()));
    MeshMap.entry(7.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num7.png".to_string()));
    MeshMap.entry(8.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num8.png".to_string()));
    MeshMap.entry(9.to_string()).or_insert(mesh::Mesh::create("./Data/Img/AlphaNum/num9.png".to_string()));

    'main: loop {
		
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
		sdl2::event::Event::MouseButtonUp { mouse_btn : sdl2::mouse::MouseButton::Left, ..} => { open_field = true;},
                sdl2::event::Event::MouseButtonUp { mouse_btn : sdl2::mouse::MouseButton::Right, ..} => { set_flag = true;},
	        _ => {}
            }
	       
        }



        if (open_field || set_flag)  && gamefield.is_running(){
            state = event_pump.mouse_state();
	    mouse_pos_x = state.x() as f32 / (width/2) as f32 -1.0;
	    mouse_pos_y = -1.0* (state.y() as f32 / (height/2) as f32 -1.0);
	    for i in 0..cells.len() {
		if cells[i].hit(mouse_pos_x,mouse_pos_y){
		    if open_field {gamefield.open(i);gamefield.check_game_won(); } else {gamefield.flag(i);}
		}
	    }
	    open_field = false;
	    set_flag = false;
        }
	
	let game_lost = gamefield.is_lost();
	let game_won =  gamefield.is_won();
	for cell in gamefield.get_cells() {
	    if cell.open && cell.mine {MeshMap[&"MCell".to_string()].draw(&cells[cell.id].get_info());}
	    else if cell.mine && game_lost {MeshMap[&"M2Cell".to_string()].draw(&cells[cell.id].get_info());}
	    else if cell.mine && game_won  {MeshMap[&"DCell".to_string()].draw(&cells[cell.id].get_info());}
	    else if cell.open {MeshMap[&"OCell".to_string()].draw(&cells[cell.id].get_info());}
	    else if cell.flag {MeshMap[&"FCell".to_string()].draw(&cells[cell.id].get_info());}
	    else {MeshMap[&"Cell".to_string()].draw(&cells[cell.id].get_info());}
	    if cell.open && cell.mines_in_neighborhood > 0 {MeshMap[&cell.mines_in_neighborhood.to_string()].draw(&cells[cell.id].get_info());}
	    
	}
        _window.gl_swap_window();

	unsafe {gl::Clear(gl::COLOR_BUFFER_BIT);}
    }
}

