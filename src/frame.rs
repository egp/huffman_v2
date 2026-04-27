// Frame system skeleton (intentionally minimal for now)

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Header = 0,
    FrequencyTableInternal = 1,
    FrequencyTableExternal = 2,
    Payload = 3,
}
