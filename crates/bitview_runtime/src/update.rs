use std::time::Instant;

use bitview_plugin::UpdateContext;
use brk_error::Result;
use tracing::info;

use crate::{BootstrapAction, ComputePluginSet};

/// Performs and publishes one complete steady-state update.
pub fn update<P>(plugins: &mut P, context: UpdateContext<'_>) -> Result<()>
where
    P: ComputePluginSet,
{
    run(plugins, context, ComputePluginSet::compute)
}

pub fn bootstrap_update<P>(plugins: &mut P, context: UpdateContext<'_>) -> Result<BootstrapAction>
where
    P: ComputePluginSet,
{
    run(plugins, context, ComputePluginSet::bootstrap_compute)
}

fn run<P, T>(
    plugins: &mut P,
    context: UpdateContext<'_>,
    compute: impl FnOnce(&mut P, UpdateContext<'_>) -> Result<T>,
) -> Result<T>
where
    P: ComputePluginSet,
{
    plugins.for_each_plugin(&mut |plugin| plugin.gate().begin_update());

    let start = Instant::now();
    let output = compute(plugins, context)?;
    plugins.commit()?;
    plugins.for_each_plugin(&mut |plugin| plugin.gate().finish_update());
    info!("Update completed in {:.2?}", start.elapsed());
    Ok(output)
}

#[cfg(test)]
#[path = "../tests/unit/update.rs"]
mod tests;
