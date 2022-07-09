
mod answers;

use answers::{answer_01, answer_02, answer_03};

use crate::answers::answer_01::Timer;

fn main() 
{
    /* Question 01 */
    let mut light = answer_01::TrafficLight::Red;
    println!("Red Light Duration: {}", light.get_duration().unwrap());

    light = answer_01::TrafficLight::Green;
    println!("Green Light Duration: {}", light.get_duration().unwrap());

    light = answer_01::TrafficLight::Yellow;
    println!("Yellow Light Duration: {}", light.get_duration().unwrap());

    
    /* Question 02 */
    let mut my_num_set = vec![1, 3, 5, 7, 9, 11];
    let mut sum_res = answer_02::cal_sum(&my_num_set);

    match answer_02::cal_sum(&my_num_set)
    {
        Some(_) =>
        {
            println!("Cal_Sum First: {:?}", sum_res);
        },
        None =>
        {
            println!("Cal_Sum Second Overflow");
        }

    }

    my_num_set = vec![1, u32::max_value()];
    sum_res = answer_02::cal_sum(&my_num_set);
 
    match answer_02::cal_sum(&my_num_set)
    {
        Some(_) =>
        {
            println!("Cal_Sum First: {:?}", sum_res);
        },
        None =>
        {
            println!("Cal_Sum Second Overflow");
        }

    }

    
    /* Question 03 */
    let my_circle = answer_03::Circle{radius: 2.5};
    let my_triangle = answer_03::Triangle{length: 4, height: 5};
    let my_square = answer_03::Square{length: 5.5};

    let area_circle = answer_03::cal_area(&my_circle);
    println!("My Circle Area: {}", area_circle);

    let area_triangle = answer_03::cal_area(&my_triangle);
    println!("My Triangel Area: {}", area_triangle);

    let area_square = answer_03::cal_area(&my_square);
    println!("My Square Area: {}", area_square);
}
