mod path_node;

pub use path_node::PathNode;

// /// Legend:
// /// @: Dwarf
// /// X: Supporting block
// /// O: Non-supporting block
// /// .: Air
// pub enum DwarfMoveCommand {
//     /// Standing -> Standing
//     /// @.    .@
//     /// XX -> XX
//     Walk(I3),
//     /// Standing -> Climbing
//     /// @O    @X
//     /// X  -> O
//     StartClimb(AbsoluteFacingXZ),
//     /// Climbing -> Standing
//     /// ..    @.    .@
//     /// @X -> .X -> .X
//     EndClimbUp,
//     /// Climbing -> Standing
//     /// @X    @O
//     /// O  -> X
//     EndClimbDown,
//     /// Climbing -> Falling
//     DropFromClimb,
//     /// From above:
//     /// @O    @X
//     /// X  -> O
//     RotateClimb(AbsoluteFacingXZ),
//     /// @..    .@.
//     /// X   -> X
//     JumpForward,
//     /// @.    .@
//     /// X. ->  X
//     JumpDown,
//     /// Accelerated by gravity, otherwise constant speed
//     Fall { velocity: Vec3 },
//     /// @.    .@
//     ///  X ->  X
//     LandFall,
// }
