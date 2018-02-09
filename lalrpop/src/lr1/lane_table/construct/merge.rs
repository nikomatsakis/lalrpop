use super::*;

use collections::Multimap;
use lr1::lane_table::table::context_set::ContextSet;
use self::group::*;

/// The "merge" phase of the algorithm is described in "Step 3c" of
/// [the README][r].  It consists of walking through the various
/// states in the lane table and merging them into sets of states that
/// have compatible context sets; if we encounter a state S that has a
/// successor T but where the context set of S is not compatible with
/// T, then we will clone T into a new T2 (and hopefully the context
/// set of S will be compatible with the reduced context of T2).
///
/// [r]: ../README.md
pub struct Merge<'m, 'grammar: 'm> {
    table: &'m LaneTable<'grammar>,
    states: &'m mut Vec<LR1State<'grammar>>,
    visited: Set<StateIndex>,
    original_indices: Map<StateIndex, StateIndex>,
    clones: Multimap<StateIndex, Vec<StateIndex>>,
    target_states: Vec<StateIndex>,
    groups: Groups,
    rows: Map<StateIndex, ContextSet>
}

impl<'m, 'grammar> Merge<'m, 'grammar> {
    pub fn new(table: &'m LaneTable<'grammar>,
               states: &'m mut Vec<LR1State<'grammar>>,
               inconsistent_state: StateIndex,
               rows: Map<StateIndex, ContextSet>)
               -> Self {
        let count_of_states = states.len();
        Merge {
            table: table,
            states: states,
            visited: Set::new(),
            original_indices: Map::new(),
            clones: Multimap::new(),
            target_states: vec![inconsistent_state],
            groups: Groups::new(count_of_states),
            rows: rows
        }
    }

    pub fn start(&mut self, beachhead_state: StateIndex) -> Result<(), (StateIndex, StateIndex)> {
        debug!("Merge::start(beachhead_state={:?})", beachhead_state);

        let group = self.groups.allocate(beachhead_state, self.rows.get(&beachhead_state));

        // Since we always start walks from beachhead states, and they
        // are not reachable from anyone else, this state should not
        // have been unioned with anything else yet.
        self.walk(beachhead_state, group)
    }

    pub fn patch_target_starts(mut self, actions: &Set<Action<'grammar>>) {
        debug!("Merge::patch_target_starts(actions={:?})", actions);

        for &target_state in &self.target_states {
            debug!("Merge::patch_target_starts: target_state={:?}", target_state);
            let context_set = self.groups.context_set_ref_for_state(target_state);
            debug!("Merge::patch_target_starts: context_set={:?}", context_set);
            context_set.apply(&mut self.states[target_state.0], actions);
        }
    }

    /// If `state` is a cloned state, find its original index.  Useful
    /// for indexing into the lane table and so forth.
    fn original_index(&self, state: StateIndex) -> StateIndex {
        *self.original_indices.get(&state).unwrap_or(&state)
    }

    fn successors(&self, state: StateIndex) -> Option<&'m Set<StateIndex>> {
        self.table.successors(self.original_index(state))
    }

    fn walk(&mut self, state: StateIndex, group: Group) -> Result<(), (StateIndex, StateIndex)> {
        debug!("Merge::walk(state={:?}, group={:?})", state, group);

        if !self.visited.insert(state) {
            debug!("Merge::walk: visited already");
            return Ok(());
        }

        for &successor in self.successors(state).iter().flat_map(|&s| s) {
            
            let successor_group_opt = self.groups.group(successor);

            debug!("Merge::walk: state={:?} successor={:?} successor_group_opt={:?}",
                   state, successor, successor_group_opt);
            if let Some(successor_group) = successor_group_opt {
                let mut successor = successor;
                if successor_group != group {
                    if (self.groups.try_merge_groups(group, successor_group)) {
                        debug!("Merge::walk: successful union, context-set = {:?}",
                            self.groups.context_set_ref(group));
                    } else {
                        successor = self.split(state, group, successor)?
                    }
                } else {

                }
                self.walk(successor, group)?
            } else {
                debug!("Merge::walk: state without group, context-set = {:?}",
                    self.groups.context_set_ref(group));
                // Successor does not belong to any group, so we just try
                // to merge it into current group
                if self.groups.merge_state(group, successor, self.rows.get(&successor)) {
                    self.walk(successor, group)?
                } else {
                    debug!("Merge::walk: failed");
                    return Err((self.original_index(state), self.original_index(successor)))
                }
            }
        }
        Ok(())
    }

    fn split(&mut self, state: StateIndex, group: Group, successor: StateIndex) -> Result<StateIndex, (StateIndex, StateIndex)> {
        // search for an existing clone with which we can merge
        debug!("Merge::walk: union failed, seek existing clone");
        let existing_clone = {
            let groups = &mut self.groups;
            let rows = &mut self.rows;
            self.clones.get(&successor)
                        .into_iter()
                        .flat_map(|clones| clones) // get() returns an Option<Set>
                        .cloned()
                        .filter(|&successor1| groups.merge_state(group, successor1, rows.get(&successor1)))
                        .next()
        };

        if let Some(successor1) = existing_clone {
            debug!("Merge::walk: found existing clone {:?}", successor1);
            self.patch_links(state, successor, successor1);
            Ok(successor1)
        } else {
            // if we don't find one, we have to make a new clone
            debug!("Merge::walk: creating new clone of {:?}", successor);
            let successor1 = self.clone(successor);
            if self.groups.merge_state(group, successor1, self.rows.get(&successor1)) {
                self.patch_links(state, successor, successor1);
                Ok(successor1)
            } else {
                debug!("Merge::walk: failed to union {:?} with {:?}",
                        group, successor1);
                debug!("Merge::walk: group context = {:?}",
                        self.groups.context_set_ref(group));
                debug!("Merge::walk: successor context = {:?}",
                        self.rows.get(&successor1));

                Err((self.original_index(state), self.original_index(successor1)))
            }
        }
    }

    fn clone(&mut self, state: StateIndex) -> StateIndex {
        // create a new state with same contents as the old one
        let new_index = StateIndex(self.states.len());
        let new_state = self.states[state.0].clone();

        self.states.push(new_state);
        // track the original index and clones
        let original_index = self.original_index(state);
        self.original_indices.insert(new_index, original_index);
        self.clones.push(original_index, new_index);

        // create a new unify key for this new state
        self.groups.add_state(new_index);

        // keep track of the clones of the target state
        if original_index == self.target_states[0] {
            self.target_states.push(new_index);
        }

        debug!("Merge::clone: cloned {:?} to {:?}", state, new_index);
        new_index
    }

    fn patch_links(&mut self,
                   predecessor: StateIndex,
                   original_successor: StateIndex,
                   cloned_successor: StateIndex)
    {
        let replace = |target_state: &mut StateIndex| {
            if *target_state == original_successor {
                *target_state = cloned_successor;
            }
        };

        let state = &mut self.states[predecessor.0];
        for (_, target_state) in &mut state.shifts {
            replace(target_state);
        }
        for (_, target_state) in &mut state.gotos {
            replace(target_state);
        }
    }
}

