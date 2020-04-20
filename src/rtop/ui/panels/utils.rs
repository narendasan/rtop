use tui::layout::Rect;

pub fn scrolling<T: Clone> (area: Rect, absolute_postion: usize, data: &[T]) -> (usize, Vec<T>) {
    let capacity: usize = area.height as usize - 5; //For the header
    let selected_row = if absolute_postion > capacity {
        capacity
    } else {
        absolute_postion
    };

    let mut displayed_data = data.to_vec();
    if absolute_postion > capacity as usize {
        let cutoff = absolute_postion - capacity;
        displayed_data = displayed_data[cutoff..].to_vec();
    }

    (selected_row, displayed_data)
}
