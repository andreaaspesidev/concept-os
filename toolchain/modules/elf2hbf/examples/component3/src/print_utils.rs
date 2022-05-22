use userlib::{sys_send, TaskId};

#[no_mangle]
#[inline(never)]
pub fn print(text: &str) {
    // Mock call
    sys_send(
        TaskId::UNBOUND, 
        1, 
        text.as_bytes(), 
        &mut [],
        &[],
    );
}