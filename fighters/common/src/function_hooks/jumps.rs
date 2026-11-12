use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0x6d2194, inline)]
unsafe fn fullhop_initial_y_speed_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(ctx.registers[8].x());
    let work_module = ctx.registers[0].x();
    let jump_y = callable(work_module, smash::hash40("jump_y"), 0);
    let gravity = callable(work_module, smash::hash40("air_accel_y"), 0);
    let initital_jump_vel = (jump_y * gravity * 2.0).sqrt() + (0.5 * gravity);
    ctx.registers_f[0].set_s(initital_jump_vel)
}

#[skyline::hook(offset = 0x6ce6d8, inline)]
unsafe fn jump1_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

#[skyline::hook(offset = 0x6ce70c, inline)]
unsafe fn jump1_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(ctx.registers[8].x());
    let work_module = ctx.registers[0].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let jump_speed_x_max = run_speed_max;
    ctx.registers_f[0].set_s(jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d19c4, inline)]
unsafe fn jump2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

#[skyline::hook(offset = 0x6d19f8, inline)]
unsafe fn jump2_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(ctx.registers[8].x());
    let work_module = ctx.registers[0].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let jump_speed_x_max = run_speed_max;
    ctx.registers_f[0].set_s(jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d1b10, inline)]
unsafe fn jump3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

#[skyline::hook(offset = 0x6d1b44, inline)]
unsafe fn jump3_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(ctx.registers[8].x());
    let work_module = ctx.registers[0].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let jump_speed_x_max = run_speed_max;
    ctx.registers_f[0].set_s(jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d0454, inline)]
unsafe fn jump4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

#[skyline::hook(offset = 0x6d04e4, inline)]
unsafe fn jump4_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(ctx.registers[8].x());
    let work_module = ctx.registers[0].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let jump_speed_x_max = run_speed_max;
    ctx.registers_f[0].set_s(jump_speed_x_max)
}

#[skyline::hook(offset = 0x6ce7d0, inline)]
unsafe fn jump_aerial_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

#[skyline::hook(offset = 0x6d05cc, inline)]
unsafe fn jump_aerial_2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

#[skyline::hook(offset = 0x6d117c, inline)]
unsafe fn jump_aerial_3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

#[skyline::hook(offset = 0x6ce28c, inline)]
unsafe fn jump_aerial_4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[0].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    ctx.registers_f[0].set_s(left_stick_x)
}

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x6d2194).nop();
        skyline::patching::Patch::in_text(0x6ce6d8).nop();
        skyline::patching::Patch::in_text(0x6d19c4).nop();
        skyline::patching::Patch::in_text(0x6d1b10).nop();
        skyline::patching::Patch::in_text(0x6d0454).nop();
        skyline::patching::Patch::in_text(0x6ce7d0).nop();
        skyline::patching::Patch::in_text(0x6d05cc).nop();
        skyline::patching::Patch::in_text(0x6d117c).nop();
        skyline::patching::Patch::in_text(0x6ce28c).nop();
        skyline::patching::Patch::in_text(0x6ce70c).nop();
        skyline::patching::Patch::in_text(0x6d19f8).nop();
        skyline::patching::Patch::in_text(0x6d1b44).nop();
        skyline::patching::Patch::in_text(0x6d04e4).nop();
    }
    skyline::install_hooks!(
        fullhop_initial_y_speed_hook,
        jump1_stick_x_hook,
        jump1_jump_speed_x_max_hook,
        jump2_stick_x_hook,
        jump2_jump_speed_x_max_hook,
        jump3_stick_x_hook,
        jump3_jump_speed_x_max_hook,
        jump4_stick_x_hook,
        jump4_jump_speed_x_max_hook,
        jump_aerial_stick_x_hook,
        jump_aerial_2_stick_x_hook,
        jump_aerial_3_stick_x_hook,
        jump_aerial_4_stick_x_hook,
    );
}