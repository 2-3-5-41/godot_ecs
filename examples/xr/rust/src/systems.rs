use bevy_ecs::{
    query::With,
    system::{Commands, Query, Res},
};
use godot::prelude::{godot_error, godot_print};
use godot_ecs::xr::{server::XrServer, tracker::XrTracker};

use crate::components::{LeftController, RightController};

pub fn init_xr(xr: Res<XrServer>) {
    let Some(interface) = xr.find_interface("OpenXR") else {
        godot_error!("Failed to find \"OpenXR\" interface!");
        return;
    };

    if interface.initialize() {
        godot_print!("OpenXR interface initialized!");
    }
}

pub fn create_ecs_controllers(mut commands: Commands, xr: Res<XrServer>) {
    {
        if let Some(left_hand) = xr.get_tracker("left_hand") {
            commands.spawn((left_hand, LeftController))
        } else {
            return;
        };
    }
    {    
        if let Some(right_hand) = xr.get_tracker("right_hand") {
            commands.spawn((right_hand, RightController))
        } else {
            return;
        };
    }
}

pub fn read_ecs_controller_positions(
    left_hand: Query<&XrTracker, With<LeftController>>,
    right_hand: Query<&XrTracker, With<RightController>>,
) {
    // NOTE: To access `XRPose` data, you'll want to get the pose object every frame
    // in order to get up-to-date data per frame, otherwise, you'll be stuck with old data.

    left_hand.iter().for_each(|tracker| {
        if let Some(pose) = tracker.get_pose("aim") {
            godot_print!("left hand pose: {:?}", pose.get_adjusted_transform());
        };
    });

    right_hand.iter().for_each(|tracker| {
        if let Some(pose) = tracker.get_pose("aim") {
            godot_print!("right hand pose: {:?}", pose.get_adjusted_transform());
        };
    });
}
