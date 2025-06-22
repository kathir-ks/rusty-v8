// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, HashSet, LinkedList, VecDeque};
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;
use std::string::ToString;

mod implementation_visitor;
mod global_context;
mod type_oracle;
mod class_type;
mod type_alias;

use crate::class_type::{ClassType, InstanceTypeConstraints};
use crate::global_context::{Declarable, GlobalContext};
use crate::implementation_visitor::ImplementationVisitor;
use crate::type_alias::TypeAlias;
use crate::type_oracle::TypeOracle;

/// Contains all necessary state for a single class type during the process of
/// assigning instance types, and provides a convenient way to access the list of
/// types that inherit from this one.
#[derive(Debug)]
struct InstanceTypeTree<'a> {
    type_: &'a ClassType,
    children: Vec<Box<InstanceTypeTree<'a>>>,
    start: i32, // Start of range for this and subclasses, or i32::MAX.
    end: i32,   // End of range for this and subclasses, or i32::MIN.
    value: i32, // Assigned value for this class itself, or -1 when unassigned.
    num_values: i32, // Number of values assigned for this and subclasses.
    num_own_values: i32, // How many values this needs (not including subclasses).
}

impl<'a> InstanceTypeTree<'a> {
    fn new(type_: &'a ClassType) -> Self {
        InstanceTypeTree {
            type_,
            children: Vec::new(),
            start: i32::MAX,
            end: i32::MIN,
            value: -1,
            num_values: 0,
            num_own_values: 0,
        }
    }
}

/// Assembles all class types into a tree, but doesn't yet attempt to assign
/// instance types for them.
fn build_instance_type_tree<'a>() -> Option<Box<InstanceTypeTree<'a>>> {
    // First, build InstanceTypeTree instances for every class but don't try to
    // attach them to their subclasses yet.
    let mut map_by_type: HashMap<*const ClassType, *mut InstanceTypeTree> = HashMap::new();
    let mut unparented_types: Vec<Box<InstanceTypeTree>> = Vec::new();

    for p in GlobalContext::all_declarables() {
        if let Some(alias) = p.downcast_ref::<TypeAlias>() {
            let type_ = alias.type_();
            if let Some(class_type) = type_.downcast_ref::<ClassType>() {
                if map_by_type.contains_key(&(class_type as *const ClassType)) {
                    continue; // We already encountered this type.
                }
                let mut type_tree = Box::new(InstanceTypeTree::new(class_type));
                let type_tree_ptr = &mut *type_tree as *mut InstanceTypeTree;
                map_by_type.insert(class_type as *const ClassType, type_tree_ptr);
                unparented_types.push(type_tree);
            }
        }
    }

    // Second, assemble them all into a tree following the inheritance hierarchy.
    let mut root: Option<Box<InstanceTypeTree>> = None;
    for mut type_tree in unparented_types {
        let parent = type_tree.type_.get_super_class();
        if parent.is_none() {
            if root.is_some() {
               panic!("Expected only one root class type. Found: {:?} and {:?}", root.as_ref().unwrap().type_.name(), type_tree.type_.name());
               //Error("Expected only one root class type. Found: ", root.type.name(),
               //   " and ", type_tree.type.name())
               //.Position(type_tree.type.GetPosition());
            }
            root = Some(type_tree);
        } else {
            let parent_ptr = parent.unwrap() as *const ClassType;
            let parent_tree_ptr = map_by_type.get(&parent_ptr).unwrap();
            unsafe {
              (*parent_tree_ptr).children.push(type_tree);
            }
        }
    }
    root
}

/// Propagates constraints about instance types from children to their parents.
fn propagate_instance_type_constraints<'a>(root: &mut InstanceTypeTree<'a>) {
    for child in &mut root.children {
        propagate_instance_type_constraints(child);
        root.start = min(root.start, child.start);
        root.end = max(root.end, child.end);
        root.num_values += child.num_values;
    }
    let constraints = root.type_.get_instance_type_constraints();
    if !root.type_.is_abstract() && !root.type_.has_same_instance_type_as_parent() {
        root.num_own_values = 1;
    }
    root.num_values += root.num_own_values;
    if constraints.num_flags_bits != -1 {
        // Children won't get any types assigned; must be done manually in C++.
        root.children.clear();
        root.num_values = 1 << constraints.num_flags_bits;
        root.num_own_values = root.num_values;
        root.start = 0;
        root.end = root.num_values - 1;
    }
    if constraints.value != -1 {
        if root.num_own_values != 1 {
            panic!("Instance type value requested for abstract class {:?}", root.type_.name());
           // Error("Instance type value requested for abstract class ",
           //     root.type.name())
           // .Position(root.type.GetPosition());
        }
        root.value = constraints.value;
        root.start = min(root.start, constraints.value);
        root.end = max(root.end, constraints.value);
    }
}

/// Assigns values for the type itself, not including any children. Returns the
/// next available value.
fn select_own_values(root: &mut InstanceTypeTree, start_value: i32) -> i32 {
    if root.value == -1 {
        root.value = start_value;
    } else if root.value < start_value {
        panic!("Failed to assign instance type {} to {}", root.value, root.type_.name());
        //Error("Failed to assign instance type ", root.value, " to ",
        //  root.type.name())
        //.Position(root.type.GetPosition());
    }
    root.value + root.num_own_values
}

/// Sorting function for types that don't have specific values they must include.
/// Prioritizes bigger type ranges (those with more subtypes) first, and
/// then sorts alphabetically within each size category.
#[derive(PartialEq, Eq)]
struct CompareUnconstrainedTypes<'a> {
    a: &'a InstanceTypeTree<'a>,
    b: &'a InstanceTypeTree<'a>,
}

impl<'a> PartialOrd for CompareUnconstrainedTypes<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for CompareUnconstrainedTypes<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.a.num_values > other.a.num_values {
            std::cmp::Ordering::Greater
        } else if self.a.num_values < other.a.num_values {
            std::cmp::Ordering::Less
        } else {
            self.a.type_.name().cmp(self.b.type_.name())
        }
    }
}

/// Assigns concrete values for every instance type range, and sorts the children
/// at each layer of the tree into increasing order. Appends the newly-assigned
/// tree to the destination vector. Returns the first unassigned value after
/// those that have been used.
fn solve_instance_type_constraints<'a>(
    mut root: Box<InstanceTypeTree<'a>>,
    start_value: i32,
    destination: &mut Vec<Box<InstanceTypeTree<'a>>>,
) -> i32 {
    if root.start < start_value {
        panic!("Failed to assign instance type {} to {}", root.start, root.type_.name());
       // Error("Failed to assign instance type ", root.start, " to ",
       //   root.type.name())
       //.Position(root.type.GetPosition());
    }

    // First, separate the children into four groups:
    // - The one child that must go first, if it exists;
    // - Children with specific value requirements ("constrained");
    // - Children without specific value requirements ("unconstrained");
    // - The one child that must go last, if it exists.
    let mut lowest_child: Option<Box<InstanceTypeTree>> = None;
    let mut highest_child: Option<Box<InstanceTypeTree>> = None;
    let mut constrained_children_by_start: BTreeMap<i32, Box<InstanceTypeTree>> = BTreeMap::new();
    // Using BTreeMap because you can't std::move out of a BTreeSet until C++17.
    let mut unconstrained_children_by_size: BTreeMap<CompareUnconstrainedTypes, Box<InstanceTypeTree>> = BTreeMap::new();

    for child in &mut root.children.drain(..) {
        if child.type_.is_highest_instance_type_within_parent() {
            if highest_child.is_some() {
                panic!("Two classes requested to be the highest instance type: {} and {} within range for parent class {}",
                       highest_child.as_ref().unwrap().type_.name(), child.type_.name(), root.type_.name());
               // Error("Two classes requested to be the highest instance type: ",
               //     highest_child.type.name(), " and ", child.type.name(),
               //     " within range for parent class ", root.type.name())
               // .Position(child.type.GetPosition());
            }
            if child.type_.is_lowest_instance_type_within_parent() {
                panic!("Class requested to be both highest and lowest instance type within its parent range: {}", child.type_.name());
                //Error(
                //    "Class requested to be both highest and lowest instance type "
                //    "within its parent range: ",
                //    child.type.name())
                //.Position(child.type.GetPosition());
            }
            highest_child = Some(child);
        } else if child.type_.is_lowest_instance_type_within_parent() {
            if lowest_child.is_some() {
                panic!("Two classes requested to be the lowest instance type: {} and {} within range for parent class {}",
                       lowest_child.as_ref().unwrap().type_.name(), child.type_.name(), root.type_.name());
                //Error("Two classes requested to be the lowest instance type: ",
                //    lowest_child.type.name(), " and ", child.type.name(),
                //    " within range for parent class ", root.type.name())
                //.Position(child.type.GetPosition());
            }
            lowest_child = Some(child);
        } else if child.start > child.end {
            unconstrained_children_by_size.insert(
                CompareUnconstrainedTypes { a: &child, b: &child },
                child,
            );
        } else {
            constrained_children_by_start.insert(child.start, child);
        }
    }

    let mut own_type_pending = root.num_own_values > 0;

    // Second, iterate and place the children in ascending order.
    if let Some(mut child) = lowest_child {
        start_value = solve_instance_type_constraints(child, start_value, &mut root.children);
    }
    for (constrained_child_start, mut constrained_child) in constrained_children_by_start {
        // Select the next constrained child type in ascending order.

        // Try to place the root type before the constrained child type if it fits.
        if own_type_pending {
            if (root.value != -1 && root.value < constrained_child_start)
                || (root.value == -1
                    && start_value + root.num_own_values <= constrained_child_start)
            {
                let start_value = select_own_values(&mut root, start_value);
                own_type_pending = false;
            }
        }

        // Try to find any unconstrained children that fit before the constrained
        // one. This simple greedy algorithm just puts the biggest unconstrained
        // children in first, which might not fill the space as efficiently as
        // possible but is good enough for our needs.
        let mut to_remove: Vec<CompareUnconstrainedTypes> = Vec::new();
        for (unconstrained_child_pair, _) in unconstrained_children_by_size.iter() {
            if unconstrained_child_pair.a.num_values + start_value <= constrained_child_start {
                to_remove.push(CompareUnconstrainedTypes {a: unconstrained_child_pair.a, b: unconstrained_child_pair.b});
            }
        }
        for key in to_remove {
            let mut child = unconstrained_children_by_size.remove(&key).unwrap();
            start_value = solve_instance_type_constraints(child, start_value, &mut root.children);
        }

        // Place the constrained child type.
        start_value = solve_instance_type_constraints(
            constrained_child,
            start_value,
            &mut root.children,
        );
    }
    if own_type_pending {
        let start_value = select_own_values(&mut root, start_value);
        own_type_pending = false;
    }
    for (child_pair, _) in unconstrained_children_by_size.drain() {
        let mut child = unconstrained_children_by_size.remove(&child_pair).unwrap();
        start_value = solve_instance_type_constraints(child, start_value, &mut root.children);
    }
    if let Some(mut child) = highest_child {
        start_value = solve_instance_type_constraints(child, start_value, &mut root.children);
    }

    // Finally, set the range for this class to include all placed subclasses.
    root.end = start_value - 1;
    root.start = if root.children.is_empty() {
        start_value
    } else {
        root.children.first().unwrap().start
    };
    if root.value != -1 && root.value < root.start {
        root.start = root.value;
    }
    root.num_values = root.end - root.start + 1;
    let value = if root.value == -1 {
        None
    } else {
        Some(root.value)
    };
    root.type_
        .initialize_instance_types(value, (root.start, root.end));

    if root.num_values > 0 {
        destination.push(root);
    }
    start_value
}

fn solve_instance_type_constraints_root<'a>(
    root: Option<Box<InstanceTypeTree<'a>>>,
) -> Option<Box<InstanceTypeTree<'a>>> {
    let mut destination: Vec<Box<InstanceTypeTree>> = Vec::new();
    if let Some(root) = root {
        solve_instance_type_constraints(root, 0, &mut destination);
        if !destination.is_empty() {
            return Some(destination.remove(0));
        }
    }
    None
}

fn assign_instance_types<'a>() -> Option<Box<InstanceTypeTree<'a>>> {
    let root = build_instance_type_tree();
    if let Some(mut root) = root {
        propagate_instance_type_constraints(&mut root);
        solve_instance_type_constraints_root(Some(root))
    } else {
        None
    }
}

// Prints items in macro lists for the given type and its descendants.
// - definitions: This list is pairs of instance type name and assigned value,
//   such as V(ODDBALL_TYPE, 67). It includes FIRST_* and LAST_* items for each
//   type that has more than one associated InstanceType. Items within those
//   ranges are indented for readability.
// - values: This list is just instance type names, like V(ODDBALL_TYPE). It
//   does not include any FIRST_* and LAST_* range markers.
// - fully_defined_single_instance_types: This list is pairs of class name and
//   instance type, for classes which have defined layouts and a single
//   corresponding instance type.
// - fully_defined_multiple_instance_types: This list is pairs of class name and
//   instance type, for classes which have defined layouts and subclasses.
// - only_declared_single_instance_types: This list is pairs of class name and
//   instance type, for classes which have a single corresponding instance type
//   and do not have layout definitions in Torque.
// - only_declared_multiple_instance_types: This list is pairs of class name and
//   instance type, for classes which have subclasses but also have a single
//   corresponding instance type, and do not have layout definitions in Torque.
// - fully_defined_range_instance_types: This list is triples of class name,
//   first instance type, and last instance type, for classes which have defined
//   layouts and multiple corresponding instance types.
// - only_declared_range_instance_types: This list is triples of class name,
//   first instance type, and last instance type, for classes which have
//   multiple corresponding instance types and do not have layout definitions in
//   Torque.
fn print_instance_types(
    root: &InstanceTypeTree,
    definitions: &mut String,
    values: &mut String,
    fully_defined_single_instance_types: &mut String,
    fully_defined_multiple_instance_types: &mut String,
    only_declared_single_instance_types: &mut String,
    only_declared_multiple_instance_types: &mut String,
    fully_defined_range_instance_types: &mut String,
    only_declared_range_instance_types: &mut String,
    indent: &str,
) {
    let type_name = capify_string_with_underscores(root.type_.name()) + "_TYPE";
    let mut inner_indent = indent.to_string();

    if root.num_values > 1 {
        *definitions += &format!("{}V(FIRST_{}, {}) \\\\\n", indent, type_name, root.start);
        inner_indent += "  ";
    }
    if root.num_own_values == 1 {
        *definitions += &format!("{}V({}, {}) /* {} */\\\\\n", inner_indent, type_name, root.value, root.type_.get_position());
        *values += &format!("  V({}) /* {} */\\\\\n", type_name, root.type_.get_position());
        let type_checker_list = if root.type_.has_undefined_layout() {
            if root.num_values == 1 {
                only_declared_single_instance_types
            } else {
                only_declared_multiple_instance_types
            }
        } else {
            if root.num_values == 1 {
                fully_defined_single_instance_types
            } else {
                fully_defined_multiple_instance_types
            }
        };
        *type_checker_list += &format!(
            "  V({}, {}) /* {} */ \\\\\n",
            root.type_.name(),
            type_name,
            root.type_.get_position()
        );
    }
    for child in &root.children {
        print_instance_types(
            child,
            definitions,
            values,
            fully_defined_single_instance_types,
            fully_defined_multiple_instance_types,
            only_declared_single_instance_types,
            only_declared_multiple_instance_types,
            fully_defined_range_instance_types,
            only_declared_range_instance_types,
            &inner_indent,
        );
    }
    if root.num_values > 1 {
        // We can't emit LAST_STRING_TYPE because it's not a valid flags
        // combination. So if the class type has multiple own values, which only
        // happens when using ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE, then omit
        // the end marker.
        if root.num_own_values <= 1 {
            *definitions += &format!("{}V(LAST_{}, {}) \\\\\n", indent, type_name, root.end);
        }

        // Only output the instance type range for things other than the root type.
        if root.type_.get_super_class().is_some() {
            let range_instance_types = if root.type_.has_undefined_layout() {
                only_declared_range_instance_types
            } else {
                fully_defined_range_instance_types
            };
            *range_instance_types += &format!(
                "  V({}, FIRST_{}, LAST_{}) \\\\\n",
                root.type_.name(),
                type_name,
                type_name
            );
        }
    }
}

fn capify_string_with_underscores(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            result.push('_');
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }
    result
}

fn snakeify_string(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

impl ImplementationVisitor {
    pub fn generate_instance_types(&mut self, output_directory: &str) -> Result<(), io::Error> {
        let mut header = String::new();
        let file_name = "instance-types.h";
        {
            let mut guard = IncludeGuardScope::new(&mut header, file_name);

            header.push_str(
                "// Instance types for all classes except for those that use \
                 InstanceType as flags.\n",
            );
            header.push_str("#define TORQUE_ASSIGNED_INSTANCE_TYPES(V) \\\\\n");
            let instance_types = assign_instance_types();
            let mut values_list = String::new();
            let mut fully_defined_single_instance_types = String::new();
            let mut fully_defined_multiple_instance_types = String::new();
            let mut only_declared_single_instance_types = String::new();
            let mut only_declared_multiple_instance_types = String::new();
            let mut fully_defined_range_instance_types = String::new();
            let mut only_declared_range_instance_types = String::new();
            if let Some(instance_types) = &instance_types {
                print_instance_types(
                    instance_types,
                    &mut header,
                    &mut values_list,
                    &mut fully_defined_single_instance_types,
                    &mut fully_defined_multiple_instance_types,
                    &mut only_declared_single_instance_types,
                    &mut only_declared_multiple_instance_types,
                    &mut fully_defined_range_instance_types,
                    &mut only_declared_range_instance_types,
                    "  ",
                );
            }
            header.push_str("\n");

            header.push_str("// Instance types for all classes except for those that use\n");
            header.push_str("// InstanceType as flags.\n");
            header.push_str("#define TORQUE_ASSIGNED_INSTANCE_TYPE_LIST(V) \\\\\n");
            header.push_str(&values_list);
            header.push_str("\n");

            header.push_str("// Pairs of (ClassName, INSTANCE_TYPE) for classes that have\n");
            header.push_str("// full Torque definitions.\n");
            header.push_str("#define TORQUE_INSTANCE_CHECKERS_SINGLE_FULLY_DEFINED(V) \\\\\n");
            header.push_str(&fully_defined_single_instance_types);
            header.push_str("\n");

            header.push_str("// Pairs of (ClassName, INSTANCE_TYPE) for classes that have\n");
            header.push_str("// full Torque definitions and subclasses.\n");
            header.push_str("#define TORQUE_INSTANCE_CHECKERS_MULTIPLE_FULLY_DEFINED(V) \\\\\n");
            header.push_str(&fully_defined_multiple_instance_types);
            header.push_str("\n");

            header.push_str("// Pairs of (ClassName, INSTANCE_TYPE) for classes that are\n");
            header.push_str("// declared but not defined in Torque. These classes may\n");
            header.push_str("// correspond with actual C++ classes, but they are not\n");
            header.push_str("// guaranteed to.\n");
            header.push_str("#define TORQUE_INSTANCE_CHECKERS_SINGLE_ONLY_DECLARED(V) \\\\\n");
            header.push_str(&only_declared_single_instance_types);
            header.push_str("\n");

            header.push_str("// Pairs of (ClassName, INSTANCE_TYPE) for classes that are\n");
            header.push_str("// declared but not defined in Torque, and have subclasses.\n");
            header.push_str("// These classes may correspond with actual C++ classes, but\n");
            header.push_str("// they are not guaranteed to.\n");
            header.push_str("#define TORQUE_INSTANCE_CHECKERS_MULTIPLE_ONLY_DECLARED(V) \\\\\n");
            header.push_str(&only_declared_multiple_instance_types);
            header.push_str("\n");

            header.push_str("// Triples of (ClassName, FIRST_TYPE, LAST_TYPE) for classes\n");
            header.push_str("// that have full Torque definitions.\n");
            header.push_str("#define TORQUE_INSTANCE_CHECKERS_RANGE_FULLY_DEFINED(V) \\\\\n");
            header.push_str(&fully_defined_range_instance_types);
            header.push_str("\n");

            header.push_str("// Triples of (ClassName, FIRST_TYPE, LAST_TYPE) for classes\n");
            header.push_str("// that are declared but not defined in Torque. These classes\n");
            header.push_str("// may correspond with actual C++ classes, but they are not\n");
            header.push_str("// guaranteed to.\n");
            header.push_str("#define TORQUE_INSTANCE_CHECKERS_RANGE_ONLY_DECLARED(V) \\\\\n");
            header.push_str(&only_declared_range_instance_types);
            header.push_str("\n");

            let mut torque_defined_class_list = String::new();
            let mut torque_defined_varsize_instance_type_list = String::new();
            let mut torque_defined_fixed_instance_type_list = String::new();
            let mut torque_defined_map_csa_list = String::new();
            let mut torque_defined_map_root_list = String::new();

            for type_ in TypeOracle::get_classes() {
                let upper_case_name = type_.name().to_string();
                let lower_case_name = snakeify_string(type_.name());
                let instance_type_name = capify_string_with_underscores(type_.name()) + "_TYPE";

                if !type_.is_extern() {
                    torque_defined_class_list.push_str(&format!("  V({}) \\\\\n", upper_case_name));
                }

                if type_.should_generate_unique_map() {
                    torque_defined_map_csa_list.push_str(&format!(
                        "  V(_, {}Map, {}, {}) \\\\\n",
                        upper_case_name, lower_case_name, upper_case_name
                    ));
                    torque_defined_map_root_list.push_str(&format!(
                        "  V(Map, {}_map, {}Map) \\\\\n",
                        lower_case_name, upper_case_name
                    ));
                    let list = if type_.has_static_size() {
                        &mut torque_defined_fixed_instance_type_list
                    } else {
                        &mut torque_defined_varsize_instance_type_list
                    };
                    list.push_str(&format!(
                        "  V({}, {}, {}) \\\\\n",
                        instance_type_name, upper_case_name, lower_case_name
                    ));
                }
            }

            header.push_str("// Fully Torque-defined classes (both internal and exported).\n");
            header.push_str("#define TORQUE_DEFINED_CLASS_LIST(V) \\\\\n");
            header.push_str(&torque_defined_class_list);
            header.push_str("\n");
            header.push_str("#define TORQUE_DEFINED_VARSIZE_INSTANCE_TYPE_LIST(V) \\\\\n");
            header.push_str(&torque_defined_varsize_instance_type_list);
            header.push_str("\n");
            header.push_str("#define TORQUE_DEFINED_FIXED_INSTANCE_TYPE_LIST(V) \\\\\n");
            header.push_str(&torque_defined_fixed_instance_type_list);
            header.push_str("\n");
            header.push_str("#define TORQUE_DEFINED_INSTANCE_TYPE_LIST(V) \\\\\n");
            header.push_str("  TORQUE_DEFINED_VARSIZE_INSTANCE_TYPE_LIST(V) \\\\\n");
            header.push_str("  TORQUE_DEFINED_FIXED_INSTANCE_TYPE_LIST(V) \\\\\n");
            header.push_str("\n");
            header.push_str("#define TORQUE_DEFINED_MAP_CSA_LIST_GENERATOR(V, _) \\\\\n");
            header.push_str(&torque_defined_map_csa_list);
            header.push_str("\n");
            header.push_str("#define TORQUE_DEFINED_MAP_ROOT_LIST(V) \\\\\n");
            header.push_str(&torque_defined_map_root_list);
            header.push_str("\n");
        }
        let output_header_path = Path::new(output_directory).join(file_name);
        write_file(&output_header_path, &header)?;

        GlobalContext::set_instance_types_initialized();
        Ok(())
    }
}

struct IncludeGuardScope<'a> {
    header: &'a mut String,
    file_name: &'a str,
    guard_name: String,
}

impl<'a> IncludeGuardScope<'a> {
    fn new(header: &'a mut String, file_name: &'a str) -> Self {
        let guard_name = file_name.replace(".", "_").to_uppercase();
        header.push_str(&format!("#ifndef {}\n", guard_name));
        header.push_str(&format!("#define {}\n\n", guard_name));

        IncludeGuardScope {
            header,
            file_name,
            guard_name,
        }
    }
}

impl<'a> Drop for IncludeGuardScope<'a> {
    fn drop(&mut self) {
        self.header.push_str(&format!("#endif  // {}\n", self.guard_name));
    }
}

fn write_file(path: &Path, content: &str) -> Result<(), io::Error> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}