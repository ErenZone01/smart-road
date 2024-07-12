use rand::Rng;
use sdl2::event::Event;
use sdl2::image::{self, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::messagebox::{show_simple_message_box, MessageBoxFlag};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const CAR_WIDTH: u32 = 30;
const CAR_HEIGHT: u32 = 40;
const DISTANCE: i32 = 60;
const DISTANCE_SAFE: i32 = 250;
const UPDATE_THRESHOLD: u32 = 10; // Adjust this to control update frequency

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy)]
struct Car<'a> {
    texture: &'a sdl2::render::Texture<'a>,
    x: i32,
    y: i32,
    direction: Direction,
    velocity: i32,
    distance: i32,
    time: i32,
    timer: Instant,
    states: bool,
    frame_count: u32,
    angle: f64,
    cote:i32,
    turning: bool, 
}

impl<'a> Car<'a> {
    fn new(
        texture: &'a sdl2::render::Texture<'a>,
        x: i32,
        y: i32,
        direction: Direction,
        angle: f64,
        cote : i32,
    ) -> Self {
        Car {
            texture,
            x,
            y,
            direction,
            velocity: 3,
            distance: 0,
            time: 0,
            timer: Instant::now(),
            states: true,
            frame_count: 0,
            angle, 
            cote ,
            turning: false,
        }
    }
    fn distance(&self, other: &Car, safe_distance: i32) -> bool {
    
        match self.direction {
            Direction::Down => {
                let dx = (other.x - self.x).abs();
                let dy = (other.y - self.y).abs();
                if other.y >= self.y {
                    return dy <= safe_distance as i32 && dx < (CAR_WIDTH as i32);
                }
            }
            Direction::Up => {
                let dx = (self.x - other.x).abs();
                let dy = (self.y - other.y).abs();
                if self.y >= other.y {
                    return dy <= safe_distance as i32 && dx < (CAR_WIDTH as i32);
                }
            }
            Direction::Left => {
                let dx = (self.x - other.x).abs();
                let dy = (self.y - other.y).abs();
                if self.x >= other.x {
                    return dx <= safe_distance as i32 && dy < (CAR_HEIGHT as i32);
                }
            }
            Direction::Right => {
                let dx = (other.x - self.x).abs();
                let dy = (other.y - self.y).abs();
                if other.x >= self.x {
                    return dx <= safe_distance as i32 && dy < (CAR_HEIGHT as i32);
                }
            }
        }
        false
    }

    fn should_turning(&self)-> Option<Direction> {
        match self.direction {
            Direction::Up => {
                if self.turning {
                    if self.y <= 355 && self.angle == 0.0 && self.x == 410 {
                        return  Some(Direction::Left);
                    } else if self.y <= 490 && self.angle == 0.0 && self.x == 500 {
                        return  Some(Direction::Right);
                    }
                }
                None
            }
            Direction::Down => {
                if self.turning {
                    if self.y >= 405 && self.angle == 180.0 && self.x == 365 {
                        return  Some(Direction::Right);
                        
                    } else if self.y >= 270 && self.angle == 180.0 && self.x == 275 {
                        return  Some(Direction::Left);
                    }
                }
                None
            }
            Direction::Left => {
                if self.turning {
                    if self.y == 270 && self.angle == -90.0 && self.x <= 500 {
                        return Some(Direction::Up);
                    } else if self.y == 360 && self.angle == -90.0 && self.x <= 365 {
                        return  Some(Direction::Down);
                    }
                }
                None
            }
            Direction::Right => {
                if self.turning {
                    if self.y == 400 && self.angle == 90.0 && self.x >= 405 {
                        return  Some(Direction::Up);
                    } else if self.y == 490 && self.angle == 90.0 && self.x >= 275 {
                        return  Some(Direction::Down);
                    }
                }
                None
            }
        }
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.turning {
                    if self.y <= 355 && self.angle == 0.0 && self.x == 410 {
                        self.direction = Direction::Left;
                        self.angle = -90.0;
                        self.turning=false;

                        self.velocity = 0;
                        //self.y += self.velocity;
                    } else if self.y <= 490 && self.angle == 0.0 && self.x == 500 {
                        self.direction = Direction::Right;
                        self.angle = 90.0;
                        self.turning=false;
                        self.velocity = 0;
                        //self.y += self.velocity;
                    }
                }
                self.y -= self.velocity;
            }
            Direction::Down => {
                if self.turning {
                    if self.y >= 405 && self.angle == 180.0 && self.x == 365 {
                        self.direction = Direction::Right;
                        self.angle = 90.0;
                        self.turning=false;
                        self.velocity = 0;
                    } else if self.y >= 270 && self.angle == 180.0 && self.x == 275 {
                        self.direction = Direction::Left;
                        self.angle = -90.0;
                        self.turning=false;
                        self.velocity = 0;
                    }
                }
                self.y += self.velocity;
            }
            Direction::Left => {
                if self.turning {
                    if self.y == 270 && self.angle == -90.0 && self.x <= 500 {
                        self.direction = Direction::Up;
                        self.angle = 0.0;
                        self.turning=false;
                        self.velocity = 0; 
                    } else if self.y == 360 && self.angle == -90.0 && self.x <= 365 {
                        self.direction = Direction::Down;
                        self.angle = 180.0;
                        self.turning=false;
                        self.velocity = 0;
                    }
                }
                self.x -= self.velocity;
            }
            Direction::Right => {
                if self.turning {
                    if self.y == 400 && self.angle == 90.0 && self.x >= 405 {
                        self.direction = Direction::Up;
                        self.angle = 0.0;
                        self.turning=false;
                        self.velocity = 0;
                    } else if self.y == 490 && self.angle == 90.0 && self.x >= 275 {
                        self.direction = Direction::Down;
                        self.angle = 180.0;
                        self.turning=false;
                        self.velocity = 0;
                    }
                }
                self.x += self.velocity;
            }
        }

        self.distance += self.velocity;
        self.time += 1;
    }
    fn should_update(&mut self) -> bool {
        self.frame_count >= UPDATE_THRESHOLD
    }

    fn reset_frame_count(&mut self) {
        self.frame_count = 0;
    }

    fn increment_frame_count(&mut self) {
        self.frame_count += 1;
    }
}

fn main() -> Result<(), String> {
    // Initialisation de SDL2
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    // Initialisation de SDL2_image
    let _image_context = image::init(image::InitFlag::PNG).map_err(|e| e.to_string())?;

    // Création de la fenêtre
    let window = video_subsystem
        .window("SDL2 Window", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialisation du canevas pour dessiner
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    // Charger l'image de fond
    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator
        .load_texture("road.png")
        .map_err(|e| e.to_string())?;

    // Charger la texture de la voiture
    let car_texture = texture_creator
        .load_texture("vehicule.png")
        .map_err(|e| e.to_string())?;

    let mut cars: VecDeque<Car> = VecDeque::new();
    let mut car_count: i32 = 0;
    let mut collision = 0;
    let collision_check = 0;
    let mut max_velocity :i32 = 3;
    let mut min_velocity :i32 = 3;

    let mut vec_timer: Vec<Duration> = Vec::new();
    // Variables pour la temporisation par direction
    let mut last_up_car_time = Instant::now() - Duration::from_secs(2);
    let mut last_down_car_time = Instant::now() - Duration::from_secs(2);
    let mut last_left_car_time = Instant::now() - Duration::from_secs(2);
    let mut last_right_car_time = Instant::now() - Duration::from_secs(2);


    // Boucle principale
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
    'running: loop {
        let mut new_cars: VecDeque<Car> = VecDeque::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    let max_timer = vec_timer.iter().max().unwrap();
                    let min_timer = vec_timer.iter().min().unwrap();
                
                    let messages = format!(
                        "Nombre de véhicules ayant traversé l'intersection: {}\nLes accidents évités de justesse: {}\nNombre de collisions: {}\nVitesse maximale de tous les véhicules en px/s :{}\nVitesse minimale de tous les véhicules en px/s : {}\nTemps maximum que les véhicules ont mis pour passer l'intersection :{:?}\nTemps minimun que les véhicules ont mis pour passer l'intersection :{:?}",
                        car_count, collision,collision_check,max_velocity, min_velocity, max_timer, min_timer,
                    );
                    show_simple_message_box(
                        MessageBoxFlag::INFORMATION,
                        "Simulation terminee",
                        &messages,
                        None,
                    )
                    .map_err(|e| e.to_string())?;
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let mut rng = rand::thread_rng();
                    let auto_direction = rng.gen_range(0..4);
                    match auto_direction {
                        0 => {
                            if Instant::now() - last_up_car_time >= Duration::from_secs(2) {
                                let mut rng = rand::thread_rng();
                                let lane_offset = rng.gen_range(0..3) * 45; // Randomize between 0, 45, 90 (lanes)
                                let mut new_car =
                                    Car::new(&car_texture, 410 + lane_offset, 800, Direction::Up, 0.0, lane_offset);
                                if lane_offset == 0 || lane_offset == 90 {
                                    new_car.turning = true;
                                }
                                cars.push_back(new_car);
                                
                                last_up_car_time = Instant::now();
                            }
                        }
                        1 => {
                            if Instant::now() - last_right_car_time >= Duration::from_secs(2) {
                                let mut rng = rand::thread_rng();
                                let lane_offset = rng.gen_range(0..3) * 45; // Ra\domize between 0, 45, 90 (lanes)  
                                let mut new_car =
                                    Car::new(&car_texture, 0, 400 + lane_offset, Direction::Right, 90.0, lane_offset);
                                if lane_offset == 0 || lane_offset == 90 {
                                    new_car.turning = true;
                                }
                                cars.push_back(new_car);
                                
                                last_right_car_time = Instant::now();
                            }
                        }
                        2 => {
                            if Instant::now() - last_down_car_time >= Duration::from_secs(2) {
                                let mut rng = rand::thread_rng();
                                let lane_offset = rng.gen_range(0..3) * 45; // Randomize between 0, 45, 90 (lanes)
        
                                let mut new_car =
                                    Car::new(&car_texture, 275 + lane_offset, 0, Direction::Down, 180.0, lane_offset);
                                if lane_offset == 0 || lane_offset == 90 {
                                    new_car.turning = true;
                                }
                                cars.push_back(new_car);
    
                                last_down_car_time = Instant::now();
                            }
                        }
                        3 => {
                            if Instant::now() - last_left_car_time >= Duration::from_secs(2) {
                                let mut rng = rand::thread_rng();
                                let lane_offset = rng.gen_range(0..3) * 45; // Randomize between 0, 45, 90 (lanes) 
                                let mut new_car =
                                    Car::new(&car_texture, 800, 270 + lane_offset, Direction::Left, -90.0, lane_offset);
                                if lane_offset == 0 || lane_offset == 90 {
                                    new_car.turning = true;
                                }
                                cars.push_back(new_car);
    
                                last_left_car_time = Instant::now();
                            }
                        }
                        _ => {}
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if Instant::now() - last_up_car_time >= Duration::from_secs(2) {
                        let mut rng = rand::thread_rng();
                        let lane_offset = rng.gen_range(0..3) * 45; // Randomize between 0, 45, 90 (lanes)
                        let mut new_car =
                            Car::new(&car_texture, 410 + lane_offset, 800, Direction::Up, 0.0, lane_offset);
                        if lane_offset == 0 || lane_offset == 90 {
                            new_car.turning = true;
                        }
                        cars.push_back(new_car);
                        last_up_car_time = Instant::now();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if Instant::now() - last_down_car_time >= Duration::from_secs(2) {
                        let mut rng = rand::thread_rng();
                        let lane_offset = rng.gen_range(0..3) * 45; // Randomize between 0, 45, 90 (lanes)

                        let mut new_car =
                            Car::new(&car_texture, 275 + lane_offset, 0, Direction::Down, 180.0, lane_offset);
                        if lane_offset == 0 || lane_offset == 90 {
                            new_car.turning = true;
                        }
                        cars.push_back(new_car);
                        last_down_car_time = Instant::now();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if Instant::now() - last_left_car_time >= Duration::from_secs(2) {
                        let mut rng = rand::thread_rng();
                        let lane_offset = rng.gen_range(0..3) * 45; // Randomize between 0, 45, 90 (lanes) 
                        let mut new_car =
                            Car::new(&car_texture, 800, 270 + lane_offset, Direction::Left, -90.0, lane_offset);
                        if lane_offset == 0 || lane_offset == 90 {
                            new_car.turning = true;
                        }
                        cars.push_back(new_car);
                        last_left_car_time = Instant::now();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if Instant::now() - last_right_car_time >= Duration::from_secs(2) {
                        let mut rng = rand::thread_rng();
                        let lane_offset = rng.gen_range(0..3) * 45; // Ra\domize between 0, 45, 90 (lanes)  
                        let mut new_car =
                            Car::new(&car_texture, 0, 400 + lane_offset, Direction::Right, 90.0, lane_offset);
                        if lane_offset == 0 || lane_offset == 90 {
                            new_car.turning = true;
                        }
                        cars.push_back(new_car);
                        last_right_car_time = Instant::now();
                    }
                }
                _ => {}
            }
        }
        let car2: &VecDeque<Car> = &cars.clone();
        let mut c1 = 0;
        
        // Dans votre boucle principale de mise à jour des voitures :
        for car in &mut cars {
            c1 += 1;
            let mut actif = true;
            let mut state_acceleration = true;
        
            for (i, v) in car2.iter().enumerate() {
                if c1 != i + 1 && car.distance(v ,DISTANCE_SAFE) {
                    state_acceleration = false;
                }
                if c1 != i + 1 && car.distance(v ,DISTANCE) {
                    actif = false;
                    if car.states {
                        collision += 1;
                    }
                    car.states = false;
                    break;
                }
                
                let mut s_car = car.clone();
                let s_direction = s_car.should_turning();
                if c1 != i + 1 && s_direction.is_some() {
                    s_car.direction = s_direction.unwrap();
                    if car.distance( v, DISTANCE) {
                        actif = false;
                        break;
                    }
                }
            }
            if actif {
                if car.should_update() {
                    if state_acceleration {
                        car.velocity = 3;
                        if car_count == 1 {
                            min_velocity = car.velocity
                        }
                    } else {
                        car.velocity = 1;
                        if min_velocity > car.velocity {
                            min_velocity = car.velocity;
                            println!("i'm here");
                        }
                    }
                    car.states = true;
                    car.update();
                    car.reset_frame_count();
                } else {
                    car.increment_frame_count();
                }
            }else{
                min_velocity = 0;
            }
            //fin de course
            match car.direction {
                Direction::Down => {
                    if car.y > 800 {
                        car_count +=1;
                        vec_timer.push(car.timer.elapsed());
                    } else {
                        new_cars.push_back(*car);
                    }
                }
                Direction::Up => {
                    if car.y < 0 {
                        car_count +=1;
                        vec_timer.push(car.timer.elapsed());
                    } else {
                        new_cars.push_back(*car);
                    }
                }
                Direction::Left => {
                    if car.x < 0 {
                        car_count +=1;
                        vec_timer.push(car.timer.elapsed());
                    } else {
                        new_cars.push_back(*car);
                    }
                }
                Direction::Right => {
                    if car.x > 800 {
                        car_count +=1;
                        vec_timer.push(car.timer.elapsed());
                    } else {
                        new_cars.push_back(*car);
                    }
                }
            };
        }
        cars = new_cars;
        // Effacer le canevas avec une couleur de fond (optionnel)
        canvas.set_draw_color(Color::RGB(0, 0, 255)); // Bleu foncé
        canvas.clear();

        // Dessiner l'image de fond
        canvas.copy(&background_texture, None, None)?;

        // Dessiner les voitures
        for car in &cars {
            let target_rect = Rect::new(car.x, car.y, CAR_WIDTH, CAR_HEIGHT);
            canvas.copy_ex(
                &car.texture,
                None,
                Some(target_rect),
                car.angle,
                None,
                false,
                false,
            )?;
        }

        // Mettre à jour l'affichage
        canvas.present();
    }

    println!("Exiting program.");
    Ok(())
}
