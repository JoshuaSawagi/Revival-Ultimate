/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;
use globals::*;

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jumpsquat_main,
            status_jumpsquat_common,
            status_jumpsquat,
            status_end_jumpsquat,
            uniq_process_JumpSquat_exec_status,
            uniq_process_JumpSquat_exec_status_param,
            sub_status_jumpsquat_check_stick_lr_update,
            sub_jump_squat_uniq_check_sub,
            sub_jump_squat_uniq_check_sub_mini_attack,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe fn status_jumpsquat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let should_end = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter).get_bool()
        } else {
            false
        }
    } else { false };
    if should_end {
        return L2CValue::I32(1);
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_JUMP),
            L2CValue::Bool(false)
        );
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
    && fighter.is_situation(*SITUATION_KIND_AIR)  {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
            L2CValue::Bool(false)
        );
        return 0.into();
    }

    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && ItemModule::is_have_item(fighter.module_accessor, 0) {
                            //(boma, x_speed, y_speed, hand_index)
        ItemModule::drop_item(fighter.module_accessor, 0.0, 0.0, 0);
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_CATCH),
            L2CValue::Bool(true)
        );
        return 0.into();
    }

    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
            && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
            && fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_SPECIAL_HI),
                L2CValue::Bool(true)
            );
        } else if !fighter.sub_transition_specialflag_hoist().get_bool() {
            // let cat2 = fighter.global_table[CMD_CAT2].get_i32();
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[0x58].get_bool() != false && {
                    let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x58].get_ptr());
                    callable(fighter).get_bool()
                } {
                    return L2CValue::I32(0);
                }
                if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
                    && fighter.is_situation(*SITUATION_KIND_GROUND) {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_HI4_START),
                        L2CValue::Bool(true)
                    );
                }
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_common)]
unsafe extern "C" fn status_jumpsquat_common(fighter: &mut L2CFighterCommon, lr_update: L2CValue) {
    let is_button_jump = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE) == 0
                        || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0;
    if is_button_jump {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_BUTTON);
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if lr_update.get_bool() {
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    let potential_enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND
    ];
    for x in potential_enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ABNORMAL_MINIJUMP_SLOWWALK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
    }
    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF]) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    }
}

#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status)]
unsafe fn uniq_process_JumpSquat_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    uniq_process_JumpSquat_exec_status_param(fighter, L2CValue::Ptr(0 as _));
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status_param)]
unsafe extern "C" fn uniq_process_JumpSquat_exec_status_param(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    let should_check = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter);
            true
        } else {
            true
        }
    } else { true };
    if should_check {
        fighter.sub_jump_squat_uniq_check_sub(L2CValue::I32(*FIGHTER_STATUS_JUMP_FLAG_BUTTON));
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }

    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(mot));
    if frame >= end_frame {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        let situation_kind = fighter.global_table[SITUATION_KIND].clone();
        fighter.global_table[PREV_SITUATION_KIND].assign(&situation_kind);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_SQUAT, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub)]
unsafe fn sub_jump_squat_uniq_check_sub(fighter: &mut L2CFighterCommon, flag: L2CValue) {

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    if WorkModule::is_flag(fighter.module_accessor, flag.get_i32()){
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(mot));
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            || ControlModule::is_jump_mini_button(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    } else {
        if fighter.left_stick_y() < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y")) {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
            && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat)]
unsafe fn status_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr_update = fighter.sub_status_JumpSquat_check_stick_lr_update();
    fighter.status_JumpSquat_common(lr_update);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_jumpsquat_main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_JumpSquat_check_stick_lr_update)]
unsafe fn sub_status_jumpsquat_check_stick_lr_update(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();

    L2CValue::Bool(prev_status == *FIGHTER_STATUS_KIND_DASH && fighter.is_button_on(Buttons::CStickOverride))
}

#[skyline::hook(replace = L2CFighterCommon_status_end_JumpSquat)]
unsafe fn status_end_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    InputModule::disable_persist(fighter.battle_object);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE_SECOND);
    VarModule::set_int(fighter.battle_object, vars::common::instance::JUMP_SQUAT_FRAME, 0);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_TAP_JUMP);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_ATTACK_CANCEL);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub_mini_attack)]
unsafe fn sub_jump_squat_uniq_check_sub_mini_attack(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON)
            && current_frame > 0.0 {
            FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        }
    } else {
        if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
                return;
            }
        } else if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
            return;
        }
        let unables = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW
        ];
        for x in unables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
}