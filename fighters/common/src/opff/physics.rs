use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;
use vars::*;
use crate::sv_kinetic_energy;
use crate::globals::STICK_X;
use smash::app;
use super::*;
use crate::globals::*;

//static mut ECB_Y_OFFSETS: [f32; 9] = [0.0; 9];
pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

unsafe fn waveland_check(boma: &mut BattleObjectModuleAccessor) {
    if !boma.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR)
    || !boma.is_situation(*SITUATION_KIND_AIR) {
        return;
    }
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    if speed_y >= 0.0 { return; }

    let pos = Vector2f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) };
    let ray = Vector2f { x: speed_x, y: speed_y };
    let mut hit_pos = Vector2f { x: 0.0, y: 0.0 };

    if GroundModule::ray_check_hit_pos(boma, &pos, &ray, &mut hit_pos, true) {
        PostureModule::set_pos_2d(boma, &hit_pos);
        StatusModule::set_situation_kind(boma, app::SituationKind(*SITUATION_KIND_GROUND), true);
        GroundModule::attach_ground(boma, false);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

unsafe fn double_shield_button_airdodge(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    // airdodge with second shield button while holding another shield button
    if boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_button_trigger(Buttons::GuardHold)
    && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_PASS,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3
    ])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        return;
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {
    waveland_check(boma);
    double_shield_button_airdodge(boma, status_kind, situation_kind, cat[0]);
}