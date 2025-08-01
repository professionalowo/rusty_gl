unsafe extern "C" {
    unsafe fn glGenVertexArrays(n: u32, arrays: *mut u32);
    unsafe fn glBindVertexArray(n: u32);
    unsafe fn glDeleteVertexArrays(n: u32, arrays: *const u32);
}

pub fn gen_vertex_arrays() -> u32 {
    let mut vao = 0;
    unsafe {
        glGenVertexArrays(1, &mut vao);
    }
    vao
}

pub fn bind_vertex_array(vao: u32) {
    unsafe {
        glBindVertexArray(vao);
    }
}

pub fn delete_vertex_array(vao: u32) {
    unsafe {
        glDeleteVertexArrays(1, &vao);
    }
}
