use super::*;
use crate::globals::*;
use crate::opff::physics::get_player_number;

pub mod controls;
pub mod energy;
pub mod kinetic;
pub mod momentum_transfer;
pub mod transition;
pub mod jumps;
mod lua_bind_hook;
mod game;

pub fn install() {
    controls::install();
    energy::install();
    kinetic::install();
    momentum_transfer::install();
    //jumps::install();
    transition::install();
    lua_bind_hook::install();
    game::install();

    unsafe {
        /*
        const KILL_ZOOM_DATA: u32 = 0xD503201Fu32;
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_regular()).nop();
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_throw()).data(KILL_ZOOM_DATA);
        */
        // Allows airtime counter (FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR)
        // to increment when on ledge (SITUATION_KIND_CLIFF)
        // Matches Melee behavior
        //skyline::patching::Patch::in_text(0x614fb4).data(0x54000060);
        skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32); //Removes phantoms
    }
}