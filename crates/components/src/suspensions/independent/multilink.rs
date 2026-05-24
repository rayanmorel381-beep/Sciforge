#[derive(Debug, Clone)]
pub struct MultiLink {
    pub link_count: u8,
    pub subframe_mounted: bool,
    pub camber_adjustment: bool,
    pub toe_adjustment: bool,
    pub aluminum_links: bool,
}

impl MultiLink {
    pub fn five_link(subframe_mounted: bool) -> Self {
        Self { link_count: 5, subframe_mounted, camber_adjustment: true, toe_adjustment: true, aluminum_links: false }
    }

    pub fn four_link() -> Self {
        Self { link_count: 4, subframe_mounted: true, camber_adjustment: true, toe_adjustment: true, aluminum_links: true }
    }
}
