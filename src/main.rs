extern crate sdl2;
//mod gameboy;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[allow(dead_code)]
fn sdl_loop() -> () {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
    .window("Rustboy 64", 1280, 720)
    .resizable()
    .build()
    .unwrap();



    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {},
            }
        }

        //insert render code here
    }
}

#[allow(dead_code)]
fn read_rom(filename: &str) -> io::Result<()> {
    let mut rom = File::open(filename)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Failed to read rom file");
    println!("Read rom into memory!");
    Ok(())
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Need a rom file to read!");
        return ()
    }

    let rom_name = &args[1];
    println!("Got filename {:?}", rom_name);
    read_rom(&rom_name);
}
