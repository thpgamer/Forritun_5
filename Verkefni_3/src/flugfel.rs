pub struct Flugvelar{
    pub call_name : String,
    pub speed : u32,
    pub leingt : i32
}
impl Flugvelar{
    pub fn new(call_name : &str, speed : u32, leingt : i32) -> Self{
        Self {
            call_name: call_name.to_string(),
            speed,
            leingt,
        }
    }
    pub fn timi(&self) {
        if self.leingt == 0 {
            println!("Error: Leingt cannot be zero.");
            return;
        }
        let etimi = (self.leingt as f32 / self.speed as f32) * 60.0;
        println!("Tími: {} mínutur", etimi);
    }
}