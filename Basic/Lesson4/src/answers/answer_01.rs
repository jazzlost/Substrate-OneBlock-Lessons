
pub enum TrafficLight
{
    Red,
    Green,
    Yellow,
}

pub trait Timer
{
    fn get_duration(&self) -> Option<u8>
    {
        return None;
    }
}

impl Timer for TrafficLight
{
    fn get_duration(&self) -> Option<u8> 
    {
        match *self
        {
            TrafficLight::Red =>
            {
                return Some(60);
            },
            TrafficLight::Green =>
            {
                return Some(50);
            },
            TrafficLight::Yellow =>
            {
                return Some(10);
            },
        }
    }
}