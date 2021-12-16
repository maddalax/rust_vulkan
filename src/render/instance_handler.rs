use crate::render::instance;
use crate::render::instance::{InstanceType, MAX_INSTANCES};
use crate::Instance;
use cgmath::{Quaternion, Rotation3, Vector3};
use std::collections::HashMap;

pub struct InstanceHandler {
    pub(crate) instances: Vec<instance::Instance>,
    pub(crate) instance_changes: Vec<usize>,
    pub(crate) max_allowed_sizes: HashMap<InstanceType, usize>,
    pub(crate) max_index: usize,
    pub(crate) total_added: usize,
}

impl InstanceHandler {
    pub(crate) fn new() -> InstanceHandler {
        let mut instances = Vec::with_capacity(MAX_INSTANCES);
        for _ in 0..instances.capacity() {
            instances.push(Instance {
                instance_type: InstanceType::Empty,
                position: Vector3 {
                    x: (0.0),
                    y: (0.0),
                    z: (0.0),
                },
                rotation: Quaternion::from_angle_y(cgmath::Deg(2.0)),
                start_offset: 0,
                array_index: 0,
                max_allowed: 0,
            });
        }

        InstanceHandler {
            instances,
            instance_changes: Vec::new(),
            max_allowed_sizes: HashMap::new(),
            max_index: 0,
            total_added: 0,
        }
    }

    pub fn get(&mut self, index: usize) -> &mut Instance {
        return self.instances.get_mut(index).unwrap();
    }

    pub fn add(&mut self, mut instance: instance::Instance) {
        self.max_allowed_sizes
            .insert(instance.instance_type, instance.max_allowed);

        let offsets = self.find_offset(instance.instance_type);

        if offsets.0.is_none() {
            println!(
                "Could not find open slot for {}",
                instance.instance_type as u32
            );
            return;
        }

        let array_index = offsets.0.unwrap();
        instance.array_index = array_index;
        instance.start_offset = offsets.1;

        let mut o = instance.start_offset;
        if o == 0 {
            o = 1
        }

        if instance.array_index >= (instance.max_allowed + o) {
            return;
        }

        std::mem::replace(&mut self.instances[instance.array_index], instance);
        self.instance_changes.push(array_index);

        if array_index > self.max_index {
            self.max_index = array_index;
        }

        self.total_added += 1;

        // println!("Add Took: {} Ms", now.elapsed().as_millis());
        // println!("Total Entities: {}", self.total_added);
    }

    pub fn update(&mut self, index: usize) {
        self.instance_changes.push(index);
    }

    fn find_open_slot(&self, start_index: usize) -> Option<usize> {
        let mut offset = start_index;
        loop {
            if offset >= self.instances.len() {
                return None;
            }
            let instance = self.instances.get(offset).unwrap();
            if instance.instance_type == InstanceType::Empty {
                return Option::Some(offset);
            }
            offset += 1;
        }
    }

    fn find_offset(&self, instance_type: InstanceType) -> (Option<usize>, usize) {
        let mut offset = 0;
        loop {
            let instance = self.instances.get(offset).unwrap();

            if instance.instance_type == InstanceType::Empty {
                return (self.find_open_slot(offset), offset);
            }

            // Found a match of the same entity
            if instance.instance_type == instance_type {
                return (self.find_open_slot(offset), offset);
            } else {
                offset += instance.max_allowed;
            }
        }
    }
}
