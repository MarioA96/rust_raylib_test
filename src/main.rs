use raylib::{ffi::{Vector2,Vector3}, prelude::*};
use std::f32::consts;


const BACKGROUND: &str = "101010";
// const FOREGROUND: &str = "50FF50";
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 800;

// #[derive(Copy, Clone)]
// struct VectorStruct2d {
//     x: f32,
//     y: f32,
// }

// #[derive(Copy, Clone)]
// struct VectorStruct2dMapped {
//     x: i32,
//     y: i32,
// }

// #[derive(Copy, Clone)]
// struct VectorStruct3d {
//     x: f32,
//     y: f32,
//     z: f32,
// }

const VS3:[Vector3;8] = [
   Vector3{x: 0.25, y: 0.25, z: 0.25},
   Vector3{x: -0.25, y: 0.25, z: 0.25},
   Vector3{x: -0.25, y: -0.25, z: 0.25},
   Vector3{x: 0.25, y: -0.25, z: 0.25},

   Vector3{x: 0.25, y: 0.25, z: -0.25},
   Vector3{x: -0.25, y: 0.25, z: -0.25},
   Vector3{x: -0.25, y: -0.25, z: -0.25},
   Vector3{x: 0.25, y: -0.25, z: -0.25},
];



// const SIZE: i32 = 20;
static mut DZ: f32 = 1.0;
static mut ANGLE: f32 = 0.0;


fn main() {                     
    let background = Color::from_hex(BACKGROUND)
                                    .expect("Color no valido");    
    // let foreground = Color::from_hex(FOREGROUND)
    //                                 .expect("Color no valido");

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let fs: Vec<Vec<i32>> = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![0, 4],
            vec![1, 5],
            vec![2, 6],
            vec![3, 7],
        ];

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(background);

        let frame = || {
            unsafe {
                // DZ += 1.25 * raylib::ffi::GetFrameTime(); // 1*dt -> dt=1/FPS

                ANGLE += (consts::PI)*(raylib::ffi::GetFrameTime());

                // for v3 in VS3 {
                //     let mapped = screen( projection( translate( rotate_xz(v3, ANGLE), DZ) ) );
                    
                //     d.draw_line_ex(Vector2 {x: 0.0, y: 0.0}, Vector2 {x: 200.0, y: 200.0}, 3.0, raylib::ffi::GetColor(0xFF50FF)); //color complentario...
                //     d.draw_rectangle(mapped.x - (SIZE / 2), mapped.y - (SIZE / 2), SIZE, SIZE, foreground);
                // }

                for f in fs {
                    for i in 0..f.len(){
                        let idx_a = f[i];
                        let a = VS3[idx_a as usize];

                        let idx_b = f[ (i+1)%f.len() ];
                        let b = VS3[idx_b as usize];

                        d.draw_line_ex(
                            screen( projection( translate( rotate_xz(a, ANGLE), DZ) ) ), 
                            screen( projection( translate( rotate_xz(b, ANGLE), DZ) ) ),  
                            3.0,
                            raylib::ffi::GetColor(0xFF50FF)
                        ); //color complentario...
                    }
                }

                raylib::ffi::WaitTime((0.5*raylib::ffi::GetFrameTime()) as f64);
            };

        };
        
        frame();
    }
}


fn screen(p: Vector2) -> Vector2
{
    let size_w: f32 = SCREEN_WIDTH as f32;
    let size_h: f32 = SCREEN_HEIGHT as f32;

    let t_x = ((p.x + 1.0)/2.0)*size_w;
    let t_y = (1.0-((p.y + 1.0)/2.0))*size_h;

    return Vector2{ x: t_x, y: t_y };
}

fn projection(v:Vector3) -> Vector2
{
    return Vector2 {
        x: v.x/v.z,
        y: v.y/v.z,
    };
}

fn translate(v: Vector3, dz: f32) -> Vector3 {

    return Vector3{
        x: v.x,
        y: v.y,
        z: v.z+dz,
    };
}

fn rotate_xz(v: Vector3, angle: f32) -> Vector3 {

    let c_theta:f32 = angle.cos();
    let s_theta:f32 = angle.sin();

    return Vector3{
        x: (v.x*c_theta) - (v.z*s_theta),
        y: v.y,
        z: (v.x*s_theta) + (v.z*c_theta),
    }
}