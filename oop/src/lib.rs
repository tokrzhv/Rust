 pub trait Draw { //trait object
    fn draw(&self);
}
 //-> like this you can store mixed type (button. selectBox
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //dynamic dispatch
}
 impl Screen {
     pub fn run(&self) {
         for component in self.components.iter() {
             component.draw();
         }
     }
 }
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button struct todo!()")
    }
}

/*

//in generic usage, you limited to one type
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl <T> Screen<T>
    where
        T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
 */