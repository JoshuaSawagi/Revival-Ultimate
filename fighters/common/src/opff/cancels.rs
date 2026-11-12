use utils::{
    *,
    ext::*,
    consts::*
};

use utils::consts::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use super::*;
use crate::globals::*;

pub unsafe fn run(fighter: &mut L2CFighterCommon ,boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    //jump_cancel_grab(fighter);
}