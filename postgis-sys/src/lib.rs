extern "C" {
    pub fn mvt_geom(
        geom: *const lwgeom_sys::LWGEOM,
        gbox: *const lwgeom_sys::GBOX,
        extend: u32,
        buffer: u32,
        clip_geom: bool,
    ) -> *mut lwgeom_sys::LWGEOM;
}

extern "C" {
    fn foo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_foo() {
        unsafe { super::foo() };
    }
}
