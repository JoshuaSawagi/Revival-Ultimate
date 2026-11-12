use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::phx::*;
use smash::app::sv_animcmd::*;
use smash::app::smashball::*;
use smash_script::*;
use globals::*;
use crate::util::get_fighter_common_from_accessor;
use smash::cpp::root::app::SituationKind;
use smash::hash40;
use smash::phx::Hash40;
use vars::*;
use crate::StatusModule::status_kind;
use crate::app::utility::get_kind;
use crate::StatusModule::situation_kind;
use super::*;

// Shifts Run, RunBrake, TurnRun, and TurnRunBrake animations to match any horizontal hip bone adjustment to vanilla dash animations
// otherwise the animations don't transition properly into one another
// This is so we don't have to edit those 4 other animations if we want to edit a dash anim
unsafe fn custom_dash_anim_support(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_RUN) && fighter.is_motion(Hash40::new("run")) {
        let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
        let run_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X);
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    }
    else if fighter.is_prev_status(*FIGHTER_STATUS_KIND_RUN)
    && StatusModule::is_changing(fighter.module_accessor)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_TURN_RUN) {
        ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("hip"));
    }
    
    if fighter.is_status(*FIGHTER_STATUS_KIND_TURN_RUN) && fighter.is_motion(Hash40::new("turn_run")) {
        let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
        let run_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X);
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    }
    else if fighter.is_prev_status(*FIGHTER_STATUS_KIND_TURN_RUN)
    && StatusModule::is_changing(fighter.module_accessor)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_RUN) {
        ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("hip"));
    }
}

pub extern "C" fn left_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.left_stick_x().signum() != fighter.prev_left_stick_x().signum() || fighter.prev_left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X);
        }
        
        if fighter.left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.left_stick_y().signum() != fighter.prev_left_stick_y().signum()
        || fighter.prev_left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y);
        }
    }
}

pub extern "C" fn right_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.right_stick_x().signum() != fighter.prev_right_stick_x().signum() || fighter.prev_right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X);
        }
        
        if fighter.right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.right_stick_y().signum() != fighter.prev_right_stick_y().signum() || fighter.prev_right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y);
        }
    }
}
pub unsafe fn cliff_xlu_frame_counter(fighter: &mut L2CFighterCommon) {
    let cliff_xlu = VarModule::get_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
    // If you have ledge intan frames left
    if cliff_xlu > 0 {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            // Remove ledge intan on landing
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
            VarModule::set_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME, 0);
        }
        else{
            VarModule::dec_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
        }
    }
}

pub unsafe extern "C" fn held_buffer(fighter : &mut L2CFighterCommon) {
    let module_accessor = &mut *fighter.module_accessor;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = get_kind(module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
    let cat3 = ControlModule::get_command_flag_cat(fighter.module_accessor, 2);
    let stick_value_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_value_x = ControlModule::get_stick_x(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let curr_frame = MotionModule::frame(fighter.module_accessor);
    let kinetic_type = KineticModule::get_kinetic_type(fighter.module_accessor);
    let control_pad = [
        *CONTROL_PAD_BUTTON_APPEAL_HI, *CONTROL_PAD_BUTTON_APPEAL_LW, *CONTROL_PAD_BUTTON_APPEAL_S_L, *CONTROL_PAD_BUTTON_APPEAL_S_R, *CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_ATTACK_RAW, *CONTROL_PAD_BUTTON_CATCH, *CONTROL_PAD_BUTTON_CSTICK_ON,
        *CONTROL_PAD_BUTTON_FLICK_JUMP, *CONTROL_PAD_BUTTON_GUARD, *CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_INVALID, *CONTROL_PAD_BUTTON_JUMP, *CONTROL_PAD_BUTTON_JUMP_MINI, *CONTROL_PAD_BUTTON_SMASH, *CONTROL_PAD_BUTTON_SPECIAL, 
        *CONTROL_PAD_BUTTON_SPECIAL_RAW, *CONTROL_PAD_BUTTON_SPECIAL_RAW2, *CONTROL_PAD_BUTTON_STOCK_SHARE, *CONTROL_PAD_BUTTON_TERM, *CONTROL_PAD_CLATTER_CAUSE_NONE, *CONTROL_PAD_CLATTER_FLOWER, *CONTROL_PAD_CLATTER_MAIN, *CONTROL_PAD_CLATTER_NONE,
        *CONTROL_PAD_CLATTER_TERM, *CONTROL_PAD_STICK_REVERSE_ALL, *CONTROL_PAD_STICK_REVERSE_NONE, *CONTROL_PAD_STICK_REVERSE_X, *CONTROL_PAD_STICK_REVERSE_Y
    ];
    for i in control_pad {
        if ControlModule::get_trigger_count(module_accessor, i as u8) > 15 && ControlModule::check_button_on(module_accessor, i)
        && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
            ControlModule::reset_trigger(module_accessor);
            ControlModule::clear_command(module_accessor, true);
        }
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    left_stick_flick_counter(fighter);
    right_stick_flick_counter(fighter);
    cliff_xlu_frame_counter(fighter);
    custom_dash_anim_support(fighter);
    //held_buffer(fighter);
}
