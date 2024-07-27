use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

use truco_domain_engine::juego::{Sin, Truco, TrucoBuilder};
use truco_domain_engine::juego::maquinada_de_estado::Cero;

#[repr(C)]
pub enum ResultEnum {
    Ok = 0,
    Err = 1,
}

#[repr(C)]
pub union ResultVoid {
    value: u8,
    error: *const c_char,
}

#[repr(C)]
pub struct CResult {
    result: ResultEnum,
    error: *const c_char,
}

impl CResult {
    #[must_use]
    pub fn new(result: Result<(), &str>) -> Self {
        match result {
            Ok(()) => Self {
                result: ResultEnum::Ok,
                error: null(),
            },
            Err(s) => Self {
                result: ResultEnum::Err,
                error: str_to_c_char(s),
            },
        }
    }
}

#[repr(C)]
pub enum OptionEnum {
    Some = 0,
    None = 1,
}

#[repr(C)]
pub enum CEquipo {
    Nosotros = 0,
    Ellos = 1,
}

#[repr(C)]
pub struct CUnionEquipo {
    result: OptionEnum,
    some: CEquipo,
}

fn str_to_c_char(s: &str) -> *const c_char {
    let c_string = CString::new(s).expect("CString::new failed");
    c_string.into_raw()
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn irse_al_maso(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).irse_al_maso(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_quiero(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_quiero(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_no_quiero(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_no_quiero(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_envido(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_envido(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_real_envido(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_real_envido(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_falta_envido(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_falta_envido(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_truco(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_truco(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_re_truco(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_re_truco(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn cantar_vale_cuatro(ptr: *const Truco, player: *const c_char) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).cantar_vale_cuatro(&player);
        CResult::new(ret)
    }
}

/// # Errors
/// # Panics
#[no_mangle]
pub extern "C" fn tirar_carta(ptr: *const Truco, player: *const c_char, card: usize) -> CResult {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).tirar_carta(&player, card);
        CResult::new(ret)
    }
}

#[repr(C)]
pub struct CStringArray {
    data: *mut *mut c_char,
    length: usize,
}

impl CStringArray {
    fn from_vec(vec: Vec<String>) -> Self {
        let length = vec.len();
        let mut data = vec
            .into_iter()
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect::<Vec<*mut c_char>>();

        data.shrink_to_fit();
        let data_ptr = data.as_mut_ptr();
        std::mem::forget(data);

        CStringArray {
            data: data_ptr,
            length,
        }
    }

    fn free(self) {
        unsafe {
            if !self.data.is_null() {
                for i in 0..self.length {
                    let _ = CString::from_raw(*self.data.add(i));
                }
                Vec::from_raw_parts(self.data, self.length, self.length);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn free_string_array(array: CStringArray) {
    array.free();
}

/// # Panics
#[no_mangle]
pub extern "C" fn valid_commands(ptr: *const Truco, player: *const c_char) -> CStringArray {
    unsafe {
        assert!(!ptr.is_null());
        let player = CString::from_raw(player.cast_mut()).into_string().unwrap();
        let ret = (*ptr.cast_mut()).valid_commands(&player);
        CStringArray::from_vec(ret)
    }
}

#[no_mangle]
pub extern "C" fn new_truco_builder() -> *const TrucoBuilder<Sin, Cero> {
    Box::into_raw(Box::new(TrucoBuilder::new()))
}
