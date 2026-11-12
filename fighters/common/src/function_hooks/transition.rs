use super::*;
use globals::*;
use smash::app::FighterUtil::get_ground_correct_kind_air_trans;

//=================================================================
//== WorkModule::is_enable_transition_term
//== Note: Disable transition terms
//==        - Airdodge out of tumble
//==        - Airdodge out of footstool during footstool lockout
//=================================================================
static mut DISABLE_TRANS_TERMS: [bool; 8] = [false;8];

#[skyline::hook(replace=WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {

    DISABLE_TRANS_TERMS[get_player_number(boma)] = StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR &&
        (flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON || flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH || 
        flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE || flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F || flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);

    if DISABLE_TRANS_TERMS[get_player_number(boma)] && MotionModule::frame(boma) / MotionModule::rate(boma) as f32 <= 5.0 {
        return false;
    }  

    // Fighters
    if boma.is_fighter() {
        let fighter_kind = boma.kind();
        let status_kind = StatusModule::status_kind(boma);
        let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN {
            if ([*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && MotionModule::frame(boma) < ((MotionModule::end_frame(boma) * 0.5645).ln()) * 9.2157) {
                return false;
            }
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT {
            if ([*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && MotionModule::frame(boma) < ((MotionModule::end_frame(boma) * 0.5645).ln()) * 9.2157) {
                return false;
            }
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && MotionModule::frame(boma) < ((MotionModule::end_frame(boma) * 0.5645).ln()) * 9.2157 {
            return false;
        }

        // Allow dash, run, run_brake => taunt
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW].contains(&flag)
            && [*FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status_kind) {
            return true;
        }

        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
            return false;
        }

        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
            return false;
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S && (VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) || VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT)) {
            return false;
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND && (VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) || VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT)) {
            return false;
        }

        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO && VarModule::is_flag(boma.object(), vars::common::instance::DISABLE_AIR_LASSO) {
            return false;
        }

        if fighter_kind == *FIGHTER_KIND_NANA {
            if ([*FIGHTER_STATUS_KIND_WAIT, 
                *FIGHTER_STATUS_KIND_TURN, 
                *FIGHTER_STATUS_KIND_WALK, 
                *FIGHTER_STATUS_KIND_WALK_BRAKE, 
                *FIGHTER_STATUS_KIND_RUN_BRAKE, 
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_WAIT,
                *FIGHTER_STATUS_KIND_SQUAT_RV].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) || 
            ([*FIGHTER_STATUS_KIND_DASH, 
                *FIGHTER_STATUS_KIND_TURN_DASH, 
                *FIGHTER_STATUS_KIND_RUN].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) || 
            ([*FIGHTER_STATUS_KIND_DASH,
                *FIGHTER_STATUS_KIND_TURN_DASH, 
                *FIGHTER_STATUS_KIND_RUN].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
                return true;
            }
        }
    }
    original!()(boma, flag)
}


#[skyline::hook(replace=WorkModule::enable_transition_term)]
unsafe fn enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN {
        VarModule::on_flag(boma.object(), vars::common::status::IS_DASH_TO_RUN_FRAME);
    }
    original!()(boma, flag)
}
pub fn install() {
    skyline::install_hooks!(
        is_enable_transition_term_hook,
        enable_transition_term_hook
    );
}