
pub fn cal_sum(num_set: &[u32]) -> Option<u32>
{
    let mut res: u64 = 0;

    res = num_set.iter().fold(0, |sum, item| sum + *item as u64);
    
    match res <= u32::max_value() as u64
    {
        true => 
        {
            return Some(res as u32);
        }
        false => return None
    }
}