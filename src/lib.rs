// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dependency-pinning: dependency pinning component model.
//!
//! Real implementation: pinnings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pinning {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyPinningError {
    EmptyPinningName,
    DuplicatePinning(String),
    EmptyEventSubject,
}

impl fmt::Display for DependencyPinningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPinningName => write!(f, "pinning name must not be empty"),
            Self::DuplicatePinning(id) => write!(f, "duplicate pinning {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DependencyPinningError {}

pub struct DependencyPinning {
    pub name: String,
    pinnings: BTreeMap<String, Pinning>,
    events: BTreeMap<String, Event>,
}

impl DependencyPinning {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pinnings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_pinning(&mut self, item: Pinning) -> Result<(), DependencyPinningError> {
        if item.name.trim().is_empty() {
            return Err(DependencyPinningError::EmptyPinningName);
        }
        let id = item.id.clone();
        if self.pinnings.contains_key(&id) {
            return Err(DependencyPinningError::DuplicatePinning(id.clone()));
        }
        self.pinnings.insert(id, item);
        Ok(())
    }

    pub fn pinning_count(&self) -> usize {
        self.pinnings.len()
    }

    pub fn get_pinning(&self, id: &str) -> Option<&Pinning> {
        self.pinnings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DependencyPinningError> {
        if event.subject.trim().is_empty() {
            return Err(DependencyPinningError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_pinning(id: &str, name: &str) -> Pinning {
        Pinning {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DependencyPinning::new("DependencyPinning");
        assert_eq!(m.name, "DependencyPinning");
        assert_eq!(m.pinning_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_pinning() {
        let mut m = DependencyPinning::new("DependencyPinning");
        m.add_pinning(sample_pinning("a1", "Alpha")).unwrap();
        assert_eq!(m.pinning_count(), 1);
        assert!(m.get_pinning("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DependencyPinning::new("DependencyPinning");
        assert!(m.add_pinning(sample_pinning("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DependencyPinning::new("DependencyPinning");
        m.add_pinning(sample_pinning("x", "One")).unwrap();
        assert!(m.add_pinning(sample_pinning("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DependencyPinning::new("DependencyPinning");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
