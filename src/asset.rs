use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::{anyhow, bail, Result};
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::HashMap,
};
use paperdoll_tar::paperdoll::{Doll, Fragment, Paperdoll, PaperdollFactory, Slot};

pub type PaperdollId = u32;

enum SetSlotBy {
    Empty,
    FragmentId(u32),
}

/// A structure storing the paperdoll asset and all paperdolls you created.
#[derive(TypePath, TypeUuid)]
#[uuid = "f5c9a7ce-3a4a-11ee-be56-0242ac120002"]
pub struct PaperdollAsset {
    factory: PaperdollFactory,
    id_to_paperdoll: HashMap<PaperdollId, Paperdoll>,
    id_to_texture: HashMap<PaperdollId, Image>,
}

impl PaperdollAsset {
    pub fn new(factory: PaperdollFactory) -> Self {
        Self {
            factory,
            id_to_paperdoll: HashMap::new(),
            id_to_texture: HashMap::new(),
        }
    }

    /// Creates a paperdoll from this asset.
    ///
    /// Returns the id used to refer to this paperdoll for later usage.
    pub fn create_paperdoll(&mut self, doll_id: u32) -> PaperdollId {
        let mut paperdoll = self.factory.builder().doll(doll_id).build();

        let id = get_id();

        let slots = self
            .factory
            .get_doll(paperdoll.doll)
            .map(|doll| {
                doll.slots
                    .iter()
                    .map(|slot_id| self.factory.get_slot(*slot_id))
                    .filter(|slot| slot.is_some())
                    .map(|slot| slot.unwrap())
                    .collect::<Vec<&Slot>>()
            })
            .unwrap_or_default();

        for slot in slots {
            if !slot.required {
                continue;
            }

            if let Some(fragment_id) = slot.candidates.first() {
                paperdoll.slot_map.insert(slot.id(), *fragment_id);
            }
        }

        self.id_to_paperdoll.insert(id, paperdoll);

        self.update_texture(id);

        id
    }

    /// Gets all dolls available in this asset.
    pub fn get_dolls(&self) -> Vec<&Doll> {
        self.factory
            .dolls()
            .map(|(_, doll)| doll)
            .collect::<Vec<&Doll>>()
    }

    /// Gets all fragments those can be used in this slot.
    pub fn get_fragments_by_slot(&self, slot_id: u32) -> Vec<&Fragment> {
        self.factory
            .get_slot(slot_id)
            .map(|slot| {
                slot.candidates
                    .iter()
                    .map(|fragment_id| self.factory.get_fragment(*fragment_id))
                    .filter(|fragment| fragment.is_some())
                    .map(|fragment| fragment.unwrap())
                    .collect::<Vec<&Fragment>>()
            })
            .unwrap_or_default()
    }

    /// Gets the fragment currently used in this slot.
    pub fn get_slot_fragment(&self, id: PaperdollId, slot_id: u32) -> Option<&Fragment> {
        let fragment_id = self
            .id_to_paperdoll
            .get(&id)
            .map(|paperdoll| paperdoll.slot_map.get(&slot_id))
            .flatten();

        fragment_id
            .map(|fragment_id| self.factory.get_fragment(*fragment_id))
            .flatten()
    }

    /// Gets all slots in this paperdoll.
    pub fn get_slots(&self, id: PaperdollId) -> Vec<&Slot> {
        let doll_id = self
            .id_to_paperdoll
            .get(&id)
            .map(|paperdoll| paperdoll.doll);

        let doll = doll_id
            .map(|doll_id| self.factory.get_doll(doll_id))
            .flatten();

        doll.map(|doll| {
            doll.slots
                .iter()
                .map(|slot_id| self.factory.get_slot(*slot_id))
                .filter(|slot| slot.is_some())
                .map(|slot| slot.unwrap())
                .collect::<Vec<&Slot>>()
        })
        .unwrap_or_default()
    }

    /// Gets a reference to the image for the given paperdoll.
    pub fn get_texture(&self, id: PaperdollId) -> Option<&Image> {
        self.id_to_texture.get(&id)
    }

    /// Removes a paperdoll.
    ///
    /// Returns the removed paperdoll if it previously existed, otherwise returns [`None`].
    pub fn remove_paperdoll(&mut self, id: PaperdollId) -> Option<Paperdoll> {
        let paperdoll = self.id_to_paperdoll.remove(&id);

        self.id_to_texture.remove(&id);

        paperdoll
    }

    /// Sets the given slot to empty.
    ///
    /// # Errors
    ///
    /// - Will return an error if the slot is required.
    pub fn slot_use_empty(&mut self, id: PaperdollId, slot_id: u32) -> Result<()> {
        self.set_slot(id, slot_id, SetSlotBy::Empty)
    }

    /// Sets the given slot to the given fragment.
    ///
    /// # Errors
    ///
    /// - Will return an error if the fragment is not a candidate of this slot.
    pub fn slot_use_fragment(
        &mut self,
        id: PaperdollId,
        slot_id: u32,
        fragment_id: u32,
    ) -> Result<()> {
        self.set_slot(id, slot_id, SetSlotBy::FragmentId(fragment_id))
    }

    /// Sets the given slot to the nth fragment of its candidates.
    ///
    /// # Errors
    ///
    /// - Will return an error if the index is invalid.
    pub fn slot_use_index(&mut self, id: PaperdollId, slot_id: u32, index: usize) -> Result<()> {
        let fragment = self.find_fragment_in_candidates_by_index(slot_id, index)?;

        self.set_slot(id, slot_id, SetSlotBy::FragmentId(fragment.id()))
    }

    /// Sets the given slot to the next fragment of its candidates.
    ///
    /// If the current fragment is the last one, then:
    ///
    /// - If the slot is required, set to the first fragment.
    /// - If the slot is not required, set to empty.
    ///
    /// If it is an empty slot, set to the first fragment.
    pub fn slot_use_next(&mut self, id: PaperdollId, slot_id: u32) -> Result<()> {
        let fragment = self.get_slot_fragment(id, slot_id);

        let slot = self.get_slot(slot_id)?;

        let position = match fragment {
            Some(fragment) => self.find_fragment_index_in_candidates(slot_id, fragment.id())?,
            None => {
                if slot.required {
                    bail!("Slot {} has no valid fragment set.", slot_id)
                }

                usize::MAX
            }
        };

        let mut position_next = position.wrapping_add(1);

        if position_next >= slot.candidates.len() {
            if slot.required {
                position_next = 0;
            } else {
                return self.set_slot(id, slot_id, SetSlotBy::Empty);
            }
        }

        let fragment = self.find_fragment_in_candidates_by_index(slot_id, position_next)?;

        self.set_slot(id, slot_id, SetSlotBy::FragmentId(fragment.id()))
    }

    /// Sets the given slot to the previous fragment of its candidates.
    ///
    /// If the current fragment is the first one, then:
    ///
    /// - If the slot is required, set to the last fragment.
    /// - If the slot is not required, set to empty.
    ///
    /// If it is an empty slot, set to the last fragment.
    pub fn slot_use_prev(&mut self, id: PaperdollId, slot_id: u32) -> Result<()> {
        let fragment = self.get_slot_fragment(id, slot_id);

        let slot = self.get_slot(slot_id)?;

        let position = match fragment {
            Some(fragment) => self.find_fragment_index_in_candidates(slot_id, fragment.id())?,
            None => {
                if slot.required {
                    bail!("Slot {} has no valid fragment set.", slot_id)
                }

                slot.candidates.len()
            }
        };

        let mut position_prev = position - 1;

        if position_prev == usize::MAX {
            if slot.required {
                position_prev = slot.candidates.len() - 1;
            } else {
                return self.set_slot(id, slot_id, SetSlotBy::Empty);
            }
        }

        let fragment = self.find_fragment_in_candidates_by_index(slot_id, position_prev)?;

        self.set_slot(id, slot_id, SetSlotBy::FragmentId(fragment.id()))
    }

    /// Gets the ownership of the image for the given paperdoll.
    pub fn take_texture(&mut self, id: PaperdollId) -> Option<Image> {
        self.id_to_texture.remove(&id)
    }

    fn find_fragment_in_candidates_by_index(
        &self,
        slot_id: u32,
        index: usize,
    ) -> Result<&Fragment> {
        let slot = self.get_slot(slot_id)?;

        let fragment_id = slot.candidates.iter().nth(index).ok_or(anyhow!(
            "Index out of range: '{}' in candidates of slot {}.",
            index,
            slot_id
        ))?;

        self.get_fragment(*fragment_id)
    }

    fn find_fragment_index_in_candidates(&self, slot_id: u32, fragment_id: u32) -> Result<usize> {
        let slot = self.get_slot(slot_id)?;

        slot.candidates
            .iter()
            .position(|id| *id == fragment_id)
            .ok_or(anyhow!(
                "Fragment {} is not a candidate for slot {}.",
                fragment_id,
                slot_id
            ))
    }

    fn get_fragment(&self, fragment_id: u32) -> Result<&Fragment> {
        self.factory
            .get_fragment(fragment_id)
            .ok_or(anyhow!("Fragment with id '{}' not found.", fragment_id))
    }

    fn get_slot(&self, slot_id: u32) -> Result<&Slot> {
        self.factory
            .get_slot(slot_id)
            .ok_or(anyhow!("Slot with id '{}' not found.", slot_id))
    }

    fn set_slot(&mut self, id: PaperdollId, slot_id: u32, set_slot_by: SetSlotBy) -> Result<()> {
        if !self.id_to_paperdoll.contains_key(&id) {
            bail!("Paperdoll with id '{}' not found.", id)
        }

        let slot = self.get_slot(slot_id)?;

        match set_slot_by {
            SetSlotBy::Empty => {
                if slot.required {
                    bail!("Slot {} cannot be empty.", slot_id)
                }

                if let Some(paperdoll) = self.id_to_paperdoll.get_mut(&id) {
                    paperdoll.slot_map.remove(&slot_id);
                }
            }
            SetSlotBy::FragmentId(fragment_id) => {
                let slot = self.get_slot(slot_id)?;

                if !slot.candidates.contains(&fragment_id) {
                    bail!(
                        "Slot {} does not accept fragment {} as a candidate.",
                        slot_id,
                        fragment_id
                    )
                }

                if let Some(paperdoll) = self.id_to_paperdoll.get_mut(&id) {
                    paperdoll.slot_map.insert(slot_id, fragment_id);
                }
            }
        }

        self.update_texture(id)?;

        Ok(())
    }

    fn update_texture(&mut self, id: PaperdollId) -> Result<()> {
        if let Some(paperdoll) = self.id_to_paperdoll.get(&id) {
            let texture = self.factory.render_paperdoll(&paperdoll)?;

            let image = Image::new(
                Extent3d {
                    width: texture.width,
                    height: texture.height,
                    ..Default::default()
                },
                TextureDimension::D2,
                texture.pixels,
                TextureFormat::Rgba8UnormSrgb,
            );

            self.id_to_texture.insert(id, image);

            return Ok(());
        }

        bail!("Paperdoll with id '{}' not found.", id)
    }
}

fn get_id() -> PaperdollId {
    static ID: AtomicU32 = AtomicU32::new(1);

    ID.fetch_add(1, Ordering::Relaxed)
}
