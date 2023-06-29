


pub fn exec(_: &mut crate::screen::Screen, _: Vec<String>) -> color_eyre::Result<usize> {
    unsafe {
        *crate::runner::MACHINE_STATUS.get_mut().unwrap().get_mut() |= crate::runner::machine_status_bits::MS_SHOULD_EXIT;
    }

    Ok(0)
}