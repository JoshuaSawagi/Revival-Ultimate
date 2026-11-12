use super::*;
use globals::*;
//=================================================================
//== init_settings for edge_slipoffs module
//== Note: This is called from init_settings::init_settings_hook
//== Note: Forces GroundModule::correct to be called for
//         certain statuses
//== Note: JostleModule::set_team(boma, 0) is for walking through
//         other fighters
//=================================================================
pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
                              ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool,
                              arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u32 {
    /* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);

    if boma.is_fighter()
    && boma.is_situation(*SITUATION_KIND_GROUND) {

        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            fix = *GROUND_CORRECT_KIND_GROUND as u32; /* GROUND_CORRECT_KIND_GROUND is i32 */
        }

        if [*FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_TURN,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT_F,
            *FIGHTER_STATUS_KIND_SQUAT_B,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            // *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            // *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
            *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
            *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE].contains(&status_kind) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }

        if    (fighter_kind == *FIGHTER_KIND_EDGE && [*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind) && StatusModule::prev_status_kind(boma, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
           || (fighter_kind == *FIGHTER_KIND_KAMUI && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_MIIFIGHTER && [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_LANDING,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_LANDING,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_SZEROSUIT && [*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_LANDING,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_DOLLY && [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK,
                                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING,
                                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING,
                                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind)) 
           || (boma.kind() == *FIGHTER_KIND_KOOPAJR && boma.is_status(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_SHEIK && [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK,
                                                       *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
           || (boma.kind() == *FIGHTER_KIND_LUIGI && boma.is_status(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL))
           || (boma.kind() == *FIGHTER_KIND_PIT && boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_LANDING))
           || (boma.kind() == *FIGHTER_KIND_PITB && boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_LANDING))
           || (boma.kind() == *FIGHTER_KIND_SIMON && boma.is_status(*FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING))
           || (boma.kind() == *FIGHTER_KIND_RICHTER && boma.is_status(*FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING))
           || (boma.kind() == *FIGHTER_KIND_DEMON && boma.is_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_LANDING))
           || (boma.kind() == *FIGHTER_KIND_RYU && boma.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_KEN && boma.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_PACKUN && boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_KROOL && boma.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_PIKMIN && boma.is_status(*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_FALCO && boma.is_status(*FIGHTER_FALCO_STATUS_KIND_SPECIAL_S_FALL_LANDING))
           || (boma.kind() == *FIGHTER_KIND_MURABITO && boma.is_status(*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_NESS && boma.is_status(*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END))
           || (boma.kind() == *FIGHTER_KIND_LUCAS && boma.is_status(*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END))
        {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }
    }
    return fix
}

#[skyline::hook(replace = GroundModule::correct)]
pub unsafe fn correct_hook(boma: &mut app::BattleObjectModuleAccessor, param_2: u64) -> u64{

    if boma.is_fighter() {
        let status_kind = StatusModule::status_kind(boma);
        if [/*Air Dodge*/ 0x22, /*Landing*/0x16, /*Run brake*/0x5, /*Turn Dash*/0x7, /*Dash*/0x3].contains(&status_kind) {
            return original!()(boma, *GROUND_CORRECT_KIND_GROUND as u64);
        }
    }
    original!()(boma, param_2)
}

pub fn install() {
    skyline::install_hooks!(
        correct_hook
    );
}

