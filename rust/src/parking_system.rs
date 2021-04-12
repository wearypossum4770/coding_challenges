pub struct ParkingSystem{
    big:u32, 
    medium:u32, 
    small:u32
}

impl ParkingSystem{
    pub fn addCar(&mut self, carType:u32)->{
        if carType ==1{
            if self.big>0{
                self.big-=1;
                true
            }else {
                false
            }
        }else if carType ==2{
if self.medium>0{
    self.medium-=1;
    true
}else {
    false
}
        }else {
            if self.small >0 {
                self.small-=1;
                true
            }else {
                false
            }
        }
    }
}