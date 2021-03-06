// This file was generated by gir (dc86d20) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use translate::*;

bitflags! {
    pub flags KeyFileFlags: u32 {
        const KEY_FILE_NONE = 0,
        const KEY_FILE_KEEP_COMMENTS = 1,
        const KEY_FILE_KEEP_TRANSLATIONS = 2,
    }
}

#[doc(hidden)]
impl ToGlib for KeyFileFlags {
    type GlibType = ffi::GKeyFileFlags;

    fn to_glib(&self) -> ffi::GKeyFileFlags {
        ffi::GKeyFileFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileFlags> for KeyFileFlags {
    fn from_glib(value: ffi::GKeyFileFlags) -> KeyFileFlags {
        KeyFileFlags::from_bits_truncate(value.bits())
    }
}

