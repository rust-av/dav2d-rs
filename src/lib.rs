use dav2d_sys::*;

pub use av_data::pixel;
use std::ffi::{c_int, c_uint, c_void};
use std::fmt::{self, Debug};
use std::i64;
use std::mem;
use std::ptr;
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    Again,
    Eof,
    InvalidArgument,
    NotEnoughMemory,
    UnsupportedBitstream,
    UnknownError(i32),
}

impl From<i32> for Error {
    fn from(err: i32) -> Self {
        if err == DAV2D_EOF {
            return Error::Eof;
        }
        assert!(err < 0);

        match err {
            DAV2D_ERR_AGAIN => Error::Again,
            DAV2D_ERR_INVAL => Error::InvalidArgument,
            DAV2D_ERR_NOMEM => Error::NotEnoughMemory,
            DAV2D_ERR_NOPROTOOPT => Error::UnsupportedBitstream,
            _ => Error::UnknownError(err),
        }
    }
}

impl Error {
    pub const fn is_again(&self) -> bool {
        matches!(self, Error::Again)
    }

    pub const fn is_eof(&self) -> bool {
        matches!(self, Error::Eof)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Again => write!(fmt, "Try again"),
            Error::Eof => write!(fmt, "End of file"),
            Error::InvalidArgument => write!(fmt, "Invalid argument"),
            Error::NotEnoughMemory => write!(fmt, "Not enough memory available"),
            Error::UnsupportedBitstream => write!(fmt, "Unsupported bitstream"),
            Error::UnknownError(err) => write!(fmt, "Unknown error {}", err),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct PictureParameters {
    pic: ptr::NonNull<Dav2dPicture>,
}

impl PictureParameters {
    pub fn bit_depth(&self) -> usize {
        unsafe { self.pic.as_ref().p.bpc as usize }
    }

    pub fn bits_per_component(&self) -> Option<BitsPerComponent> {
        unsafe {
            match (*self.pic.as_ref().seq_hdr).hbd {
                0 => Some(BitsPerComponent(8)),
                1 => Some(BitsPerComponent(10)),
                2 => Some(BitsPerComponent(12)),
                _ => None,
            }
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { self.pic.as_ref().p.w as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { self.pic.as_ref().p.h as u32 }
    }

    pub fn pixel_layout(&self) -> PixelLayout {
        unsafe {
            #[allow(non_upper_case_globals)]
            match self.pic.as_ref().p.layout {
                DAV2D_PIXEL_LAYOUT_I400 => PixelLayout::I400,
                DAV2D_PIXEL_LAYOUT_I420 => PixelLayout::I420,
                DAV2D_PIXEL_LAYOUT_I422 => PixelLayout::I422,
                DAV2D_PIXEL_LAYOUT_I444 => PixelLayout::I444,
                _ => unreachable!(),
            }
        }
    }

    pub fn color_primaries(&self) -> pixel::ColorPrimaries {
        unsafe {
            #[allow(non_upper_case_globals)]
            match (*self.pic.as_ref().seq_hdr).layout {
                _ => {
                    let ci = (*self.pic.as_ref()).ci;
                    if !ci.is_null() {
                        return map_color_primaries((*ci).color.pri);
                    }
                    pixel::ColorPrimaries::Unspecified
                }
            }
        }
    }

    pub fn transfer_characteristic(&self) -> pixel::TransferCharacteristic {
        unsafe {
            let ci = (*self.pic.as_ref()).ci;
            if !ci.is_null() {
                return map_transfer_characteristic((*ci).color.trc);
            }
            pixel::TransferCharacteristic::Unspecified
        }
    }

    pub fn matrix_coefficients(&self) -> pixel::MatrixCoefficients {
        unsafe {
            let ci = (*self.pic.as_ref()).ci;
            if !ci.is_null() {
                return map_matrix_coefficients((*ci).color.mtrx);
            }
            pixel::MatrixCoefficients::Unspecified
        }
    }

    pub fn color_range(&self) -> pixel::YUVRange {
        unsafe {
            let ci = (*self.pic.as_ref()).ci;
            if !ci.is_null() {
                match (*ci).color.range {
                    0 => pixel::YUVRange::Limited,
                    _ => pixel::YUVRange::Full,
                }
            } else {
                pixel::YUVRange::Limited
            }
        }
    }

    pub fn chroma_location(&self) -> pixel::ChromaLocation {
        unsafe {
            let ci = (*self.pic.as_ref()).ci;
            if !ci.is_null() {
                #[allow(non_upper_case_globals)]
                match (*ci).chr[0] as c_uint {
                    DAV2D_CHR_LEFT => pixel::ChromaLocation::Left,
                    DAV2D_CHR_CENTER => pixel::ChromaLocation::Center,
                    DAV2D_CHR_TOPLEFT => pixel::ChromaLocation::TopLeft,
                    DAV2D_CHR_TOP => pixel::ChromaLocation::Top,
                    DAV2D_CHR_BOTTOMLEFT => pixel::ChromaLocation::BottomLeft,
                    DAV2D_CHR_BOTTOM => pixel::ChromaLocation::Bottom,
                    _ => pixel::ChromaLocation::Center,
                }
            } else {
                pixel::ChromaLocation::Center
            }
        }
    }
}

fn map_color_primaries(pri: u8) -> pixel::ColorPrimaries {
    #[allow(non_upper_case_globals)]
    match pri as c_uint {
        DAV2D_COLOR_PRI_BT709 => pixel::ColorPrimaries::BT709,
        DAV2D_COLOR_PRI_UNKNOWN => pixel::ColorPrimaries::Unspecified,
        DAV2D_COLOR_PRI_BT470M => pixel::ColorPrimaries::BT470M,
        DAV2D_COLOR_PRI_BT470BG => pixel::ColorPrimaries::BT470BG,
        DAV2D_COLOR_PRI_BT601 => pixel::ColorPrimaries::BT470BG,
        DAV2D_COLOR_PRI_SMPTE240 => pixel::ColorPrimaries::ST240M,
        DAV2D_COLOR_PRI_FILM => pixel::ColorPrimaries::Film,
        DAV2D_COLOR_PRI_BT2020 => pixel::ColorPrimaries::BT2020,
        DAV2D_COLOR_PRI_XYZ => pixel::ColorPrimaries::ST428,
        DAV2D_COLOR_PRI_SMPTE431 => pixel::ColorPrimaries::P3DCI,
        DAV2D_COLOR_PRI_SMPTE432 => pixel::ColorPrimaries::P3Display,
        DAV2D_COLOR_PRI_EBU3213 => pixel::ColorPrimaries::Tech3213,
        23..=DAV2D_COLOR_PRI_RESERVED => pixel::ColorPrimaries::Unspecified,
        _ => pixel::ColorPrimaries::Unspecified,
    }
}

fn map_transfer_characteristic(trc: u8) -> pixel::TransferCharacteristic {
    #[allow(non_upper_case_globals)]
    match trc as c_uint {
        DAV2D_TRC_BT709 => pixel::TransferCharacteristic::BT1886,
        DAV2D_TRC_UNKNOWN => pixel::TransferCharacteristic::Unspecified,
        DAV2D_TRC_BT470M => pixel::TransferCharacteristic::BT470M,
        DAV2D_TRC_BT470BG => pixel::TransferCharacteristic::BT470BG,
        DAV2D_TRC_BT601 => pixel::TransferCharacteristic::ST170M,
        DAV2D_TRC_SMPTE240 => pixel::TransferCharacteristic::ST240M,
        DAV2D_TRC_LINEAR => pixel::TransferCharacteristic::Linear,
        DAV2D_TRC_LOG100 => pixel::TransferCharacteristic::Logarithmic100,
        DAV2D_TRC_LOG100_SQRT10 => pixel::TransferCharacteristic::Logarithmic316,
        DAV2D_TRC_IEC61966 => pixel::TransferCharacteristic::SRGB,
        DAV2D_TRC_BT1361 => pixel::TransferCharacteristic::BT1886,
        DAV2D_TRC_SRGB => pixel::TransferCharacteristic::SRGB,
        DAV2D_TRC_BT2020_10BIT => pixel::TransferCharacteristic::BT2020Ten,
        DAV2D_TRC_BT2020_12BIT => pixel::TransferCharacteristic::BT2020Twelve,
        DAV2D_TRC_SMPTE2084 => pixel::TransferCharacteristic::PerceptualQuantizer,
        DAV2D_TRC_SMPTE428 => pixel::TransferCharacteristic::ST428,
        DAV2D_TRC_HLG => pixel::TransferCharacteristic::HybridLogGamma,
        19..=DAV2D_TRC_RESERVED => pixel::TransferCharacteristic::Unspecified,
        _ => pixel::TransferCharacteristic::Unspecified,
    }
}

fn map_matrix_coefficients(mtrx: u8) -> pixel::MatrixCoefficients {
    #[allow(non_upper_case_globals)]
    match mtrx as c_uint {
        DAV2D_MC_IDENTITY => pixel::MatrixCoefficients::Identity,
        DAV2D_MC_BT709 => pixel::MatrixCoefficients::BT709,
        DAV2D_MC_UNKNOWN => pixel::MatrixCoefficients::Unspecified,
        DAV2D_MC_FCC => pixel::MatrixCoefficients::BT470M,
        DAV2D_MC_BT470BG => pixel::MatrixCoefficients::BT470BG,
        DAV2D_MC_BT601 => pixel::MatrixCoefficients::BT470BG,
        DAV2D_MC_SMPTE240 => pixel::MatrixCoefficients::ST240M,
        DAV2D_MC_SMPTE_YCGCO => pixel::MatrixCoefficients::YCgCo,
        DAV2D_MC_BT2020_NCL => pixel::MatrixCoefficients::BT2020NonConstantLuminance,
        DAV2D_MC_BT2020_CL => pixel::MatrixCoefficients::BT2020ConstantLuminance,
        DAV2D_MC_SMPTE2085 => pixel::MatrixCoefficients::ST2085,
        DAV2D_MC_CHROMAT_NCL => {
            pixel::MatrixCoefficients::ChromaticityDerivedNonConstantLuminance
        }
        DAV2D_MC_CHROMAT_CL => {
            pixel::MatrixCoefficients::ChromaticityDerivedConstantLuminance
        }
        DAV2D_MC_ICTCP => pixel::MatrixCoefficients::ICtCp,
        15..=DAV2D_MC_RESERVED => pixel::MatrixCoefficients::Unspecified,
        _ => pixel::MatrixCoefficients::Unspecified,
    }
}

pub const PICTURE_ALIGNMENT: usize = 64;

#[derive(Debug, PartialEq, Eq)]
pub struct PictureAllocation<D: Send + 'static> {
    pub data: [*mut u8; 3],
    pub stride: [isize; 2],
    pub allocator_data: D,
}

unsafe impl<D: Send + 'static> Send for PictureAllocation<D> {}

pub unsafe trait PictureAllocator: Send + Sync + 'static {
    type AllocatorData: Send + 'static;

    unsafe fn alloc_picture(
        &self,
        pic_params: &PictureParameters,
    ) -> Result<PictureAllocation<Self::AllocatorData>, Error>;

    unsafe fn release_picture(&self, allocation: PictureAllocation<Self::AllocatorData>);
}

#[derive(Debug)]
pub struct DefaultAllocator(());

unsafe impl PictureAllocator for DefaultAllocator {
    type AllocatorData = ();

    unsafe fn alloc_picture(
        &self,
        _pic_params: &PictureParameters,
    ) -> Result<PictureAllocation<Self::AllocatorData>, Error> {
        unimplemented!()
    }

    unsafe fn release_picture(&self, _allocation: PictureAllocation<Self::AllocatorData>) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Settings {
    dav2d_settings: Dav2dSettings,
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        unsafe {
            let mut dav2d_settings = mem::MaybeUninit::uninit();

            dav2d_default_settings(dav2d_settings.as_mut_ptr());

            Self {
                dav2d_settings: dav2d_settings.assume_init(),
            }
        }
    }

    pub fn set_n_threads(&mut self, n_threads: u32) {
        self.dav2d_settings.n_threads = n_threads as i32;
    }

    pub fn get_n_threads(&self) -> u32 {
        self.dav2d_settings.n_threads as u32
    }

    pub fn set_max_frame_delay(&mut self, max_frame_delay: u32) {
        self.dav2d_settings.max_frame_delay = max_frame_delay as i32;
    }

    pub fn get_max_frame_delay(&self) -> u32 {
        self.dav2d_settings.max_frame_delay as u32
    }

    pub fn set_apply_grain(&mut self, apply_grain: bool) {
        self.dav2d_settings.apply_grain = i32::from(apply_grain);
    }

    pub fn get_apply_grain(&self) -> bool {
        self.dav2d_settings.apply_grain != 0
    }

    pub fn set_operating_point(&mut self, operating_point: u32) {
        self.dav2d_settings.operating_point = operating_point as i32;
    }

    pub fn get_operating_point(&self) -> u32 {
        self.dav2d_settings.operating_point as u32
    }

    pub fn set_all_layers(&mut self, all_layers: bool) {
        self.dav2d_settings.all_layers = i32::from(all_layers);
    }

    pub fn get_all_layers(&self) -> bool {
        self.dav2d_settings.all_layers != 0
    }

    pub fn set_frame_size_limit(&mut self, frame_size_limit: u32) {
        self.dav2d_settings.frame_size_limit = frame_size_limit;
    }

    pub fn get_frame_size_limit(&self) -> u32 {
        self.dav2d_settings.frame_size_limit
    }

    pub fn set_strict_std_compliance(&mut self, strict_std_compliance: bool) {
        self.dav2d_settings.strict_std_compliance = i32::from(strict_std_compliance);
    }

    pub fn get_strict_std_compliance(&self) -> bool {
        self.dav2d_settings.strict_std_compliance != 0
    }

    pub fn set_output_invisible_frames(&mut self, output_invisible_frames: bool) {
        self.dav2d_settings.output_invisible_frames = i32::from(output_invisible_frames);
    }

    pub fn get_output_invisible_frames(&self) -> bool {
        self.dav2d_settings.output_invisible_frames != 0
    }

    pub fn set_inloop_filters(&mut self, inloop_filters: InloopFilterType) {
        self.dav2d_settings.inloop_filters = inloop_filters.bits();
    }

    pub fn get_inloop_filters(&self) -> InloopFilterType {
        InloopFilterType::from_bits_truncate(self.dav2d_settings.inloop_filters)
    }

    pub fn set_decode_frame_type(&mut self, decode_frame_type: DecodeFrameType) {
        self.dav2d_settings.decode_frame_type = decode_frame_type.into();
    }

    pub fn get_decode_frame_type(&self) -> DecodeFrameType {
        DecodeFrameType::try_from(self.dav2d_settings.decode_frame_type)
            .expect("Invalid Dav2dDecodeFrameType")
    }

    pub fn get_frame_delay(&self) -> Result<u32, Error> {
        unsafe {
            let ret = dav2d_get_frame_delay(&self.dav2d_settings);
            if ret < 0 {
                Err(Error::from(ret))
            } else {
                Ok(ret as u32)
            }
        }
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
    pub struct InloopFilterType: u32 {
        const DEBLOCK = DAV2D_INLOOPFILTER_DEBLOCK;
        const CDEF = DAV2D_INLOOPFILTER_CDEF;
        const CCSO = DAV2D_INLOOPFILTER_CCSO;
        const WIENER = DAV2D_INLOOPFILTER_WIENER;
        const GDF = DAV2D_INLOOPFILTER_GDF;
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum DecodeFrameType {
    #[default]
    All,
    Reference,
    Intra,
    Key,
}

impl TryFrom<u32> for DecodeFrameType {
    type Error = TryFromEnumError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            DAV2D_DECODEFRAMETYPE_ALL => Ok(DecodeFrameType::All),
            DAV2D_DECODEFRAMETYPE_REFERENCE => Ok(DecodeFrameType::Reference),
            DAV2D_DECODEFRAMETYPE_INTRA => Ok(DecodeFrameType::Intra),
            DAV2D_DECODEFRAMETYPE_KEY => Ok(DecodeFrameType::Key),
            _ => Err(TryFromEnumError(())),
        }
    }
}

impl From<DecodeFrameType> for u32 {
    fn from(v: DecodeFrameType) -> u32 {
        match v {
            DecodeFrameType::All => DAV2D_DECODEFRAMETYPE_ALL,
            DecodeFrameType::Reference => DAV2D_DECODEFRAMETYPE_REFERENCE,
            DecodeFrameType::Intra => DAV2D_DECODEFRAMETYPE_INTRA,
            DecodeFrameType::Key => DAV2D_DECODEFRAMETYPE_KEY,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromEnumError(());

impl std::fmt::Display for TryFromEnumError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("Invalid enum value")
    }
}

impl From<std::convert::Infallible> for TryFromEnumError {
    fn from(x: std::convert::Infallible) -> TryFromEnumError {
        match x {}
    }
}

impl std::error::Error for TryFromEnumError {}

#[derive(Debug)]
pub struct Decoder<A: PictureAllocator = DefaultAllocator> {
    dec: ptr::NonNull<Dav2dContext>,
    pending_data: Option<Dav2dData>,
    allocator: Option<Arc<A>>,
    settings: Dav2dSettings,
}

static_assertions::assert_impl_all!(Decoder<DefaultAllocator>: Send, Sync, Debug);

unsafe extern "C" fn release_wrapped_data<T: AsRef<[u8]>>(_data: *const u8, cookie: *mut c_void) {
    let buf = Box::from_raw(cookie as *mut T);
    drop(buf);
}

impl Decoder {
    pub fn with_settings(settings: &Settings) -> Result<Self, Error> {
        unsafe {
            let mut dec = mem::MaybeUninit::uninit();

            let ret = dav2d_open(dec.as_mut_ptr(), &settings.dav2d_settings);

            if ret < 0 {
                return Err(Error::from(ret));
            }

            Ok(Decoder {
                dec: ptr::NonNull::new(dec.assume_init()).unwrap(),
                pending_data: None,
                allocator: None,
                settings: settings.dav2d_settings,
            })
        }
    }

    pub fn new() -> Result<Self, Error> {
        Self::with_settings(&Settings::default())
    }
}

unsafe extern "C" fn alloc_picture_callback<A: PictureAllocator>(
    pic: *mut Dav2dPicture,
    cookie: *mut c_void,
) -> c_int {
    let allocator = &*(cookie as *const A);

    let pic_parameters = PictureParameters {
        pic: ptr::NonNull::new_unchecked(pic),
    };

    let res = allocator.alloc_picture(&pic_parameters);
    match res {
        Ok(allocation) => {
            (*pic).data[0] = allocation.data[0] as *mut c_void;
            (*pic).data[1] = allocation.data[1] as *mut c_void;
            (*pic).data[2] = allocation.data[2] as *mut c_void;
            (*pic).stride[0] = allocation.stride[0];
            (*pic).stride[1] = allocation.stride[1];
            (*pic).allocator_data =
                Box::into_raw(Box::new(allocation.allocator_data)) as *mut c_void;

            0
        }
        Err(err) => match err {
            Error::Again => DAV2D_ERR_AGAIN,
            Error::Eof => DAV2D_EOF,
            Error::InvalidArgument => DAV2D_ERR_INVAL,
            Error::NotEnoughMemory => DAV2D_ERR_NOMEM,
            Error::UnsupportedBitstream => DAV2D_ERR_NOPROTOOPT,
            Error::UnknownError(err) => {
                assert!(err < 0);
                err
            }
        },
    }
}

unsafe extern "C" fn release_picture_callback<A: PictureAllocator>(
    pic: *mut Dav2dPicture,
    cookie: *mut c_void,
) {
    let allocator = &*(cookie as *const A);
    let allocator_data = Box::from_raw((*pic).allocator_data as *mut A::AllocatorData);
    let allocation = PictureAllocation {
        data: [
            (*pic).data[0] as *mut u8,
            (*pic).data[1] as *mut u8,
            (*pic).data[2] as *mut u8,
        ],
        stride: (*pic).stride,
        allocator_data: *allocator_data,
    };
    allocator.release_picture(allocation);
}

impl<A: PictureAllocator> Decoder<A> {
    pub fn with_settings_and_allocator(settings: &Settings, allocator: A) -> Result<Self, Error> {
        unsafe {
            let allocator = Arc::new(allocator);

            let mut dec = mem::MaybeUninit::uninit();

            let settings = Dav2dSettings {
                allocator: Dav2dPicAllocator {
                    cookie: &*allocator as *const A as *mut c_void,
                    alloc_picture_callback: Some(alloc_picture_callback::<A>),
                    release_picture_callback: Some(release_picture_callback::<A>),
                },
                ..settings.dav2d_settings
            };
            let ret = dav2d_open(dec.as_mut_ptr(), &settings);

            if ret < 0 {
                return Err(Error::from(ret));
            }

            Ok(Decoder {
                dec: ptr::NonNull::new(dec.assume_init()).unwrap(),
                pending_data: None,
                allocator: Some(allocator),
                settings,
            })
        }
    }

    pub fn with_allocator(allocator: A) -> Result<Self, Error> {
        Self::with_settings_and_allocator(&Settings::default(), allocator)
    }
}

impl<A: PictureAllocator> Decoder<A> {
    pub fn flush(&mut self) {
        unsafe {
            dav2d_flush(self.dec.as_ptr());
            if let Some(mut pending_data) = self.pending_data.take() {
                dav2d_data_unref(&mut pending_data);
            }
        }
    }

    pub fn send_data<T: AsRef<[u8]> + Send + 'static>(
        &mut self,
        buf: T,
        offset: Option<i64>,
        timestamp: Option<i64>,
        duration: Option<i64>,
    ) -> Result<(), Error> {
        assert!(
            self.pending_data.is_none(),
            "Have pending data that needs to be handled first"
        );

        let buf = Box::new(buf);
        let slice = (*buf).as_ref();
        let len = slice.len();

        unsafe {
            let mut data: Dav2dData = mem::zeroed();
            let _ret = dav2d_data_wrap(
                &mut data,
                slice.as_ptr(),
                len,
                Some(release_wrapped_data::<T>),
                Box::into_raw(buf) as *mut c_void,
            );
            if let Some(offset) = offset {
                data.m.offset = offset;
            }
            if let Some(timestamp) = timestamp {
                data.m.timestamp = timestamp;
            }
            if let Some(duration) = duration {
                data.m.duration = duration;
            }

            let ret = dav2d_send_data(self.dec.as_ptr(), &mut data);
            if ret < 0 {
                let ret = Error::from(ret);

                if ret.is_again() {
                    self.pending_data = Some(data);
                } else {
                    dav2d_data_unref(&mut data);
                }

                return Err(ret);
            }

            if data.sz > 0 {
                self.pending_data = Some(data);
                return Err(Error::Again);
            }

            Ok(())
        }
    }

    pub fn send_pending_data(&mut self) -> Result<(), Error> {
        let mut data = match self.pending_data.take() {
            None => {
                return Ok(());
            }
            Some(data) => data,
        };

        unsafe {
            let ret = dav2d_send_data(self.dec.as_ptr(), &mut data);
            if ret < 0 {
                let ret = Error::from(ret);

                if ret.is_again() {
                    self.pending_data = Some(data);
                } else {
                    dav2d_data_unref(&mut data);
                }

                return Err(ret);
            }

            if data.sz > 0 {
                self.pending_data = Some(data);
                return Err(Error::Again);
            }

            Ok(())
        }
    }

    pub fn drain(&mut self) -> Result<(), Error> {
        unsafe {
            let ret = dav2d_send_data(self.dec.as_ptr(), ptr::null_mut());
            if ret < 0 && ret != DAV2D_EOF {
                return Err(Error::from(ret));
            }
            Ok(())
        }
    }

    pub fn get_picture(&mut self) -> Result<Picture<A>, Error> {
        unsafe {
            let mut pic: Dav2dPicture = mem::zeroed();
            let ret = dav2d_get_picture(self.dec.as_ptr(), &mut pic);

            if ret < 0 {
                Err(Error::from(ret))
            } else {
                let inner = InnerPicture { pic };
                Ok(Picture {
                    inner: Arc::new(inner),
                    allocator: self.allocator.clone(),
                })
            }
        }
    }

    pub fn get_frame_delay(&self) -> Result<u32, Error> {
        unsafe {
            let ret = dav2d_get_frame_delay(&self.settings);
            if ret < 0 {
                Err(Error::from(ret))
            } else {
                Ok(ret as u32)
            }
        }
    }
}

impl<A: PictureAllocator> Drop for Decoder<A> {
    fn drop(&mut self) {
        unsafe {
            if let Some(mut pending_data) = self.pending_data.take() {
                dav2d_data_unref(&mut pending_data);
            }
            let mut dec = self.dec.as_ptr();
            dav2d_close(&mut dec);
        };
    }
}

unsafe impl<A: PictureAllocator> Send for Decoder<A> {}
unsafe impl<A: PictureAllocator> Sync for Decoder<A> {}

#[derive(Debug)]
struct InnerPicture {
    pub pic: Dav2dPicture,
}

#[derive(Debug)]
pub struct Picture<A: PictureAllocator = DefaultAllocator> {
    inner: Arc<InnerPicture>,
    allocator: Option<Arc<A>>,
}

impl<A: PictureAllocator> Clone for Picture<A> {
    fn clone(&self) -> Self {
        Picture {
            inner: self.inner.clone(),
            allocator: self.allocator.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PixelLayout {
    I400,
    I420,
    I422,
    I444,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PlanarImageComponent {
    Y,
    U,
    V,
}

impl From<usize> for PlanarImageComponent {
    fn from(index: usize) -> Self {
        match index {
            0 => PlanarImageComponent::Y,
            1 => PlanarImageComponent::U,
            2 => PlanarImageComponent::V,
            _ => panic!("Invalid YUV index: {}", index),
        }
    }
}

impl From<PlanarImageComponent> for usize {
    fn from(component: PlanarImageComponent) -> Self {
        match component {
            PlanarImageComponent::Y => 0,
            PlanarImageComponent::U => 1,
            PlanarImageComponent::V => 2,
        }
    }
}

#[derive(Debug)]
pub struct Plane<A: PictureAllocator = DefaultAllocator>(Picture<A>, PlanarImageComponent);

impl<A: PictureAllocator> Clone for Plane<A> {
    fn clone(&self) -> Self {
        Plane(self.0.clone(), self.1)
    }
}

impl<A: PictureAllocator> AsRef<[u8]> for Plane<A> {
    fn as_ref(&self) -> &[u8] {
        let (stride, height) = self.0.plane_data_geometry(self.1);
        let data = self.0.plane_data_ptr(self.1) as *const u8;
        if stride == 0 || data.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(data, stride as usize * height as usize) }
    }
}

impl<A: PictureAllocator> std::ops::Deref for Plane<A> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

static_assertions::assert_impl_all!(Plane<DefaultAllocator>: Send, Sync, Clone, Debug);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BitsPerComponent(pub usize);

impl<A: PictureAllocator> Picture<A> {
    pub fn stride(&self, component: PlanarImageComponent) -> u32 {
        let s = match component {
            PlanarImageComponent::Y => 0,
            _ => 1,
        };
        self.inner.pic.stride[s] as u32
    }

    pub fn plane_data_ptr(&self, component: PlanarImageComponent) -> *mut c_void {
        let index: usize = component.into();
        self.inner.pic.data[index]
    }

    pub fn plane_data_geometry(&self, component: PlanarImageComponent) -> (u32, u32) {
        let height = match component {
            PlanarImageComponent::Y => self.height(),
            _ => match self.pixel_layout() {
                PixelLayout::I420 => (self.height() + 1) / 2,
                PixelLayout::I400 | PixelLayout::I422 | PixelLayout::I444 => self.height(),
            },
        };
        (self.stride(component), height)
    }

    pub fn plane(&self, component: PlanarImageComponent) -> Plane<A> {
        Plane(self.clone(), component)
    }

    pub fn bit_depth(&self) -> usize {
        self.inner.pic.p.bpc as usize
    }

    pub fn bits_per_component(&self) -> Option<BitsPerComponent> {
        unsafe {
            match (*self.inner.pic.seq_hdr).hbd {
                0 => Some(BitsPerComponent(8)),
                1 => Some(BitsPerComponent(10)),
                2 => Some(BitsPerComponent(12)),
                _ => None,
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.inner.pic.p.w as u32
    }

    pub fn height(&self) -> u32 {
        self.inner.pic.p.h as u32
    }

    pub fn pixel_layout(&self) -> PixelLayout {
        #[allow(non_upper_case_globals)]
        match self.inner.pic.p.layout {
            DAV2D_PIXEL_LAYOUT_I400 => PixelLayout::I400,
            DAV2D_PIXEL_LAYOUT_I420 => PixelLayout::I420,
            DAV2D_PIXEL_LAYOUT_I422 => PixelLayout::I422,
            DAV2D_PIXEL_LAYOUT_I444 => PixelLayout::I444,
            _ => unreachable!(),
        }
    }

    pub fn timestamp(&self) -> Option<i64> {
        let ts = self.inner.pic.m.timestamp;
        if ts == i64::MIN {
            None
        } else {
            Some(ts)
        }
    }

    pub fn duration(&self) -> i64 {
        self.inner.pic.m.duration
    }

    pub fn offset(&self) -> i64 {
        self.inner.pic.m.offset
    }

    pub fn color_primaries(&self) -> pixel::ColorPrimaries {
        unsafe {
            let ci = self.inner.pic.ci;
            if !ci.is_null() {
                return map_color_primaries((*ci).color.pri);
            }
            pixel::ColorPrimaries::Unspecified
        }
    }

    pub fn transfer_characteristic(&self) -> pixel::TransferCharacteristic {
        unsafe {
            let ci = self.inner.pic.ci;
            if !ci.is_null() {
                return map_transfer_characteristic((*ci).color.trc);
            }
            pixel::TransferCharacteristic::Unspecified
        }
    }

    pub fn matrix_coefficients(&self) -> pixel::MatrixCoefficients {
        unsafe {
            let ci = self.inner.pic.ci;
            if !ci.is_null() {
                return map_matrix_coefficients((*ci).color.mtrx);
            }
            pixel::MatrixCoefficients::Unspecified
        }
    }

    pub fn color_range(&self) -> pixel::YUVRange {
        unsafe {
            let ci = self.inner.pic.ci;
            if !ci.is_null() {
                match (*ci).color.range {
                    0 => pixel::YUVRange::Limited,
                    _ => pixel::YUVRange::Full,
                }
            } else {
                pixel::YUVRange::Limited
            }
        }
    }

    pub fn chroma_location(&self) -> pixel::ChromaLocation {
        unsafe {
            let ci = self.inner.pic.ci;
            if !ci.is_null() {
                #[allow(non_upper_case_globals)]
                match (*ci).chr[0] as c_uint {
                    DAV2D_CHR_LEFT => pixel::ChromaLocation::Left,
                    DAV2D_CHR_CENTER => pixel::ChromaLocation::Center,
                    DAV2D_CHR_TOPLEFT => pixel::ChromaLocation::TopLeft,
                    DAV2D_CHR_TOP => pixel::ChromaLocation::Top,
                    DAV2D_CHR_BOTTOMLEFT => pixel::ChromaLocation::BottomLeft,
                    DAV2D_CHR_BOTTOM => pixel::ChromaLocation::Bottom,
                    _ => pixel::ChromaLocation::Center,
                }
            } else {
                pixel::ChromaLocation::Center
            }
        }
    }

    pub fn allocator_data(&self) -> Option<&A::AllocatorData> {
        unsafe {
            if self.inner.pic.allocator_data.is_null() {
                None
            } else {
                Some(&*(self.inner.pic.allocator_data as *const A::AllocatorData))
            }
        }
    }

    pub fn content_light(&self) -> Option<ContentLightLevel> {
        unsafe {
            if self.inner.pic.content_light.is_null() {
                None
            } else {
                Some(ContentLightLevel {
                    max_content_light_level: (*self.inner.pic.content_light)
                        .max_content_light_level,
                    max_frame_average_light_level: (*self.inner.pic.content_light)
                        .max_frame_average_light_level,
                })
            }
        }
    }

    pub fn mastering_display(&self) -> Option<MasteringDisplay> {
        unsafe {
            if self.inner.pic.mastering_display.is_null() {
                None
            } else {
                Some(MasteringDisplay {
                    primaries: (*self.inner.pic.mastering_display).primaries,
                    white_point: (*self.inner.pic.mastering_display).white_point,
                    max_luminance: (*self.inner.pic.mastering_display).max_luminance,
                    min_luminance: (*self.inner.pic.mastering_display).min_luminance,
                })
            }
        }
    }

    pub fn film_grain_data(&self) -> Option<&Dav2dFilmGrainData> {
        unsafe {
            if self.inner.pic.fgm.is_null() {
                None
            } else {
                Some(&*self.inner.pic.fgm)
            }
        }
    }

    pub fn content_interpretation(&self) -> Option<&Dav2dContentInterpretation> {
        unsafe {
            if self.inner.pic.ci.is_null() {
                None
            } else {
                Some(&*self.inner.pic.ci)
            }
        }
    }
}

static_assertions::assert_impl_all!(Picture<DefaultAllocator>: Send, Sync, Clone, Debug);

unsafe impl Send for InnerPicture {}
unsafe impl Sync for InnerPicture {}

impl Drop for InnerPicture {
    fn drop(&mut self) {
        unsafe {
            dav2d_picture_unref(&mut self.pic);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContentLightLevel {
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MasteringDisplay {
    pub primaries: [[u16; 2usize]; 3usize],
    pub white_point: [u16; 2usize],
    pub max_luminance: u32,
    pub min_luminance: u32,
}
