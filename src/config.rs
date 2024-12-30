const MAX_MEMORY_BYTES: usize = 50 * 1024 * 1024 * 1024; // 50GB
const FIELD_ELEMENT_SIZE: usize = 32; // Fr 的大小（字节）

pub fn calculate_max_input_size() -> usize {
    MAX_MEMORY_BYTES / FIELD_ELEMENT_SIZE
}
