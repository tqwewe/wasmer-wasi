pub enum Error {
    Command(Vec<u8>),
    DeserializeState,
    DeserializeEvent,
    DeserializeCommand,
    SerializeState,
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Command(e) => f.debug_tuple("Error::Command").field(e).finish(),
            Error::DeserializeState => f.debug_tuple("Error::DeserializeState").finish(),
            Error::DeserializeEvent => f.debug_tuple("Error::DeserializeEvent").finish(),
            Error::DeserializeCommand => f.debug_tuple("Error::DeserializeCommand").finish(),
            Error::SerializeState => f.debug_tuple("Error::SerializeState").finish(),
        }
    }
}
#[export_name = "new-instance"]
unsafe extern "C" fn __wit_bindgen_new_instance(arg0: i32, arg1: i32) -> i32 {
    let len0 = arg1 as usize;
    let result1 = <super::Domain as Domain>::new_instance(
        String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
    );
    let (result5_0, result5_1, result5_2, result5_3) = match result1 {
        Ok(e) => {
            let vec2 = (e).into_boxed_slice();
            let ptr2 = vec2.as_ptr() as i32;
            let len2 = vec2.len() as i32;
            core::mem::forget(vec2);
            (0i32, ptr2, len2, 0i32)
        }
        Err(e) => {
            let (result4_0, result4_1, result4_2) = match e {
                Error::Command(e) => {
                    let vec3 = (e).into_boxed_slice();
                    let ptr3 = vec3.as_ptr() as i32;
                    let len3 = vec3.len() as i32;
                    core::mem::forget(vec3);
                    (0i32, ptr3, len3)
                }
                Error::DeserializeState => (1i32, 0i32, 0i32),
                Error::DeserializeEvent => (2i32, 0i32, 0i32),
                Error::DeserializeCommand => (3i32, 0i32, 0i32),
                Error::SerializeState => (4i32, 0i32, 0i32),
            };
            (1i32, result4_0, result4_1, result4_2)
        }
    };
    let ptr6 = RET_AREA.as_mut_ptr() as i32;
    *((ptr6 + 24) as *mut i32) = result5_3;
    *((ptr6 + 16) as *mut i32) = result5_2;
    *((ptr6 + 8) as *mut i32) = result5_1;
    *((ptr6 + 0) as *mut i32) = result5_0;
    ptr6
}
#[export_name = "apply-events"]
unsafe extern "C" fn __wit_bindgen_apply_events(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32 {
    let len0 = arg1 as usize;
    let base2 = arg2;
    let len2 = arg3;
    let mut result2 = Vec::with_capacity(len2 as usize);
    for i in 0..len2 {
        let base = base2 + i * 8;
        result2.push({
            let len1 = *((base + 4) as *const i32) as usize;
            Vec::from_raw_parts(*((base + 0) as *const i32) as *mut _, len1, len1)
        });
    }
    std::alloc::dealloc(
        base2 as *mut _,
        std::alloc::Layout::from_size_align_unchecked((len2 as usize) * 8, 4),
    );
    let result3 = <super::Domain as Domain>::apply_events(
        Vec::from_raw_parts(arg0 as *mut _, len0, len0),
        result2,
    );
    let (result7_0, result7_1, result7_2, result7_3) = match result3 {
        Ok(e) => {
            let vec4 = (e).into_boxed_slice();
            let ptr4 = vec4.as_ptr() as i32;
            let len4 = vec4.len() as i32;
            core::mem::forget(vec4);
            (0i32, ptr4, len4, 0i32)
        }
        Err(e) => {
            let (result6_0, result6_1, result6_2) = match e {
                Error::Command(e) => {
                    let vec5 = (e).into_boxed_slice();
                    let ptr5 = vec5.as_ptr() as i32;
                    let len5 = vec5.len() as i32;
                    core::mem::forget(vec5);
                    (0i32, ptr5, len5)
                }
                Error::DeserializeState => (1i32, 0i32, 0i32),
                Error::DeserializeEvent => (2i32, 0i32, 0i32),
                Error::DeserializeCommand => (3i32, 0i32, 0i32),
                Error::SerializeState => (4i32, 0i32, 0i32),
            };
            (1i32, result6_0, result6_1, result6_2)
        }
    };
    let ptr8 = RET_AREA.as_mut_ptr() as i32;
    *((ptr8 + 24) as *mut i32) = result7_3;
    *((ptr8 + 16) as *mut i32) = result7_2;
    *((ptr8 + 8) as *mut i32) = result7_1;
    *((ptr8 + 0) as *mut i32) = result7_0;
    ptr8
}
#[export_name = "handle-command"]
unsafe extern "C" fn __wit_bindgen_handle_command(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
) -> i32 {
    let len0 = arg1 as usize;
    let len1 = arg3 as usize;
    let result2 = <super::Domain as Domain>::handle_command(
        Vec::from_raw_parts(arg0 as *mut _, len0, len0),
        Vec::from_raw_parts(arg2 as *mut _, len1, len1),
    );
    let (result7_0, result7_1, result7_2, result7_3) = match result2 {
        Ok(e) => {
            let vec4 = e;
            let len4 = vec4.len() as i32;
            let layout4 = core::alloc::Layout::from_size_align_unchecked(vec4.len() * 8, 4);
            let result4 = std::alloc::alloc(layout4);
            if result4.is_null() {
                std::alloc::handle_alloc_error(layout4);
            }
            for (i, e) in vec4.into_iter().enumerate() {
                let base = result4 as i32 + (i as i32) * 8;
                {
                    let vec3 = (e).into_boxed_slice();
                    let ptr3 = vec3.as_ptr() as i32;
                    let len3 = vec3.len() as i32;
                    core::mem::forget(vec3);
                    *((base + 4) as *mut i32) = len3;
                    *((base + 0) as *mut i32) = ptr3;
                }
            }
            (0i32, result4 as i32, len4, 0i32)
        }
        Err(e) => {
            let (result6_0, result6_1, result6_2) = match e {
                Error::Command(e) => {
                    let vec5 = (e).into_boxed_slice();
                    let ptr5 = vec5.as_ptr() as i32;
                    let len5 = vec5.len() as i32;
                    core::mem::forget(vec5);
                    (0i32, ptr5, len5)
                }
                Error::DeserializeState => (1i32, 0i32, 0i32),
                Error::DeserializeEvent => (2i32, 0i32, 0i32),
                Error::DeserializeCommand => (3i32, 0i32, 0i32),
                Error::SerializeState => (4i32, 0i32, 0i32),
            };
            (1i32, result6_0, result6_1, result6_2)
        }
    };
    let ptr8 = RET_AREA.as_mut_ptr() as i32;
    *((ptr8 + 24) as *mut i32) = result7_3;
    *((ptr8 + 16) as *mut i32) = result7_2;
    *((ptr8 + 8) as *mut i32) = result7_1;
    *((ptr8 + 0) as *mut i32) = result7_0;
    ptr8
}
pub trait Domain {
    fn new_instance(id: String) -> Result<Vec<u8>, Error>;
    fn apply_events(state: Vec<u8>, event: Vec<Vec<u8>>) -> Result<Vec<u8>, Error>;
    fn handle_command(state: Vec<u8>, command: Vec<u8>) -> Result<Vec<Vec<u8>>, Error>;
}
static mut RET_AREA: [i64; 4] = [0; 4];

const _ : & str = "// Errors which can occur\nvariant error {\n  command(list<u8>),\n  deserialize-state,\n  deserialize-event,\n  deserialize-command,\n  serialize-state,\n}\n\n// Creates a new instance\n// Returns state\nnew-instance: function(id: string) -> expected<list<u8>, error>\n\n// Takes state, event\n// Returns new state\napply-events: function(state: list<u8>, event: list<list<u8>>) -> expected<list<u8>, error>\n\n// Takes state, command\n// Returns list of events\nhandle-command: function(state: list<u8>, command: list<u8>) -> expected<list<list<u8>>, error>\n" ;
