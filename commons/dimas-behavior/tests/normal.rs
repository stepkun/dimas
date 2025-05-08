// Copyright © 2025 Stephan Kunz

//! Tests

use dimas_behavior::{
	blackboard::{Blackboard, BlackboardData, SharedBlackboard},
	port::{PortDefinition, PortList, PortRemappings},
	tree::{BehaviorTree, BehaviorTreeComponentList, BehaviorTreeLeaf, BehaviorTreeNode},
};

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<Blackboard>();
	is_normal::<BlackboardData>();
	is_normal::<SharedBlackboard>();

	is_normal::<BehaviorTree>();
	is_normal::<BehaviorTreeComponentList>();
	is_normal::<BehaviorTreeLeaf>();
	is_normal::<BehaviorTreeNode>();

	is_normal::<PortDefinition>();
	is_normal::<PortList>();
	is_normal::<PortRemappings>();
}
