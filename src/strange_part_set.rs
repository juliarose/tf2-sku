//! Set for holding up to 3 strange parts.

use crate::error::InsertError;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Sub;
use tf2_enum::StrangePart;

const STRANGE_PART_COUNT: usize = 3;

/// Contains up to 3 strange parts. Although the underlying data structure is an array, it behaves 
/// like a set. Most methods mimic that of [`HashSet`](std::collections::HashSet).
/// 
/// This struct solves the following problems:
/// - An item can only hold up to 3 strange parts.
/// - An item cannot have duplicate strange parts.
/// - Comparing strange parts for equality is order-independent.
/// 
/// Since the number of strange parts is fixed, the struct uses zero heap allocations and is 
/// therefore [`Copy`].
/// 
/// # Examples
/// ```
/// use tf2_sku::StrangePartSet;
/// use tf2_enum::StrangePart;
/// 
/// // Create a set for strange parts with one strange part.
/// let mut strange_parts = StrangePartSet::double(
///     StrangePart::CriticalKills,
///     StrangePart::DamageDealt,
/// );
/// 
/// // Check that strange parts contains Damage Dealt.
/// assert!(strange_parts.contains(&StrangePart::DamageDealt));
/// assert_eq!(strange_parts.len(), 2);
/// 
/// // Add a strange part.
/// strange_parts.insert(StrangePart::EngineersKilled).ok();
/// 
/// assert_eq!(strange_parts.len(), 3);
/// 
/// // If a strange part is added when strange parts are full, an error is returned.
/// assert!(strange_parts.insert(StrangePart::MedicsKilled).is_err());
/// assert!(!strange_parts.contains(&StrangePart::MedicsKilled));
/// 
/// // Iterate over strange parts.
/// for strange_part in strange_parts {
///     println!("{strange_part}");
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, Eq)]
pub struct StrangePartSet {
    inner: [Option<StrangePart>; STRANGE_PART_COUNT],
}

impl StrangePartSet {
    /// Creates a set for strange parts.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// 
    /// let strange_parts = StrangePartSet::new();
    /// 
    /// assert!(strange_parts.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a set for strange parts with one strange part.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangePartSet::single(
    ///     StrangePart::DamageDealt,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 1);
    /// ```
    pub fn single(strange_part: StrangePart) -> Self {
        Self::from([
            Some(strange_part),
            None,
            None,
        ])
    }
    
    /// Creates a set for strange parts with two strange parts.
    /// 
    /// If the same strange part is added multiple times, only one will be kept.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangePartSet::double(
    ///     StrangePart::DamageDealt,
    ///     StrangePart::CriticalKills,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 2);
    /// ```
    pub fn double(
        strange_part1: StrangePart,
        strange_part2: StrangePart,
    ) -> Self {
        Self::from([
            Some(strange_part1),
            Some(strange_part2),
            None,
        ])
    }
    
    /// Creates a set for strange parts with two strange parts.
    /// 
    /// If the same strange part is added multiple times, only one will be kept.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangePartSet::triple(
    ///     StrangePart::DamageDealt,
    ///     StrangePart::CriticalKills,
    ///     StrangePart::EngineersKilled,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 3);
    /// ```
    pub fn triple(
        strange_part1: StrangePart,
        strange_part2: StrangePart,
        strange_part3: StrangePart,
    ) -> Self {
        Self::from([
            Some(strange_part1),
            Some(strange_part2),
            Some(strange_part3),
        ])
    }
    
    /// Clears the set, removing all strange parts.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let mut strange_parts = StrangePartSet::from([
    ///     Some(StrangePart::CriticalKills),
    ///     Some(StrangePart::DamageDealt),
    ///     None,
    /// ]);
    /// 
    /// strange_parts.clear();
    /// 
    /// assert_eq!(strange_parts.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.inner = [None, None, None];
    }
    
    /// Adds a strange part to the first available slot. If no slots are available, the new strange 
    /// part will be ignored.
    /// 
    /// # Errors
    /// - If the set is full.
    /// - If the strange part is already in the set.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let mut strange_parts = StrangePartSet::double(
    ///     StrangePart::CriticalKills,
    ///     StrangePart::DamageDealt,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 2);
    /// 
    /// strange_parts.insert(StrangePart::EngineersKilled).ok();
    /// 
    /// assert_eq!(strange_parts.len(), 3);
    /// 
    /// // Strange parts are full.
    /// assert!(strange_parts.insert(StrangePart::MedicsKilled).is_err());
    /// ```
    pub fn insert(&mut self, strange_part: StrangePart) -> Result<(), InsertError> {
        if self.contains(&strange_part) {
            return Err(InsertError::Duplicate);
        }
        
        for index in 0..STRANGE_PART_COUNT {
            if self.inner[index].is_none() {
                self.inner[index] = Some(strange_part);
                return Ok(());
            }
        }
        
        Err(InsertError::Full)
    }
    
    /// Removes a strange part.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let mut strange_parts = StrangePartSet::single(StrangePart::CriticalKills);
    /// 
    /// assert!(strange_parts.remove(&StrangePart::CriticalKills));
    /// assert!(!strange_parts.contains(&StrangePart::CriticalKills));
    /// ```
    pub fn remove(&mut self, strange_part: &StrangePart) -> bool {
        for s in self.inner.iter_mut() {
            if *s == Some(*strange_part) {
                *s = None;
                return true;
            }
        }
        
        false
    }
    
    /// Removes and returns the strange part in the set, if any, that is equal to the given one.
    pub fn take(&mut self, strange_part: &StrangePart) -> Option<StrangePart> {
        for s in self.inner.iter_mut() {
            if *s == Some(*strange_part) {
                *s = None;
                return Some(*strange_part);
            }
        }
        
        None
    }
    
    /// Returns `true` if the set contains no strange parts.
    pub fn is_empty(&self) -> bool {
        self.inner
            .iter()
            .all(|s| s.is_none())
    }
    
    /// Returns `true` if the set contains a strange part.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangePartSet::from([
    ///     Some(StrangePart::CriticalKills),
    ///     Some(StrangePart::DamageDealt),
    ///     None,
    /// ]);
    /// 
    /// assert!(strange_parts.contains(&StrangePart::CriticalKills));
    /// ```
    pub fn contains(&self, strange_part: &StrangePart) -> bool {
        self.inner.contains(&Some(*strange_part))
    }
    
    /// Returns the number of strange parts in the set.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangePartSet::from([
    ///     Some(StrangePart::CriticalKills),
    ///     Some(StrangePart::DamageDealt),
    ///     None,
    /// ]);
    /// 
    /// assert_eq!(strange_parts.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.inner
            .into_iter() // inner is Copy
            .filter(Option::is_some)
            .count()
    }
    
    /// Returns the strange parts that are in `self` but not in `other`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts1 = StrangePartSet::double(StrangePart::DamageDealt, StrangePart::CriticalKills);
    /// let strange_parts2 = StrangePartSet::double(StrangePart::DamageDealt, StrangePart::EngineersKilled);
    /// let difference = strange_parts1.difference(&strange_parts2);
    /// 
    /// assert_eq!(difference, StrangePartSet::single(StrangePart::CriticalKills));
    /// 
    /// let difference = strange_parts2.difference(&strange_parts1);
    /// 
    /// assert_eq!(difference, StrangePartSet::single(StrangePart::EngineersKilled));
    /// ```
    pub fn difference(&self, other: &Self) -> Self {
        let mut inner = [None, None, None];
        
        for (i, s_option) in inner.iter_mut().enumerate() {
            if let Some(s) = self.inner[i] {
                if !other.contains(&s) {
                    *s_option = Some(s);
                }
            }
        }
        
        Self {
            inner,
        }
    }
    
    /// Returns the strange parts that are both in `self` and `other`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts1 = StrangePartSet::double(StrangePart::DamageDealt, StrangePart::CriticalKills);
    /// let strange_parts2 = StrangePartSet::double(StrangePart::DamageDealt, StrangePart::EngineersKilled);
    /// let intersection = strange_parts1.intersection(&strange_parts2);
    /// 
    /// assert_eq!(intersection, StrangePartSet::single(StrangePart::DamageDealt));
    /// ```
    pub fn intersection(&self, other: &Self) -> Self {
        let mut inner = [None, None, None];
        
        for (i, s_option) in inner.iter_mut().enumerate() {
            if let Some(s) = self.inner[i] {
                if other.contains(&s) {
                    *s_option = Some(s);
                }
            }
        }
        
        Self {
            inner,
        }
    }
    
    /// Returns `true` if `self` has no strange parts in common with `other`. This is equivalent to 
    /// checking for an empty intersection.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangePartSet;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts1 = StrangePartSet::double(StrangePart::DamageDealt, StrangePart::CriticalKills);
    /// let strange_parts2 = StrangePartSet::double(StrangePart::DamageDealt, StrangePart::EngineersKilled);
    /// 
    /// assert!(!strange_parts1.is_disjoint(&strange_parts2));
    /// ```
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }
}

impl From<[Option<StrangePart>; STRANGE_PART_COUNT]> for StrangePartSet {
    fn from(inner: [Option<StrangePart>; STRANGE_PART_COUNT]) -> Self {
        let mut inner = inner;
        
        // remove duplicates
        for i in 0..STRANGE_PART_COUNT {
            for j in 0..STRANGE_PART_COUNT {
                if i == j {
                    continue;
                }
                
                if inner[i] == inner[j] {
                    inner[i] = None;
                }
            }
        }
        
        Self {
            inner,
        }
    }
}

// Only Sub is implemented because Add wouldn't make much sense with strange parts being limited 
// to 3.
impl Sub for StrangePartSet {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        self.difference(&other)
    }
}

impl PartialEq<Self> for StrangePartSet {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.inner;
        let mut b = other.inner;
        
        a.sort_unstable();
        b.sort_unstable();
        
        a == b
    }
}

impl Hash for StrangePartSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut values = self.inner;
        
        values.sort_unstable();
        
        for value in values {
            value.hash(state);
        }
    }
}

impl FromIterator<StrangePart> for StrangePartSet {
    fn from_iter<I: IntoIterator<Item = StrangePart>>(iter: I) -> Self {
        let mut strange_parts = Self::new();
        
        for strange_part in iter {
            strange_parts.insert(strange_part).ok();
        }
        
        strange_parts
    }
}

impl IntoIterator for StrangePartSet {
    type Item = StrangePart;
    type IntoIter = StrangePartSetIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        StrangePartSetIterator {
            inner: self,
            index: 0,
        }
    }
}

impl IntoIterator for &StrangePartSet {
    type Item = StrangePart;
    type IntoIter = StrangePartSetIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        StrangePartSetIterator {
            inner: *self,
            index: 0,
        }
    }
}

/// Iterates over strange parts.
#[derive(Debug)]
pub struct StrangePartSetIterator {
    inner: StrangePartSet,
    index: usize,
}

impl Iterator for StrangePartSetIterator {
    type Item = StrangePart;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(s) = self.inner.inner.get(self.index) {
            self.index += 1;
            
            if let Some(s) = s {
                // stop at first filled slot
                return Some(*s);
            }
        }
        
        None
    }
}

impl fmt::Display for StrangePartSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn iterates_strange_parts() {
        let strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        let mut iter = strange_parts.into_iter();
        
        assert_eq!(iter.next(), Some(StrangePart::TauntKills));
        assert_eq!(iter.next(), Some(StrangePart::KillsWhileExplosiveJumping));
        assert_eq!(iter.next(), Some(StrangePart::CriticalKills));
        assert_eq!(iter.next(), None);
        
        let mut count = 0;
        
        for _strange_part in &strange_parts {
            count += 1;
        }
        
        assert_eq!(count, 3);
    }
    
    #[test]
    fn mutates_strange_parts() {
        let mut strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        
        assert_eq!(strange_parts.len(), 3);
        assert!(strange_parts.contains(&StrangePart::CriticalKills));
        
        strange_parts.remove(&StrangePart::CriticalKills);
        
        assert!(!strange_parts.contains(&StrangePart::CriticalKills));
        assert_eq!(strange_parts.len(), 2);
        
        strange_parts.insert(StrangePart::DamageDealt).ok();
        
        assert!(strange_parts.contains(&StrangePart::DamageDealt));
        assert_eq!(strange_parts.len(), 3);
    }
    
    #[test]
    fn strange_parts_no_duplicates() {
        assert_eq!(StrangePartSet::from([
            Some(StrangePart::CriticalKills),
            Some(StrangePart::CriticalKills),
            Some(StrangePart::CriticalKills),
        ]), StrangePartSet::from([
            Some(StrangePart::CriticalKills),
            None,
            None,
        ]));
    }
    
    #[test]
    fn is_empty() {
        assert!(StrangePartSet::from([
            None,
            None,
            None,
        ]).is_empty());
    }
}
