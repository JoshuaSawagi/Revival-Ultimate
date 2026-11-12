use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_Jump_Main,
            //status_pre_Jump_sub_param,
        );
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_Jump_sub_param)]
unsafe extern "C" fn status_pre_Jump_sub_param(fighter: &mut L2CFighterCommon, flag_keep: L2CValue, int_keep: L2CValue, float_keep: L2CValue, kinetic_type: L2CValue, arg: L2CValue) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    let flag_keep = flag_keep.get_i32();
    let int_keep = int_keep.get_i32();
    let float_keep = float_keep.get_i32();
    let mut kinetic_type = kinetic_type.get_i32();
    let arg = arg.get_i32();
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let ground_correct_kind = app::FighterUtil::get_ground_correct_kind_air_trans(fighter.module_accessor, status_kind);
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        kinetic_type,
        ground_correct_kind as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        flag_keep,
        int_keep,
        float_keep,
        arg
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        true,
        0,
        (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32,
        0,
        0
    );
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_Main)]
unsafe extern "C" fn status_Jump_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
    VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM_SPECIALS, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
    let ret = if fighter.sub_transition_group_check_air_cliff().get_bool() {
        1.into()
    }
    else if fighter.sub_air_check_fall_common().get_bool() {
        1.into()
    }
    else if fighter.sub_air_check_stop_ceil().get_bool() {
        1.into()
    } else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
            L2CValue::Bool(false)
        );
        1.into()
    } else {
        fighter.sub_air_check_superleaf_fall_slowly();
        0.into()
    };
    
    ret
}