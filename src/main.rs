use raylib::prelude::*;


const BACKGROUND: &str = "101010";
const FOREGROUND: &str = "50FF50";
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 800;

#[derive(Copy, Clone)]
struct VectorStruct2d {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
struct VectorStruct2dMapped {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct VectorStruct3d {
    x: f32,
    y: f32,
    z: f32,
}

// const VS3:[VectorStruct3d;8] = [
//    VectorStruct3d{x: 0.25, y: 0.25, z: 0.25},
//    VectorStruct3d{x: -0.25, y: 0.25, z: 0.25},
//    VectorStruct3d{x: 0.25, y: -0.25, z: 0.25},
//    VectorStruct3d{x: -0.25, y: -0.25, z: 0.25},

//    VectorStruct3d{x: 0.25, y: 0.25, z: -0.25},
//    VectorStruct3d{x: -0.25, y: 0.25, z: -0.25},
//    VectorStruct3d{x: 0.25, y: -0.25, z: -0.25},
//    VectorStruct3d{x: -0.25, y: -0.25, z: -0.25},
// ];


const SIZE: i32 = 20;


fn main() { 
                            
    let background = Color::from_hex(BACKGROUND)
                                    .expect("Color no valido");    
    let foreground = Color::from_hex(FOREGROUND)
                                    .expect("Color no valido");

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let mut frame = || {
            d.clear_background(background);
            let mapped = screen(projection(VectorStruct3d { x: 0.5, y: 0.0, z: 1.0 }));

            d.draw_rectangle(mapped.x - (SIZE / 2), mapped.y - (SIZE / 2), SIZE, SIZE, foreground);
        };
        
        frame();
    }
}


fn screen(p: VectorStruct2d) -> VectorStruct2dMapped
{
    let size_w: f32 = SCREEN_WIDTH as f32;
    let size_h: f32 = SCREEN_HEIGHT as f32;

    let t_x = (((p.x + 1.0)/2.0)*size_w) as i32;
    let t_y = ((1.0-((p.y + 1.0)/2.0))*size_h) as i32;

    return VectorStruct2dMapped{ x: t_x, y: t_y };
}

fn projection(v:VectorStruct3d) -> VectorStruct2d
{
    return VectorStruct2d {
        x: v.x/v.z,
        y: v.y/v.z,
    };
}
