// status imports
use super::*;
use globals::*;

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

//Turn dash (runs once at the beginning of the status)
//fixes "loose" feeling that ult dashdances had... still not perfect but with this its definitely better
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_TurnDash_Sub)]
pub unsafe fn status_turndash_sub_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);

    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_TurnDash_Main)]
pub unsafe fn status_turndash_main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0) * PostureModule::lr(boma);
	let stick_x = fighter.global_table[STICK_X].get_f32();
	//println!("Current dash speed: {}", dash_speed);

	if MotionModule::frame(boma) >= 1.0 && MotionModule::frame(boma) <= 2.0 && stick_x != 0.0 {
		//println!("Changing current dash speed");
		f_agent.clear_lua_stack();
		f_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
		f_agent.push_lua_stack(&mut L2CValue::new_num(dash_speed));
		sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		f_agent.clear_lua_stack();
	}
	ret
}


fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_turndash_sub_hook,
            status_turndash_main_hook
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}