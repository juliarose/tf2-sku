//! Container for holding up to 3 strange parts.

use std::fmt;
use std::hash::{Hash, Hasher};
use tf2_enum::StrangePart;

/// Contains up to 3 strange parts.
/// 
/// # Examples
/// ```
/// use tf2_sku::StrangeParts;
/// use tf2_enum::StrangePart;
/// 
/// let strange_parts = StrangeParts::new([
///     Some(StrangePart::CriticalKills),
///     Some(StrangePart::DamageDealt),
///     None,
/// ]);
/// 
/// // Iterate over strange parts.
/// for strange_part in strange_parts {
///     println!("{strange_part}");
/// }
/// 
/// // Can hold up to 3 strange parts, however empty strange parts do not count towards length.
/// assert_eq!(strange_parts.len(), 2);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq)]
pub struct StrangeParts {
    inner: [Option<StrangePart>; 3],
}

impl StrangeParts {
    /// Creates a container for strange parts.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangeParts;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangeParts::new([
    ///     Some(StrangePart::CriticalKills),
    ///     Some(StrangePart::DamageDealt),
    ///     None,
    /// ]);
    /// ```
    pub fn new(inner: [Option<StrangePart>; 3]) -> Self {
        let mut inner = inner;
        
        // remove duplicates
        for i in 0..=2 {
            for j in 0..=2 {
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
    
    /// Clears strange parts.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangeParts;
    /// use tf2_enum::StrangePart;
    /// 
    /// let mut strange_parts = StrangeParts::new([
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
    /// # Examples
    /// ```
    /// use tf2_sku::StrangeParts;
    /// use tf2_enum::StrangePart;
    /// 
    /// let mut strange_parts = StrangeParts::new([
    ///     Some(StrangePart::CriticalKills),
    ///     Some(StrangePart::DamageDealt),
    ///     None,
    /// ]);
    /// 
    /// assert_eq!(strange_parts.len(), 2);
    /// 
    /// strange_parts.push(StrangePart::EngineersKilled);
    /// 
    /// assert_eq!(strange_parts.len(), 3);
    /// ```
    pub fn push(&mut self, strange_part: StrangePart) {
        if self.contains(&strange_part) {
            return;
        }
        
        for index in 0..=2usize {
            if self.inner[index].is_none() {
                self.inner[index] = Some(strange_part);
                return;
            }
        }
    }
    
    /// Removes a strange part.
    pub fn remove(&mut self, strange_part: StrangePart) {
        for s in self.inner.iter_mut() {
            if *s == Some(strange_part) {
                *s = None;
                
                return;
            }
        }
    }
    
    /// Checks if strange parts are empty.
    pub fn is_empty(&self) -> bool {
        self.inner
            .iter()
            .all(|s| s.is_none())
    }
    
    /// Checks if contains strange part.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangeParts;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangeParts::new([
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
    
    /// Gets the length of contained strange parts.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::StrangeParts;
    /// use tf2_enum::StrangePart;
    /// 
    /// let strange_parts = StrangeParts::new([
    ///     Some(StrangePart::CriticalKills),
    ///     Some(StrangePart::DamageDealt),
    ///     None,
    /// ]);
    /// 
    /// assert_eq!(strange_parts.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.inner
            .iter()
            .filter(|s| s.is_some())
            .count()
    }
}

impl PartialEq<Self> for StrangeParts {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.inner;
        let mut b = other.inner;
        
        a.sort_unstable();
        b.sort_unstable();
        
        a == b
    }
}

impl Hash for StrangeParts {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut values = self.inner;
        
        values.sort_unstable();
        
        for value in values {
            value.hash(state);
        }
    }
}

impl IntoIterator for StrangeParts {
    type Item = StrangePart;
    type IntoIter = StrangePartsIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        StrangePartsIterator {
            inner: self,
            index: 0,
        }
    }
}

impl IntoIterator for &StrangeParts {
    type Item = StrangePart;
    type IntoIter = StrangePartsIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        StrangePartsIterator {
            inner: *self,
            index: 0,
        }
    }
}

/// Iterates over strange parts.
#[derive(Debug)]
pub struct StrangePartsIterator {
    inner: StrangeParts,
    index: usize,
}

impl Iterator for StrangePartsIterator {
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

impl fmt::Display for StrangeParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn iterates_strange_parts() {
        let strange_parts = StrangeParts::new([
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
        let mut strange_parts = StrangeParts::new([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        
        assert_eq!(strange_parts.len(), 3);
        assert!(strange_parts.contains(&StrangePart::CriticalKills));
        
        strange_parts.remove(StrangePart::CriticalKills);
        
        assert!(!strange_parts.contains(&StrangePart::CriticalKills));
        assert_eq!(strange_parts.len(), 2);
        
        strange_parts.push(StrangePart::DamageDealt);
        
        assert!(strange_parts.contains(&StrangePart::DamageDealt));
        assert_eq!(strange_parts.len(), 3);
    }
    
    #[test]
    fn strange_parts_no_duplicates() {
        assert_eq!(StrangeParts::new([
            Some(StrangePart::CriticalKills),
            Some(StrangePart::CriticalKills),
            Some(StrangePart::CriticalKills),
        ]), StrangeParts::new([
            Some(StrangePart::CriticalKills),
            None,
            None,
        ]));
    }
    
    #[test]
    fn is_empty() {
        assert!(StrangeParts::new([
            None,
            None,
            None,
        ]).is_empty());
    }
}