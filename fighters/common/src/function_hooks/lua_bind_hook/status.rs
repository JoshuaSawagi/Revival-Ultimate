use super::*;
use globals::*;

//=================================================================
//== StatusModule::init_settings
//=================================================================
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, mut situation: smash::app::SituationKind, kinetic_type: i32, ground_correct_kind: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool, keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let mut cliff_check_kind = ground_cliff_check_kind;
    let mut kinetic_type = kinetic_type.clone();
    let mut ground_correct_kind = ground_correct_kind.clone();
    ground_correct_kind = super::ground::init_settings_edges(boma, situation, kinetic_type, ground_correct_kind, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);

    if app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if !(app::utility::get_kind(boma) == *FIGHTER_KIND_DOLLY && [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL].contains(&StatusModule::status_kind(boma))) {
            JostleModule::set_team(boma, 0);
        }

    }
    if VarModule::has_var_module(boma.object()) {
        let mut mask = 0;
        if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG {
            mask += VarModule::RESET_STATUS_FLAG;
        }
        if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT {
            mask += VarModule::RESET_STATUS_INT;
            mask += VarModule::RESET_STATUS_INT64;
        }
        if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT {
            mask += VarModule::RESET_STATUS_FLOAT;
        }
        VarModule::reset(boma.object(), mask);
    }
    original!()(boma, situation, kinetic_type, ground_correct_kind, cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;
    
    if boma.is_fighter() {  
        if [
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
        ].contains(&next_status) {
            clear_buffer = true;
        }
    }
    original!()(boma, next_status, clear_buffer)
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_hook,
        change_status_request_from_script_hook
    );
}