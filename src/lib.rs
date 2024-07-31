use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null;

use truco_domain_engine::equipos::Equipo;
use truco_domain_engine::juego::{Con, Sin, Truco, TrucoBuilder};
use truco_domain_engine::maquina_de_estados::{Cero, Cinco, Cuatro, Dos, Seis, Tres, Uno};

#[repr(C)]
pub enum ResultEnum {
    Ok = 0,
    Err = 1,
}

#[repr(C)]
pub struct CResult {
    tag: ResultEnum,
    error: *const c_char,
}

impl CResult {
    #[must_use]
    pub fn new(result: Result<(), &str>) -> Self {
        match result {
            Ok(()) => Self {
                tag: ResultEnum::Ok,
                error: null(),
            },
            Err(s) => Self {
                tag: ResultEnum::Err,
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

impl CEquipo {
    #[must_use]
    pub fn new(eq: Equipo) -> Self {
        match eq {
            Equipo::Nosotros => CEquipo::Nosotros,
            Equipo::Ellos => CEquipo::Ellos,
        }
    }
}

#[repr(C)]
pub struct COptionEquipo {
    tag: OptionEnum,
    some: CEquipo,
}

impl COptionEquipo {
    #[must_use]
    pub fn new(og: Option<Equipo>) -> Self {
        let (tag, some) = match og {
            Some(eq) => (OptionEnum::Some, CEquipo::new(eq)),
            None => (OptionEnum::None, CEquipo::Ellos),
        };
        Self { tag, some }
    }
}

fn str_to_c_char(s: &str) -> *const c_char {
    let c_string = CString::new(s).expect("CString::new failed");
    c_string.into_raw()
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn irse_al_maso(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).irse_al_maso(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_quiero(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_quiero(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_no_quiero(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_no_quiero(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_envido(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_envido(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_real_envido(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_real_envido(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_falta_envido(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_falta_envido(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_truco(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_truco(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_re_truco(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_re_truco(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cantar_vale_cuatro(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).cantar_vale_cuatro(player);
    CResult::new(ret)
}

/// # Errors
/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn tirar_carta(
    ptr: *const Truco,
    player: *const c_char,
    card: usize,
) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let ret = (*ptr.cast_mut()).tirar_carta(player, card);
    CResult::new(ret)
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
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn valid_commands(ptr: *const Truco, player: *const c_char) -> CStringArray {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CStringArray::from_vec(vec![e.to_string()]),
    };
    let ret = (*ptr.cast_mut()).valid_commands(player);
    CStringArray::from_vec(ret)
}

/// # Panics
#[no_mangle]
pub extern "C" fn terminado(ptr: *const Truco) -> bool {
    unsafe {
        assert!(!ptr.is_null());
        (*ptr.cast_mut()).terminado()
    }
}

/// # Panics
#[no_mangle]
pub extern "C" fn ganador(ptr: *const Truco) -> COptionEquipo {
    unsafe {
        assert!(!ptr.is_null());
        let res = (*ptr.cast_mut()).ganador();
        COptionEquipo::new(res)
    }
}

/// # Panics
#[no_mangle]
pub extern "C" fn print_state(ptr: *const Truco) -> *const c_char {
    unsafe {
        assert!(!ptr.is_null());
        let res = (*ptr.cast_mut()).print_state();
        str_to_c_char(&res)
    }
}

/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn print_player(ptr: *const Truco, player: *const c_char) -> CResult {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    let player = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => return CResult::new(Err(&e.to_string())),
    };
    let res = (*ptr.cast_mut()).print_player(player);
    match res {
        Ok(s) => CResult {
            tag: ResultEnum::Ok,
            error: str_to_c_char(&s),
        },
        Err(s) => CResult {
            tag: ResultEnum::Err,
            error: str_to_c_char(s),
        },
    }
}

#[repr(C)]
#[derive(Clone)]
pub enum TrucoBuilderEnum {
    SinCero(TrucoBuilder<Sin, Cero>),
    SinUno(TrucoBuilder<Sin, Uno>),
    SinDos(TrucoBuilder<Sin, Dos>),
    SinTres(TrucoBuilder<Sin, Tres>),
    SinCuatro(TrucoBuilder<Sin, Cuatro>),
    SinCinco(TrucoBuilder<Sin, Cinco>),
    SinSeis(TrucoBuilder<Sin, Seis>),
    ConCero(TrucoBuilder<Con, Cero>),
    ConUno(TrucoBuilder<Con, Uno>),
    ConDos(TrucoBuilder<Con, Dos>),
    ConTres(TrucoBuilder<Con, Tres>),
    ConCuatro(TrucoBuilder<Con, Cuatro>),
    ConCinco(TrucoBuilder<Con, Cinco>),
    ConSeis(TrucoBuilder<Con, Seis>),
}

#[no_mangle]
pub extern "C" fn new_truco_builder() -> *const TrucoBuilderEnum {
    Box::into_raw(Box::new(TrucoBuilderEnum::SinCero(TrucoBuilder::new())))
}

/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn add_player(ptr: *const TrucoBuilderEnum, player: *const c_char) {
    assert!(!ptr.is_null());
    let c_str = CStr::from_ptr(player);
    if let Ok(player) = c_str.to_str() {
        let current_value = std::ptr::read(ptr);
        let ret = match current_value {
            TrucoBuilderEnum::SinCero(tb) => TrucoBuilderEnum::SinUno(tb.add_player(player)),
            TrucoBuilderEnum::SinUno(tb) => TrucoBuilderEnum::SinDos(tb.add_player(player)),
            TrucoBuilderEnum::SinDos(tb) => TrucoBuilderEnum::SinTres(tb.add_player(player)),
            TrucoBuilderEnum::SinTres(tb) => TrucoBuilderEnum::SinCuatro(tb.add_player(player)),
            TrucoBuilderEnum::SinCuatro(tb) => TrucoBuilderEnum::SinCinco(tb.add_player(player)),
            TrucoBuilderEnum::SinCinco(tb) => TrucoBuilderEnum::SinSeis(tb.add_player(player)),
            TrucoBuilderEnum::SinSeis(tb) => TrucoBuilderEnum::SinSeis(tb),
            TrucoBuilderEnum::ConCero(tb) => TrucoBuilderEnum::ConUno(tb.add_player(player)),
            TrucoBuilderEnum::ConUno(tb) => TrucoBuilderEnum::ConDos(tb.add_player(player)),
            TrucoBuilderEnum::ConDos(tb) => TrucoBuilderEnum::ConTres(tb.add_player(player)),
            TrucoBuilderEnum::ConTres(tb) => TrucoBuilderEnum::ConCuatro(tb.add_player(player)),
            TrucoBuilderEnum::ConCuatro(tb) => TrucoBuilderEnum::ConCinco(tb.add_player(player)),
            TrucoBuilderEnum::ConCinco(tb) => TrucoBuilderEnum::ConSeis(tb.add_player(player)),
            TrucoBuilderEnum::ConSeis(tb) => TrucoBuilderEnum::ConSeis(tb),
        };
        std::ptr::write(ptr.cast_mut(), ret);
    }
}

/// # Panics
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn hasta(ptr: *const TrucoBuilderEnum, hasta: u8) {
    assert!(!ptr.is_null());
    let current_value = std::ptr::read(ptr);
    let ret = match current_value {
        TrucoBuilderEnum::SinCero(tb) => TrucoBuilderEnum::ConCero(tb.hasta(hasta)),
        TrucoBuilderEnum::SinUno(tb) => TrucoBuilderEnum::ConUno(tb.hasta(hasta)),
        TrucoBuilderEnum::SinDos(tb) => TrucoBuilderEnum::ConDos(tb.hasta(hasta)),
        TrucoBuilderEnum::SinTres(tb) => TrucoBuilderEnum::ConTres(tb.hasta(hasta)),
        TrucoBuilderEnum::SinCuatro(tb) => TrucoBuilderEnum::ConCuatro(tb.hasta(hasta)),
        TrucoBuilderEnum::SinCinco(tb) => TrucoBuilderEnum::ConCinco(tb.hasta(hasta)),
        TrucoBuilderEnum::SinSeis(tb) => TrucoBuilderEnum::ConSeis(tb.hasta(hasta)),
        TrucoBuilderEnum::ConCero(tb) => TrucoBuilderEnum::ConCero(tb),
        TrucoBuilderEnum::ConUno(tb) => TrucoBuilderEnum::ConUno(tb),
        TrucoBuilderEnum::ConDos(tb) => TrucoBuilderEnum::ConDos(tb),
        TrucoBuilderEnum::ConTres(tb) => TrucoBuilderEnum::ConTres(tb),
        TrucoBuilderEnum::ConCuatro(tb) => TrucoBuilderEnum::ConCuatro(tb),
        TrucoBuilderEnum::ConCinco(tb) => TrucoBuilderEnum::ConCinco(tb),
        TrucoBuilderEnum::ConSeis(tb) => TrucoBuilderEnum::ConSeis(tb),
    };
    std::ptr::write(ptr.cast_mut(), ret);
}

#[repr(C)]
pub union ResultTruco {
    value: *const Truco,
    error: *const TrucoBuilderEnum,
}

#[repr(C)]
pub struct CResultTruco {
    tag: ResultEnum,
    content: ResultTruco,
}

/// # Panics
#[no_mangle]
pub extern "C" fn build(ptr: *const TrucoBuilderEnum) -> CResultTruco {
    unsafe {
        assert!(!ptr.is_null());
        match (*ptr.cast_mut()).clone() {
            TrucoBuilderEnum::SinCero(_)
            | TrucoBuilderEnum::SinUno(_)
            | TrucoBuilderEnum::SinDos(_)
            | TrucoBuilderEnum::SinTres(_)
            | TrucoBuilderEnum::SinCuatro(_)
            | TrucoBuilderEnum::SinCinco(_)
            | TrucoBuilderEnum::SinSeis(_)
            | TrucoBuilderEnum::ConCero(_)
            | TrucoBuilderEnum::ConUno(_)
            | TrucoBuilderEnum::ConTres(_)
            | TrucoBuilderEnum::ConCinco(_) => CResultTruco {
                tag: ResultEnum::Err,
                content: ResultTruco { error: ptr },
            },
            TrucoBuilderEnum::ConDos(tb) => CResultTruco {
                tag: ResultEnum::Ok,
                content: ResultTruco {
                    value: Box::into_raw(Box::new(tb.build())),
                },
            },
            TrucoBuilderEnum::ConCuatro(tb) => CResultTruco {
                tag: ResultEnum::Ok,
                content: ResultTruco {
                    value: Box::into_raw(Box::new(tb.build())),
                },
            },
            TrucoBuilderEnum::ConSeis(tb) => CResultTruco {
                tag: ResultEnum::Ok,
                content: ResultTruco {
                    value: Box::into_raw(Box::new(tb.build())),
                },
            },
        }
    }
}
