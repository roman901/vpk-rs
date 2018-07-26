use header::VPKHeader;

#[derive(Debug)]
pub struct VPKBundle {
    header: VPKHeader
}

impl VPKBundle {
    pub fn new(header: VPKHeader) -> VPKBundle {
        VPKBundle { header }
    }
}