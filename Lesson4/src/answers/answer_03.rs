use std::f32::consts::PI;

pub trait Area
{
    type Output;
    fn get_area(&self) -> Self::Output;
}

pub struct Circle
{
    pub radius: f32,
}

pub struct Triangle
{
    pub length: u32,
    pub height: u32,
}

pub struct Square
{
    pub length: f32,
}

impl Area for Circle 
{
    type Output = f32;
    fn get_area(&self) -> Self::Output
    {
        PI * self.radius.powf(2 as f32)
    }
}

impl Area for Triangle
{
    type Output = u32;
    fn get_area(&self) -> Self::Output
    {
        self.length * self.height
    } 
}

impl Area for Square 
{
    type Output = f32;
    fn get_area(&self) -> Self::Output
    {
        self.length.powf(2 as f32)
    } 
}

pub fn cal_area<T: Area>(shape: &T) ->T::Output
{
     shape.get_area()
}