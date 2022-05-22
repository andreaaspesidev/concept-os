use super::HbfPayloadSection;



// Does not exist in the hbf
pub struct HbfPayloadSectionGen {
    pub offset: u32,
    pub size: u32,
}

impl HbfPayloadSection for HbfPayloadSectionGen {
    fn size(&self) -> u32 {
        self.size
    }

    fn offset(&self) -> u32 {
        self.offset
    }
}