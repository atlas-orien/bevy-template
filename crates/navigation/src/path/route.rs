use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default, PartialEq)]
pub struct NavigationPath2d {
    pub waypoints: Vec<Vec2>,
    pub next_waypoint: usize,
}

#[derive(Component, Debug, Clone, Default, PartialEq)]
pub struct NavigationPath3d {
    pub waypoints: Vec<Vec3>,
    pub next_waypoint: usize,
}

impl NavigationPath2d {
    pub fn new(waypoints: Vec<Vec2>) -> Self {
        Self {
            waypoints,
            next_waypoint: 0,
        }
    }

    pub fn current_waypoint(&self) -> Option<Vec2> {
        self.waypoints.get(self.next_waypoint).copied()
    }

    pub fn advance(&mut self) {
        if self.next_waypoint < self.waypoints.len() {
            self.next_waypoint += 1;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.next_waypoint >= self.waypoints.len()
    }

    pub fn clear(&mut self) {
        self.waypoints.clear();
        self.next_waypoint = 0;
    }
}

impl NavigationPath3d {
    pub fn new(waypoints: Vec<Vec3>) -> Self {
        Self {
            waypoints,
            next_waypoint: 0,
        }
    }

    pub fn current_waypoint(&self) -> Option<Vec3> {
        self.waypoints.get(self.next_waypoint).copied()
    }

    pub fn advance(&mut self) {
        if self.next_waypoint < self.waypoints.len() {
            self.next_waypoint += 1;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.next_waypoint >= self.waypoints.len()
    }

    pub fn clear(&mut self) {
        self.waypoints.clear();
        self.next_waypoint = 0;
    }
}
