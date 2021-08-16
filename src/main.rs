

#[allow(unused)]
use macroquad::prelude::{
    screen_width,
    screen_height, 
    Vec2, 
    clear_background,
    YELLOW,
    BLUE,
    GREEN,
    rand,
    draw_circle,
    next_frame,
    get_char_pressed,
    is_mouse_button_pressed,
    mouse_position,
};
// use macroquad::*;

fn greate_tail(golova: Vec2, kolvo: u8) -> Vec<Vec2> {
    let mut tail = Vec::new();
    let mut temp_head = golova.clone();
    for _ in 0..kolvo{
        let temp = Vec2::new(temp_head.x-10.0, temp_head.y);
        tail.push(temp.clone());
        temp_head = temp;
    } 
    tail
}
struct Food{
    pos: Vec2,
}

impl Food{
    fn new() -> Self{
        Self{
            pos:Vec2::new(rand::gen_range(10., screen_width()-10.0), 
                            rand::gen_range(10., screen_height()-10.0))
        }
    }
    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 8.0, GREEN)
    }
}

struct Zmeya{
    head: Vec2,
    tail: Vec<Vec2>,
    direction: Vec2,
}
impl Zmeya {
    fn new() -> Self{
        let temp = Vec2::new(rand::gen_range(100., screen_width()-100.0), 
                            rand::gen_range(100., screen_height()-100.0));
        Self{
            head: temp.clone(), 
            tail: greate_tail(temp, 4),  
            direction: Vec2::new(1.0,0.0),
        }
    }
    fn render(&self) {
        draw_circle(self.head.x, self.head.y, 8.0, YELLOW);
        for item in &self.tail{
            draw_circle(item.x, item.y, 5.0 ,YELLOW);
        }
    }
    fn move_snake(&mut self) {
        let mut temp = self.head.clone();
        for i in 0..self.tail.len(){
            let temp2 = self.tail[i].clone();
            self.tail[i] = temp.clone();
            temp = temp2;
        }
        self.head += self.direction;
        self.head += self.direction;
        self.head += self.direction;
        self.head += self.direction;
        self.head += self.direction;
        if self.head.x > screen_width(){ self.head.x -= screen_width() }
        if self.head.x < 0.0{ self.head.x += screen_width() }
        if self.head.y > screen_height(){ self.head.y -= screen_height() }
        if self.head.y < 0.0{ self.head.y += screen_height() }
    }
    fn change_dir(&mut self, dir: (f32, f32)){
        let temp = Vec2::new(dir.0, dir.1) - self.head;
        self.direction = temp.normalize();
    }
}

#[macroquad::main("STAS")]
async fn main() {
    let mut zm = Zmeya::new();
    let mut food = Food::new();
    loop{
        clear_background(BLUE);
        zm.render();
        food.render();
        zm.move_snake();
        let dis = zm.head.clone() - food.pos.clone();
        if dis.length() < 16.0{
            food = Food::new();
            zm.tail.push(Vec2::new(0.0,0.0));
        }
        if is_mouse_button_pressed(macroquad::prelude::MouseButton::Left){
            zm.change_dir(mouse_position())
        }
        next_frame().await;
    }
}
