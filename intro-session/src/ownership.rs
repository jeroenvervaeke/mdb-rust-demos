pub struct Counter {
    count: u32,
}

impl Counter {
    /// # Ownership
    /// This code works fine in java, but what's wrong with it?
    /// Hint: think about concurrency
    /// ```java
    /// class Counter {
    ///    private int count = 0;
    ///    public void increment() {
    ///        count++;
    ///    }
    /// }
    /// ```
    ///
    /// Solution:
    /// `count++` is not atomic in java, when multiple threads call `increment` you might see some unexpected results
    ///
    /// Rust has a unique ownership model which allows to either have:
    /// - 1 mutable reference
    /// - many immutable references
    ///
    /// This makes bugs like the ones above impossible
    // Try making the reference `&self` and see what happens
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
