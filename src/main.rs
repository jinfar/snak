

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
    KeyCode,
};
use macroquad::input::*;

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
        let temp = Vec2::new(rand::gen_range(10., screen_width()-10.0), 
                            rand::gen_range(10., screen_height()-10.0));
        Self{
            pos: temp, 
        }

    }
    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 8.0, GREEN);
    }
}




struct Zmeya{
    head: Vec2,
    tail: Vec<Vec2>,
    direction: char,
}
impl Zmeya {
    fn new() -> Self{
        let temp = Vec2::new(rand::gen_range(100., screen_width()-100.0), 
                            rand::gen_range(100., screen_height()-100.0));
        Self{
            head: temp.clone(), 
            tail: greate_tail(temp, 4),  
            direction: 'r',
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
        match self.direction{
            'r' => self.head.x += 10.0,
            'l' => self.head.x += -10.0,
            'u' => self.head.y += -10.0,
            'd' => self.head.y += 10.0,
            _ => print!("Oshibka vvoda"),
        }
        if self.head.x > screen_width(){
            self.head.x -= screen_width()
        }
        if self.head.y > screen_height(){
            self.head.y -= screen_height()
        }
        if self.head.x < 0.0{
            self.head.x += screen_width()
        }
        if self.head.y < 0.0{
            self.head.y += screen_height()
        }
    }
    fn change_dir2(&mut self, dir: char){
        
        if dir == 'd' && self.direction != 'l'{
            self.direction = 'r'
        }
        if dir == 'w' && self.direction != 'd'{
            self.direction = 'u'
        }
        if dir == 's' && self.direction != 'u'{
            self.direction = 'd'
        }
        if dir == 'a' && self.direction != 'r'{
            self.direction = 'l'
        } 
        // dbg!(self.direction);
        // dbg!(dir);

    }
    fn got_it(&mut self){
        self.tail.push(Vec2::new(0.0, 0.0))
    }

}

#[macroquad::main("STAS")]
async fn main() {
    let mut zm = Zmeya::new();
    let mut food = Food::new();
    loop{
        clear_background(BLUE);
        zm.move_snake();
        food.render();
        zm.render();
        let dis = zm.head - food.pos;
        if dis.length()<16.0{
            zm.got_it();
            food = Food::new();
        }
        if is_key_pressed(KeyCode::A){
            zm.change_dir2('a');
        }
        if is_key_pressed(KeyCode::S){
            zm.change_dir2('s');
        }
        if is_key_pressed(KeyCode::D){
            zm.change_dir2('d');
        }
        if is_key_pressed(KeyCode::W){
            zm.change_dir2('w');
        }
        if is_key_pressed(KeyCode::Q){
            break;
        }
        // println!("{}", get_char_pressed().unwrap_or('x'));
        next_frame().await;
    }
}
