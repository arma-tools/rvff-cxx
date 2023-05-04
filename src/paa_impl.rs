use rvff::paa::Mipmap;

use crate::bridge::MipmapCxx;

impl From<Mipmap> for MipmapCxx {
    fn from(mm: Mipmap) -> Self {
        Self {
            width: mm.width,
            height: mm.height,
            data: mm.data,
        }
    }
}
