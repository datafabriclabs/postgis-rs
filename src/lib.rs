use postgis_sys::mvt_geom as raw_mvt_geom;

pub fn mvt_geom(
    geom: &lwgeom::LWGeom,
    gbox: &lwgeom::GBox,
    extend: u32,
    buffer: u32,
    clip_geom: bool,
) -> lwgeom::LWGeom {
    let x = unsafe { raw_mvt_geom(geom.as_ptr(), gbox.as_ptr(), extend, buffer, clip_geom) };
    lwgeom::LWGeom::from_ptr(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mvt_geom() {
        let gbox = lwgeom_sys::GBOX {
            flags: 0,
            xmin: 0.,
            xmax: 4096.,
            ymin: 0.,
            ymax: 4086.,
            zmin: 0.,
            zmax: 0.,
            mmin: 0.,
            mmax: 0.,
        };

        let x = mvt_geom(
            &lwgeom::LWGeom::from_ewkt("POLYGON ((0 0, 10 0, 10 5, 0 -5, 0 0))").unwrap(),
            &lwgeom::GBox::from_ptr(&gbox as *const lwgeom_sys::GBOX as *mut lwgeom_sys::GBOX),
            4096,
            0,
            false,
        );

        println!("{}", x.as_ewkt(None).unwrap());
    }
}
