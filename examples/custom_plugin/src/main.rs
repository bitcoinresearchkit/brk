use bitview_custom_plugin_example::composition::Plugins;
use bitviewd::run;
use brk_error::Result;

fn main() -> Result<()> {
    run(Plugins::import)
}
