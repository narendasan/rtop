
pub fn interpolate<T>(start: T, end: T, len: u16) -> Vec<T> 
where 
    T: std::ops::Sub<Output = T> + std::ops::Add<Output = T>
     + std::ops::Div<Output = T> + std::ops::Mul<Output = T> + From<u16> + Copy
{
    let m = (end - start) / T::from(len);
    (0..len).map(|i| start + (T::from(i) * m)).collect() 
} 