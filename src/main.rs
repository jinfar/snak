

#[allow(unused)]
use macroquad::prelude::{
    screen_width,
    screen_height, 
    Vec2, 
    clear_background,
    YELLOW,
    BLUE,
    rand,
    draw_circle,
    next_frame,
    get_char_pressed,
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
        draw_circle(self.head.x, self.head.y, 7.0, YELLOW);
        for item in &self.tail{
            draw_circle(item.x, item.y, 5.0 ,YELLOW);
        }
    }
    fn move_snake(&mut self) {
        let mut temp = self.head.clone();
        for i in 0..self.tail.len(){
            let mut temp2 = self.tail[i].clone();
            self.tail[i] = temp.clone();
            temp = temp2;
        }
        match self.direction{
            'r' => self.head.x += 10.0,
            'l' => self.head.x += -10.0,
            'u' => self.head.y += 10.0,
            'd' => self.head.y += -10.0,
            _ => print!("Oshibka vvoda"),
        }
    }
    fn change_dir(&mut self, dir: Option<char>){
        
        if dir.unwrap_or('x') == 'd' && self.direction != 'l'{
            self.direction = 'r'
        }
        if dir.unwrap_or('x') == 'w' && self.direction != 'd'{
            self.direction = 'u'
        }
        if dir.unwrap_or('x')  == 's' && self.direction != 'u'{
            self.direction = 'd'
        }
        if dir.unwrap_or('x') == 'a' && self.direction != 'r'{
            self.direction = 'l'
        } 
        // dbg!(self.direction);
        // dbg!(dir);

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

}

#[macroquad::main("STAS")]
async fn main() {
    let mut zm = Zmeya::new();
    loop{
        clear_background(BLUE);
        zm.render();
        zm.move_snake();
        while let Some(c) = get_char_pressed() {
            dbg!(c);
            zm.change_dir2(c);
        }
        next_frame().await;
    }
}
