/// This trait provides `stalinsort` functionality
pub trait Stalinsort {
    /// **Stalinsort** is a single pass stable (technically) sorting algorithm
    /// that removes out of order items.
    fn stalinsort(&mut self);
}

impl<T: PartialOrd> Stalinsort for Vec<T> {
    fn stalinsort(&mut self) {
        let mut index: usize = 1;

        while index < self.len() {
            if self[index] < self[index-1] {
                // This item is not great enough. Send it away.
                self.remove(index);
            } else {
                // This item has been approved by the commissar. Next!
                index += 1
            }
        }
    }
}
