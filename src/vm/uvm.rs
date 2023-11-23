use alloc::{collections::BTreeMap, vec::Vec};

use crate::kalloc::{pagealloc::PageGurd, pages::{VAddress, PAddress}};
use super::Section;
pub struct MapBlock {
    pub section: Section,
    va_pa: BTreeMap<VAddress,PAddress>
}
pub struct Uvm {
    root: PageGurd,
    paddr: Vec<MapBlock>
}



