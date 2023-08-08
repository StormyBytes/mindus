//! grass
use crate::block::make_register;
use crate::block::simple::BasicBlock;

macro_rules! register_env {
    ($($field:literal: $size:literal;)+) => {
        make_register!(
            $($field -> BasicBlock::new($size, true, &[]);)*
        );
    };
}

register_env! {
    "darksand": 1;
    "sand-floor": 1;
    "yellow-stone": 1;
    "arkyic-stone": 1;
    "carbon-stone": 1;
    "ore-beryllium": 1;
    "ore-copper": 1;
    "ore-lead": 1;
    "ore-coal": 1;
    "ore-scrap": 1;
    "ore-thorium": 1;
    "ore-titanium": 1;
    "ore-tungsten": 1;
    "ore-crystal-thorium": 1;
    "ore-wall-beryllium": 1;
    "ore-wall-thorium": 1;
    "ore-wall-tungsten": 1;
    "graphitic-wall": 1;
    "graphitic-wall-large": 2;
    "dacite": 1;
    "dirt": 1;
    "arkycite-floor": 1;
    "basalt": 1;
    "ice": 1;
    "molten-slag": 1;
    "moss": 1;
    "mud": 1;
    "magmarock": 1;
    "grass": 1;
    "ice-snow": 1;
    "hotrock": 1;
    "char": 1;
    "snow": 1;
    "salt": 1;
    "shale": 1;
    "metal-floor": 1;
    "metal-floor-2": 1;
    "metal-floor-3": 1;
    "metal-floor-4": 1;
    "metal-floor-5": 1;
    "dark-panel-1": 1;
    "dark-panel-2": 1;
    "dark-panel-3": 1;
    "dark-panel-4": 1;
    "dark-panel-5": 1;
    "dark-panel-6": 1;
    "darksand-tainted-water": 1;
    "darksand-water": 1;
    "deep-tainted-water": 1;
    "deep-water": 1;
    "sand-water": 1;
    "shallow-water": 1;
    "space": 1;
    "stone": 1;
    "build1": 1;
    "boulder": 1;
    "arkyic-vent": 3;
    "arkyic-wall-large": 2;
    "arkyic-wall": 1;
    "beryllic-stone-wall-large": 2;
    "beryllic-stone-wall": 1;
    "beryllic-stone": 1;
    "bluemat": 1;
    "carbon-vent": 3;
    "carbon-wall-large": 2;
    "carbon-wall": 1;
    "cliff": 1;
    "core-zone": 1;
    "crater-stone": 1;
    "crystal-floor": 1;
    "crystalline-stone-wall-large": 2;
    "crystalline-stone-wall": 1;
    "crystalline-stone": 1;
    "crystalline-vent": 3;
    "dacite-wall-large": 2;
    "dacite-wall": 1;
    "dark-metal-large": 2;
    "dark-metal": 1;
    "metal-floor-damaged": 1;
    "dense-red-stone": 1;
    "dirt-wall-large": 2;
    "dirt-wall": 1;
    "dune-wall-large": 2;
    "dune-wall": 1;
    "ferric-craters": 1; // ferris section
    "ferric-stone-wall-large": 2;
    "ferric-stone-wall": 1;
    "ferric-stone": 1;
    "ice-wall-large": 2;
    "ice-wall": 1;
    "pebbles": 1;
    "pine": 1;
    "pooled-cryofluid": 1;
    "red-diamond-wall": 1;
    "red-ice-wall-large": 2;
    "red-ice-wall": 1;
    "red-ice": 1;
    "red-stone-vent": 3;
    "red-stone-wall-large": 2;
    "red-stone-wall": 1;
    "red-stone": 1;
    "redmat": 1;
    "regolith-wall-large": 2;
    "regolith-wall": 1;
    "regolith": 1;
    "rhyolite-crater": 1;
    "rhyolite-vent": 3;
    "rhyolite-wall-large": 2;
    "rhyolite-wall": 1;
    "rhyolite": 1;
    "rough-rhyolite": 1;
    "salt-wall-large": 2;
    "salt-wall": 1;
    "sand-wall-large": 2;
    "sand-wall": 1;
    "shale-wall-large": 2;
    "shale-wall": 1;
    "shrubs-large": 2;
    "shrubs": 1;
    "snow-pine": 1;
    "snow-wall-large": 2;
    "snow-wall": 1;
    "spawn": 1;
    "spore-moss": 1;
    "spore-pine": 1;
    "spore-wall-large": 2;
    "spore-wall": 1;
    "stone-wall-large": 2;
    "stone-wall": 1;
    "tainted-water": 1;
    "tar": 1;
    "yellow-stone-plates": 1;
    "yellow-stone-vent": 3;
    "yellow-stone-wall-large": 2;
    "yellow-stone-wall": 1;
    // props
    "yellow-stone-boulder": 1;
    "snow-boulder": 1;
    "shale-boulder": 1;
    "arkyic-boulder": 1;
    "basalt-boulder": 1;
    "beryllic-boulder": 1;
    "carbon-boulder": 1;
    "crystalline-boulder": 1;
    "dacite-boulder": 1;
    "ferric-boulder": 1;
    "red-ice-boulder": 1;
    "red-stone-boulder": 1;
    "rhyolite-boulder": 1;
    "sand-boulder": 1;
    "pur-bush": 1;
    "tendrils": 1;
    // these are tall but uh (TODO layering)
    "white-tree-dead": 1;
    "yellowcoral": 1;
    "white-tree": 1;
    "redweed": 1;
    "spore-cluster": 1;
    "crystal-blocks": 1;
    "crystal-cluster": 1;
    "vibrant-crystal-cluster": 1;
    "crystal-orbs": 1;
    // end tall
    "build2": 1;
    "build3": 1;
    "build4": 1;
    "build5": 1;
    "build6": 1;
    "build7": 1;
    "build8": 1;
    "build9": 1;
    "build10": 1;
    "build11": 1;
    "build12": 1;
    "build13": 1;
    "build14": 1;
    "build15": 1;
    "build16": 1;
}
