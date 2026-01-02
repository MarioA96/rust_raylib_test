use raylib::prelude::*;

use futures::executor::block_on;
use std::time::Duration;
use settimeout::set_timeout;


const BACKGROUND: &str = "101010";
const FOREGROUND: &str = "50FF50";
const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

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

#[derive(Copy, Clone)]
struct VectorStruct3dMapped {
    x: i32,
    y: i32,
    z: i32,
}

const VS3:[VectorStruct3d;8] = [
   VectorStruct3d{x: 0.25, y: 0.25, z: 0.25},
   VectorStruct3d{x: -0.25, y: 0.25, z: 0.25},
   VectorStruct3d{x: 0.25, y: -0.25, z: 0.25},
   VectorStruct3d{x: -0.25, y: -0.25, z: 0.25},

   VectorStruct3d{x: 0.25, y: 0.25, z: -0.25},
   VectorStruct3d{x: -0.25, y: 0.25, z: -0.25},
   VectorStruct3d{x: 0.25, y: -0.25, z: -0.25},
   VectorStruct3d{x: -0.25, y: -0.25, z: -0.25},
];

// const VS2:[VectorStruct2d;8] = [
//    VectorStruct2d{x: 0.25, y: 0.25},
//    VectorStruct2d{x: -0.25, y: 0.25},
//    VectorStruct2d{x: 0.25, y: -0.25},
//    VectorStruct2d{x: -0.25, y: -0.25},

//    VectorStruct2d{x: 0.25, y: 0.25},
//    VectorStruct2d{x: -0.25, y: 0.25},
//    VectorStruct2d{x: 0.25, y: -0.25},
//    VectorStruct2d{x: -0.25, y: -0.25},
// ];

//const FPS: f32 = 60.0;

//static mut dz:f32 = 1.0;
//static mut angle:f32 = 0.0;

fn main() { 
                            
    block_on(frame());
}

async fn frame()
{
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

        d.clear_background(background);
        
        for v3 in VS3 {
            let mut point = |v2:VectorStruct2dMapped| {
                let size = 20;

                d.draw_rectangle((v2.x-size)/2, (v2.y-size)/2, size, size, foreground);
            };

            point( screen(projection(v3)) );
        }
    }
}

fn screen(p: VectorStruct2d) -> VectorStruct2dMapped
{
    let size_w: f32 = SCREEN_WIDTH as f32;
    let size_h: f32 = SCREEN_HEIGHT as f32;

    let t_x = ((p.x + 1.0)/2.0*size_w) as i32;
    let t_y = ((1.0-(p.y + 1.0)/2.0)*size_h) as i32;

    return VectorStruct2dMapped{ x: t_x, y: t_y };
}

fn projection(v:VectorStruct3d) -> VectorStruct2d
{
    return VectorStruct2d {
        x: v.x/v.z,
        y: v.y/v.z,
    };
}


//fn frame(rl: RaylibHandle)
//{
//
//	// for(const v of vs){
//	// 	point( screen( projection( translate_z( rotate_xz(v, angle), dz ) ) ) );
//	// }
//	// setTimeout(frame, 1000/FPS);
//    
//
//    let mut dt:f32 = 1.0/FPS; 
//    dz += 1.0*dt;
//
//    angle += consts::PI * dt;
//    
//    
//
//    d.clear_background(background);
//    
//    for v in vs {
//        point(v);
//    }
//
//
//}


//fn point(v: VectorStruct)
//{
//    let d = rl.begin_drawing(&thread);
//    //let s: f32 = 20;
//
//    // d.draw_rectangle(v.x-s/2.0, v.y-s/2.0, s, s, Color::GREEN);
//    let foreground = Color::from_hex(FOREGROUND)
//                            .expect("Color no valido");
//    d.draw_rectangle(20,20,20,20,foreground);
//}

// fn screen(p: VectorStruct): VectorStruct
// {
// 	// -1...1 => 0...2 => 0...1 => 0...w
//     let point: VectorStruct = VectorStruct{p.x, p.y};

// 	// return {
// 	// 	x: (p.x + 1)/2*env.width,
// 	// 	y: (1-(p.y + 1)/2)*env.height,
// 	// }

// }

// //intead or struct use vector or array

// fn projection(v: VectorStruct): VectorStruct
// {
// 	return {
// 		x: x/z,
// 		y: y/z,
// 	}
// }