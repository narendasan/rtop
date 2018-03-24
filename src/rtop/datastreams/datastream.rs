pub trait DataStream {
    fn new(max_hist_len: usize) -> Self;
    fn poll(&mut self); 
}