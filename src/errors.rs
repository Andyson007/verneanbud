//! Cleans up the terminal when the application panics
use std::{io::stdout, panic};

use color_eyre::{config::HookBuilder, eyre};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

/// This replaces the standard `color_eyre` panic and error hooks with hooks that
/// restore the terminal before printing the panic or error.
///
/// # Panics
/// It colud panic if `disable_raw_mode` returned an error unhandelable
///
/// # Errors
/// Don't think they are handelable
pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            execute!(stdout(), LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();

            eyre_hook(error)
        },
    ))?;

    Ok(())
}
