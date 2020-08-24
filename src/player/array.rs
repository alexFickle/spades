use super::Player;

/// Length four array that uses `Player` as an index.
#[derive(Clone)]
pub struct Array<T>
where
    T: Clone,
{
    /// The internal storage.
    array: [T; 4],
}

impl<T> Array<T>
where
    T: Clone,
{
    /// Creates a new array filled in with a value.
    pub fn from_value(value: &T) -> Self {
        Self {
            array: [value.clone(), value.clone(), value.clone(), value.clone()],
        }
    }

    /// Creates a new player array from an array.
    pub fn from_array(array: [T; 4]) -> Self {
        Self { array }
    }

    /// Fills in an array with a value.
    pub fn fill(&mut self, value: &T) {
        for entry in self.array.iter_mut() {
            *entry = value.clone();
        }
    }

    /// Returns an iterator over the values in an array.
    pub fn iter<'a>(&'a self) -> core::slice::Iter<'a, T> {
        self.array.iter()
    }
}

/// lookup operator
impl<T> std::ops::Index<Player> for Array<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: Player) -> &Self::Output {
        &self.array[index.to_index() as usize]
    }
}

/// mutable lookup operator
impl<T> std::ops::IndexMut<Player> for Array<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: Player) -> &mut Self::Output {
        &mut self.array[index.to_index() as usize]
    }
}

/// Conditionally Default
impl<T> Default for Array<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self {
            array: [T::default(), T::default(), T::default(), T::default()],
        }
    }
}

/// Conditionally Copy
impl<T> Copy for Array<T> where T: Copy + Clone {}

/// Conditionally Debug
impl<T> std::fmt::Debug for Array<T>
where
    T: std::fmt::Debug + Clone,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.array.fmt(f)
    }
}

/// Conditionally PartialEq
impl<T> PartialEq for Array<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.array == rhs.array
    }
}

/// Conditionally Eq
impl<T> Eq for Array<T> where T: Eq + Clone {}
