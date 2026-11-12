#![allow(static_mut_refs)] //Addresses creating a mutable reference to mutable static
use super::*;
use smash::app::lua_bind;
use rand::Rng;

//Credit to HDR, reduces rim lighting on fighters to 0.5 strength

static mut DATA_ACCESS_LOCK: [u8; 0x20] = [0; 0x20];

#[skyline::hook(offset = 0x3803080)]
unsafe fn set_uniform_buffer(stage: u64, index: u64, buffer: u64) {
    let cbuf = *((buffer + 8) as *const u64);
    let buffer_ptr = *((cbuf + 0x98) as *const u64);
    if buffer_ptr == 0 {
        return call_original!(stage, index, buffer);
    }
    let size = *((cbuf + 0x10) as *const usize);
    let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
    let func = *(text.add(0x5940d28) as *const u64);
    let func: extern "C" fn(*mut u8) -> *mut u8 = std::mem::transmute(func);
    let map = func(buffer_ptr as _);
    let data = std::slice::from_raw_parts_mut(map, size);
    skyline::nn::os::LockMutex(DATA_ACCESS_LOCK.as_mut_ptr().cast());
    if index == 8 {
        data[0xAC as usize..0xB0 as usize].copy_from_slice(&vec![0x00, 0x00, 0x00, 0x3F]);  // 0x00, 0x00, 0x00, 0x3F = 0.5 floating point
    }
    let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
    let func = *(text.add(0x5940d38) as *const u64);
    let func: extern "C" fn(*mut u8, isize, usize) = std::mem::transmute(func);
    func(buffer_ptr as _, 0, size);
    skyline::nn::os::UnlockMutex(DATA_ACCESS_LOCK.as_mut_ptr().cast());
    call_original!(stage, index, buffer);
}

//Disables Training Mode Reset from resetting music (Credit to HDR)
#[skyline::from_offset(0x23edc60)]
unsafe extern "C" fn music_function1(arg: u64);

#[skyline::from_offset(0x23ee510)]
unsafe extern "C" fn music_function2(arg: u64, arg2: u64);

#[skyline::hook(offset = 0x1509fd4, inline)]
unsafe extern "C" fn training_reset_music1(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function1(ctx.registers[0].x());
    }
}

#[skyline::hook(offset = 0x14f99cc, inline)]
unsafe extern "C" fn training_reset_music2(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function2(ctx.registers[0].x(), ctx.registers[1].x());
    }
}

//Credited to HDR, makes the main menu load quicker
#[skyline::hook(offset = 0x235cf20, inline)]
unsafe extern "C" fn main_menu_quick(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const skyline::hooks::InlineCtx as *mut u8).add(0x100);
    *(sp.add(0x60) as *mut u64) = 0x1100000000;
    let slice = std::slice::from_raw_parts_mut(sp.add(0x68), 18);
    slice.copy_from_slice(b"MenuSequenceScene\0");
}

//Credited to HDR, retains your previously selected character when returning from a match exit to the CSS
#[skyline::hook(offset = 0x17989e8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = ctx.registers[0].x() as *mut u64;
    let ptr2 = ctx.registers[1].x() as *mut u64;
    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

static mut SHOULD_END_RESULT_SCREEN: bool = false;

// Skip results screen with start button
#[skyline::hook(offset = 0x3665670)]
unsafe fn process_inputs_handheld(controller: &mut Controller) {
    let entry_count = lua_bind::FighterManager::entry_count(utils::singletons::FighterManager());
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager())
        && entry_count > 0
    {
        if ninput::any::is_press(ninput::Buttons::PLUS) {
            SHOULD_END_RESULT_SCREEN = true;
        }
        if ninput::any::is_press(ninput::Buttons::B) {
            SHOULD_END_RESULT_SCREEN = false;
        }
        if SHOULD_END_RESULT_SCREEN {
            let mut rng = rand::thread_rng();
            // Need to space apart A-presses so it does not seem like we are holding the button.
            let n: u32 = rng.gen_range(0..3);
            if n == 1 {
                controller.current_buttons.set_a(true);
                controller.just_down.set_a(true);
            }
        }
    }
    if entry_count == 0 {
        SHOULD_END_RESULT_SCREEN = false;
    }
    call_original!(controller);
}

pub fn install() {
    //Overwrites the vanilla functions regarding resetting the training mode music to use the custom functions above
    let _ = skyline::patching::Patch::in_text(0x14f99cc).nop().unwrap();
    let _ = skyline::patching::Patch::in_text(0x1509fd4).nop().unwrap();
	skyline::install_hooks!(
        main_menu_quick,
        fix_chara_replace,
        process_inputs_handheld,
        training_reset_music1,
        training_reset_music2,
        set_uniform_buffer
    );
}
