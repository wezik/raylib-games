use crate::systems::building_system::BuildingType;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    BuildingMenuOpened,
    BuildingSelected(BuildingType),
}

#[derive(Default, Debug)]
pub struct EventBus {
    events: Vec<Event>,
}

impl EventBus {
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Consume first event matching predicate and return it
    pub fn fetch<F>(&mut self, predicate: F) -> Option<Event>
    where
        F: Fn(&Event) -> bool,
    {
        let event_id = self.events.iter().enumerate().find(|(_, e)| predicate(e)).map(|(i, _)| i);
        if let Some(event_id) = event_id {
            Some(self.events.remove(event_id))
        } else {
            None
        }
    }

    /// Consume all events matching predicate and return it
    pub fn fetchAll<F>(&mut self, predicate: F) -> Vec<Event>
    where
        F: Fn(&Event) -> bool,
    {
        let events = self.events.iter().filter(|e| predicate(e)).cloned().collect();
        self.events.retain(|e| !predicate(e));
        events
    }
}
