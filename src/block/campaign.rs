//! campaign blocks
use crate::block::make_register;
use crate::block::simple::{cost, BasicBlock};
make_register! {
    "launch-pad" -> BasicBlock::new(3, true, cost!(Copper: 350, Lead: 200, Titanium: 150, Silicon: 140));
    "interplanetary-accelerator" -> BasicBlock::new(7, true, cost!(Copper: 16000, Silicon: 11000, Thorium: 13000, Titanium: 12000, SurgeAlloy: 6000, PhaseFabric: 5000));
}
