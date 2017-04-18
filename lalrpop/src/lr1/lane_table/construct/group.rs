use lr1::lane_table::table::context_set::{ContextSet, OverlappingLookahead};
use lr1::core::*;
use std::cmp::max;
use std::fmt;
use std::mem::replace;
use ena::unify::{UnifyKey, UnifyValue, UnificationTable};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    index: usize
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "G{}", self.index)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupUnifyKey {
    index : u32
}

impl UnifyKey for GroupUnifyKey {
    type Value = Group;

    fn index(&self) -> u32 {
        self.index
    }

    fn from_index(u: u32) -> Self {
        GroupUnifyKey { index: u }
    }

    fn tag() -> &'static str {
        "GroupUnifyKey"
    }
}

impl UnifyValue for Group {
    fn unify_values(value1: &Self, value2: &Self) -> Result<Self, (Self, Self)> {
        // correctness of merging algorithm relies on order used here
        Ok(max(*value1, *value2))
    }
}

pub struct Groups {
    ///! groups maps state index into group
    groups:             Vec<Option<Group>>,
    ///! maps group into unification key
    context_sets:       Vec<ContextSet>,
    unification_table:  UnificationTable<GroupUnifyKey>,
    ///! maps group into unification key
    unification_keys:   Vec<GroupUnifyKey>,
}

impl Groups {
    pub fn new(number_of_states : usize) -> Self {
        let groups = vec![None; number_of_states];
        Groups {
            groups:             groups,
            context_sets:       Vec::new(),
            unification_keys:   Vec::new(),
            unification_table:  UnificationTable::new(),
        }
    }

    pub fn allocate(&mut self, state: StateIndex, context_set : ContextSet) -> Group {
        let group = Group { index : self.context_sets.len() };
        debug!("Groups::allocate: allocated group={:?} for state={:?} context_set={:?}", group, state, context_set);
        self.context_sets.push(context_set);
        self.groups[state.0] = Some(group);
        self.unification_keys.push(self.unification_table.new_key(group));
        group
    }

    pub fn has_group(&self, state : StateIndex) -> bool {
        self.groups[state.0].is_some()
    }

    pub fn group(&self, state : StateIndex) -> Option<Group> {
        self.groups[state.0]
    }

    pub fn merge_state(&mut self, group: Group, state : StateIndex, context_set : &ContextSet)
        -> Result<(), (StateIndex, StateIndex)>
    {
        self.groups[state.0] = Some(group);
        match self.context_set_mutref(group).inplace_union(context_set) {
            // TODO: error reporting
            Err(_) => {
                debug!("Group::merge_state: failed to merge group={:?} with state={:?}", group, state);
                Err((StateIndex(0), StateIndex(0)))
            } 
            Ok(u) => {
                debug!("Group::merge_state: successfull merge group={:?} with state={:?} context_set={:?}", group, state, self.context_set_ref(group));
                Ok(u)
            } 
        }
    }

    fn unify_key(&self, group: Group) -> GroupUnifyKey {
        self.unification_keys[group.index]
    }

    fn context_set(&mut self, group: Group) -> ContextSet {
        let key = self.unify_key(group);
        let empty = ContextSet::new(0);
        replace(&mut self.context_sets[self.unification_table.probe_value(key).index], empty)
    }

    fn context_set_mutref(&mut self, group: Group) -> &mut ContextSet {
        let key = self.unify_key(group);
        &mut self.context_sets[self.unification_table.probe_value(key).index]
    }

    pub fn context_set_ref(&mut self, group: Group) -> &ContextSet {
        let key = self.unify_key(group);
        &self.context_sets[self.unification_table.probe_value(key).index]
    }

    pub fn unify_groups(&mut self, group1: Group, group2: Group) {
        let key1 = self.unify_key(group1);
        let key2 = self.unify_key(group2);
        assert!(self.unification_table.unify_var_var(key1, key2).is_ok())
    }

    pub fn merge_groups(&mut self, group1: Group, group2: Group) -> bool {
        let context_set = {
            // Inefficient since it creates new context-set 
            // instead of merging in-place. It will be handled later.
            let context_set1 = self.context_set(group1);
            let context_set2 = self.context_set(group2);
            debug!("Groups::merge_groups: group={:?} context_set{:?}", group1, context_set1);
            debug!("Groups::merge_groups: group={:?} context_set{:?}", group2, context_set2);
            match ContextSet::union(&context_set1, &context_set2) {
                Ok(context_set) => context_set,
                Err(_) => {
                    debug!("Groups::merge_grops: merging groups {:?} and {:?} failed", group1, group2);
                    return false
                }
            }
        };

        self.unify_groups(group1, group2);
        debug!("Groups::merge_groups: successfull merge of groups {:?} and {:?} context_set={:?}", group1, group2, context_set);
        self.context_sets[max(group1, group2).index] = context_set;
        true
    }

}