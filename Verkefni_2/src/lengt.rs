#[derive(Debug)]
pub struct Length{
    pub leingt: f32,
    pub breit: f32,
}
//gevur self value svo hægt er að nota það
impl Length {
    pub fn new(leingt: f32, breit: f32) -> Self {
        Self { leingt, breit }
    }
    //gefur area sem við munum actually nota
    pub fn area(&self) -> f32 {
        self.leingt * self.breit
    }
}
