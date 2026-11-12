use super::*;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use std::sync::atomic::Ordering;
use crate::globals::STATUS_KIND;
use smash_rs::app::camera::get_dead_area;
use other::*;
use momentum_holder::*;
pub mod cancels;
pub mod other;
pub mod physics;
pub mod momentum_transfer_line;
pub mod momentum_holder;
//pub mod reset;
use crate::StatusModule::status_kind;

#[utils::export(common::opff)]
pub unsafe fn fighter_common_opff(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        let boma = &mut *info.boma;
        if boma.is_fighter() {
            moveset_edits(fighter, &info);
        }
    } else {
        panic!("Could not get the FrameInfo for this fighter! Is this even a fighter?")
    }
}

use utils::util::MATCH_EXITING;
static mut IS_SALTY_INPUT: bool = false;

unsafe fn salty_check(fighter: &mut L2CFighterCommon) -> bool {
    if IS_SALTY_INPUT {
        return false;
    }
    if fighter.is_button_on(Buttons::StockShare) {
        if fighter.is_button_on(Buttons::AttackRaw) && !fighter.is_button_on(!(Buttons::AttackRaw | Buttons::StockShare)) {
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_assist_out"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false);
            utils::util::trigger_match_reset();
            utils::game_modes::signal_new_game();
            true
        } else if fighter.is_button_on(Buttons::SpecialRaw) && !fighter.is_button_on(!(Buttons::SpecialRaw | Buttons::StockShare)) {
            MATCH_EXITING.store(true, Ordering::Relaxed);
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY]) {
                StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD, false);
            }
            utils::util::trigger_match_exit();
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub const PIVOT_STICK_SNAPBACK_WINDOW: f32 = 2.0;

pub unsafe fn moveset_edits(fighter: &mut L2CFighterCommon, info: &FrameInfo) {
    let boma = &mut *info.boma;
    
    // allow ledge regrab iframes
    if WorkModule::get_int(fighter.boma(), *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 20 {
        // indicate that your next ledge grab grab is capable of having iframes
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
    }

    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY])
    || !sv_information::is_ready_go()
    {
        IS_SALTY_INPUT = false;
    }

    // General Engine Edits
    if salty_check(fighter) {
        IS_SALTY_INPUT = true;
        return;
    }
    
    /*Shield Drop*/
    if GroundModule::is_passable_ground(fighter.module_accessor)
        && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) <= 6
        && fighter.left_stick_y() < fighter.get_param_float("common", "pass_stick_y")
        && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_RUN,
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_GUARD_OFF
        ]) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
    }

    /*Dash to Shield-Stops*/
    let status_kind = StatusModule::status_kind(boma);
    let cat1 = ControlModule::get_command_flag_cat(boma, 0);
    if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        }
    }
    
    /*Pivots*/
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let mut pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: dash_speed * 0.75, y: 0.0, z: 0.0};
    let current_frame = MotionModule::frame(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        && current_frame <= PIVOT_STICK_SNAPBACK_WINDOW && stick_x == 0.0
        && [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&StatusModule::prev_status_kind(boma, 0))
        && ![*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(boma, 1))
    {
        PostureModule::reverse_lr(boma);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN,true);
        KineticModule::clear_speed_all(boma);
        KineticModule::add_speed(boma, &pivot_boost);
    }



    cancels::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    momentum_holder::run(fighter);
    momentum_transfer_line::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    other::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    physics::run(fighter, boma, info.cat, info.situation_kind, info.fighter_kind, info.status_kind, info.stick_y, info.facing, info.stick_x, info.frame);
    //reset::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_sub)]
pub unsafe fn status_jump_sub_hook(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);

    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(calcMomentum(boma)));
    smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);


    original!()(fighter, param_2, param_3)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let is_speed_backward = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) < 0.0;
    let prev_status_check = [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(boma, 0));    
    let mut new_speed = CURRENTMOMENTUM[get_player_number(boma)];


        /*      Shorthop aerial macro and "bair stick flick" fix     */
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 && 
        StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT && !is_speed_backward { //if you used the shorthop aerial macro
        new_speed = calcMomentum(boma);
    }

    if prev_status_check {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(new_speed));
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }

    original!()(fighter, param_1)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
pub unsafe fn sys_line_system_control_fighter_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    left_stick_flick_counter(fighter);
    right_stick_flick_counter(fighter);
    //held_buffer(fighter);
    original!()(fighter)
    
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sys_line_system_control_fighter_hook,
            status_jump_sub_hook,
            status_attack_air_hook
        );
    }
}

pub fn install() {
    Agent::new("fighter")
        .install();
    skyline::nro::add_hook(nro_hook);
}