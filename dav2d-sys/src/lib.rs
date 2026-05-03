use std::os::raw::{c_char, c_int, c_uint, c_void};

pub const DAV2D_OBU_SEQ_HDR: Dav2dObuType = 1;
pub const DAV2D_OBU_TD: Dav2dObuType = 2;
pub const DAV2D_OBU_MULTI_FRAME_HDR: Dav2dObuType = 3;
pub const DAV2D_OBU_CLOSED_LOOP_KF: Dav2dObuType = 4;
pub const DAV2D_OBU_OPEN_LOOP_KF: Dav2dObuType = 5;
pub const DAV2D_OBU_LEADING_TILE_GRP: Dav2dObuType = 6;
pub const DAV2D_OBU_TILE_GRP: Dav2dObuType = 7;
pub const DAV2D_OBU_METADATA: Dav2dObuType = 8;
pub const DAV2D_OBU_METADATA_GRP: Dav2dObuType = 9;
pub const DAV2D_OBU_SWITCH: Dav2dObuType = 10;
pub const DAV2D_OBU_LEADING_SEF: Dav2dObuType = 11;
pub const DAV2D_OBU_SEF: Dav2dObuType = 12;
pub const DAV2D_OBU_LEADING_TIP: Dav2dObuType = 13;
pub const DAV2D_OBU_TIP: Dav2dObuType = 14;
pub const DAV2D_OBU_BUF_RM_TIMING: Dav2dObuType = 15;
pub const DAV2D_OBU_LAYER_CFG_REC: Dav2dObuType = 16;
pub const DAV2D_OBU_ATLAS_SEG: Dav2dObuType = 17;
pub const DAV2D_OBU_OP_PT_SET: Dav2dObuType = 18;
pub const DAV2D_OBU_BRIDGE: Dav2dObuType = 19;
pub const DAV2D_OBU_MSDO: Dav2dObuType = 20;
pub const DAV2D_OBU_RAS: Dav2dObuType = 21;
pub const DAV2D_OBU_QM: Dav2dObuType = 22;
pub const DAV2D_OBU_FGM: Dav2dObuType = 23;
pub const DAV2D_OBU_CONTENT_INTERP: Dav2dObuType = 24;
pub const DAV2D_OBU_PADDING: Dav2dObuType = 25;
pub type Dav2dObuType = c_uint;

pub const DAV2D_TX_4X4_ONLY: Dav2dTxfmMode = 0;
pub const DAV2D_TX_LARGEST: Dav2dTxfmMode = 1;
pub const DAV2D_TX_SWITCHABLE: Dav2dTxfmMode = 2;
pub const DAV2D_N_TX_MODES: Dav2dTxfmMode = 3;
pub type Dav2dTxfmMode = c_uint;

pub const DAV2D_FILTER_8TAP_REGULAR: Dav2dFilterMode = 0;
pub const DAV2D_FILTER_8TAP_SMOOTH: Dav2dFilterMode = 1;
pub const DAV2D_FILTER_8TAP_SHARP: Dav2dFilterMode = 2;
pub const DAV2D_N_SWITCHABLE_FILTERS: Dav2dFilterMode = 3;
pub const DAV2D_FILTER_BILINEAR: Dav2dFilterMode = 3;
pub const DAV2D_N_FILTERS: Dav2dFilterMode = 4;
pub const DAV2D_FILTER_SWITCHABLE: Dav2dFilterMode = 4;
pub type Dav2dFilterMode = c_uint;

pub const DAV2D_OFF: Dav2dAdaptiveBoolean = 0;
pub const DAV2D_ON: Dav2dAdaptiveBoolean = 1;
pub const DAV2D_ADAPTIVE: Dav2dAdaptiveBoolean = 2;
pub type Dav2dAdaptiveBoolean = c_uint;

pub const DAV2D_RESTORATION_NONE: Dav2dRestorationType = 0;
pub const DAV2D_RESTORATION_PC_WIENER: Dav2dRestorationType = 1;
pub const DAV2D_RESTORATION_NS_WIENER: Dav2dRestorationType = 2;
pub const DAV2D_RESTORATION_SWITCHABLE: Dav2dRestorationType = 3;
pub type Dav2dRestorationType = c_uint;

pub const DAV2D_WM_TYPE_INVALID: Dav2dWarpedMotionType = -1;
pub const DAV2D_WM_TYPE_IDENTITY: Dav2dWarpedMotionType = 0;
pub const DAV2D_WM_TYPE_TRANSLATION: Dav2dWarpedMotionType = 1;
pub const DAV2D_WM_TYPE_ROT_ZOOM: Dav2dWarpedMotionType = 2;
pub const DAV2D_WM_TYPE_AFFINE: Dav2dWarpedMotionType = 3;
pub type Dav2dWarpedMotionType = c_int;

pub const DAV2D_PIXEL_LAYOUT_I400: Dav2dPixelLayout = 0;
pub const DAV2D_PIXEL_LAYOUT_I420: Dav2dPixelLayout = 1;
pub const DAV2D_PIXEL_LAYOUT_I422: Dav2dPixelLayout = 2;
pub const DAV2D_PIXEL_LAYOUT_I444: Dav2dPixelLayout = 3;
pub type Dav2dPixelLayout = c_uint;

pub const DAV2D_FRAME_TYPE_KEY: Dav2dFrameType = 0;
pub const DAV2D_FRAME_TYPE_INTER: Dav2dFrameType = 1;
pub const DAV2D_FRAME_TYPE_INTRA: Dav2dFrameType = 2;
pub const DAV2D_FRAME_TYPE_SWITCH: Dav2dFrameType = 3;
pub type Dav2dFrameType = c_uint;

pub const DAV2D_COLOR_DESC_EXPLICIT: Dav2dColorDescription = 0;
pub const DAV2D_COLOR_DESC_BT709SDR: Dav2dColorDescription = 1;
pub const DAV2D_COLOR_DESC_BT2100PQ: Dav2dColorDescription = 2;
pub const DAV2D_COLOR_DESC_BT2100HLG: Dav2dColorDescription = 3;
pub const DAV2D_COLOR_DESC_SRGB: Dav2dColorDescription = 4;
pub const DAV2D_COLOR_DESC_SRGBSYCC: Dav2dColorDescription = 5;
pub type Dav2dColorDescription = c_uint;

pub const DAV2D_COLOR_PRI_BT709: Dav2dColorPrimaries = 1;
pub const DAV2D_COLOR_PRI_UNKNOWN: Dav2dColorPrimaries = 2;
pub const DAV2D_COLOR_PRI_BT470M: Dav2dColorPrimaries = 4;
pub const DAV2D_COLOR_PRI_BT470BG: Dav2dColorPrimaries = 5;
pub const DAV2D_COLOR_PRI_BT601: Dav2dColorPrimaries = 6;
pub const DAV2D_COLOR_PRI_SMPTE240: Dav2dColorPrimaries = 7;
pub const DAV2D_COLOR_PRI_FILM: Dav2dColorPrimaries = 8;
pub const DAV2D_COLOR_PRI_BT2020: Dav2dColorPrimaries = 9;
pub const DAV2D_COLOR_PRI_XYZ: Dav2dColorPrimaries = 10;
pub const DAV2D_COLOR_PRI_SMPTE431: Dav2dColorPrimaries = 11;
pub const DAV2D_COLOR_PRI_SMPTE432: Dav2dColorPrimaries = 12;
pub const DAV2D_COLOR_PRI_EBU3213: Dav2dColorPrimaries = 22;
pub const DAV2D_COLOR_PRI_RESERVED: Dav2dColorPrimaries = 255;
pub type Dav2dColorPrimaries = c_uint;

pub const DAV2D_TRC_BT709: Dav2dTransferCharacteristics = 1;
pub const DAV2D_TRC_UNKNOWN: Dav2dTransferCharacteristics = 2;
pub const DAV2D_TRC_BT470M: Dav2dTransferCharacteristics = 4;
pub const DAV2D_TRC_BT470BG: Dav2dTransferCharacteristics = 5;
pub const DAV2D_TRC_BT601: Dav2dTransferCharacteristics = 6;
pub const DAV2D_TRC_SMPTE240: Dav2dTransferCharacteristics = 7;
pub const DAV2D_TRC_LINEAR: Dav2dTransferCharacteristics = 8;
pub const DAV2D_TRC_LOG100: Dav2dTransferCharacteristics = 9;
pub const DAV2D_TRC_LOG100_SQRT10: Dav2dTransferCharacteristics = 10;
pub const DAV2D_TRC_IEC61966: Dav2dTransferCharacteristics = 11;
pub const DAV2D_TRC_BT1361: Dav2dTransferCharacteristics = 12;
pub const DAV2D_TRC_SRGB: Dav2dTransferCharacteristics = 13;
pub const DAV2D_TRC_BT2020_10BIT: Dav2dTransferCharacteristics = 14;
pub const DAV2D_TRC_BT2020_12BIT: Dav2dTransferCharacteristics = 15;
pub const DAV2D_TRC_SMPTE2084: Dav2dTransferCharacteristics = 16;
pub const DAV2D_TRC_SMPTE428: Dav2dTransferCharacteristics = 17;
pub const DAV2D_TRC_HLG: Dav2dTransferCharacteristics = 18;
pub const DAV2D_TRC_RESERVED: Dav2dTransferCharacteristics = 255;
pub type Dav2dTransferCharacteristics = c_uint;

pub const DAV2D_MC_IDENTITY: Dav2dMatrixCoefficients = 0;
pub const DAV2D_MC_BT709: Dav2dMatrixCoefficients = 1;
pub const DAV2D_MC_UNKNOWN: Dav2dMatrixCoefficients = 2;
pub const DAV2D_MC_FCC: Dav2dMatrixCoefficients = 4;
pub const DAV2D_MC_BT470BG: Dav2dMatrixCoefficients = 5;
pub const DAV2D_MC_BT601: Dav2dMatrixCoefficients = 6;
pub const DAV2D_MC_SMPTE240: Dav2dMatrixCoefficients = 7;
pub const DAV2D_MC_SMPTE_YCGCO: Dav2dMatrixCoefficients = 8;
pub const DAV2D_MC_BT2020_NCL: Dav2dMatrixCoefficients = 9;
pub const DAV2D_MC_BT2020_CL: Dav2dMatrixCoefficients = 10;
pub const DAV2D_MC_SMPTE2085: Dav2dMatrixCoefficients = 11;
pub const DAV2D_MC_CHROMAT_NCL: Dav2dMatrixCoefficients = 12;
pub const DAV2D_MC_CHROMAT_CL: Dav2dMatrixCoefficients = 13;
pub const DAV2D_MC_ICTCP: Dav2dMatrixCoefficients = 14;
pub const DAV2D_MC_IPT_C2: Dav2dMatrixCoefficients = 15;
pub const DAV2D_MC_YCGCO_RE: Dav2dMatrixCoefficients = 16;
pub const DAV2D_MC_YCGCO_RO: Dav2dMatrixCoefficients = 17;
pub const DAV2D_MC_RESERVED: Dav2dMatrixCoefficients = 255;
pub type Dav2dMatrixCoefficients = c_uint;

pub const DAV2D_CHR_LEFT: Dav2dChromaSamplePosition = 0;
pub const DAV2D_CHR_CENTER: Dav2dChromaSamplePosition = 1;
pub const DAV2D_CHR_TOPLEFT: Dav2dChromaSamplePosition = 2;
pub const DAV2D_CHR_TOP: Dav2dChromaSamplePosition = 3;
pub const DAV2D_CHR_BOTTOMLEFT: Dav2dChromaSamplePosition = 4;
pub const DAV2D_CHR_BOTTOM: Dav2dChromaSamplePosition = 5;
pub const DAV2D_CHR_UNKNOWN: Dav2dChromaSamplePosition = 6;
pub type Dav2dChromaSamplePosition = c_uint;

pub const DAV2D_SAR_UNKNOWN: Dav2dAspectRatio = 0;
pub const DAV2D_SAR_1_1: Dav2dAspectRatio = 1;
pub const DAV2D_SAR_12_11: Dav2dAspectRatio = 2;
pub const DAV2D_SAR_10_11: Dav2dAspectRatio = 3;
pub const DAV2D_SAR_16_11: Dav2dAspectRatio = 4;
pub const DAV2D_SAR_40_33: Dav2dAspectRatio = 5;
pub const DAV2D_SAR_24_11: Dav2dAspectRatio = 6;
pub const DAV2D_SAR_20_11: Dav2dAspectRatio = 7;
pub const DAV2D_SAR_32_11: Dav2dAspectRatio = 8;
pub const DAV2D_SAR_80_33: Dav2dAspectRatio = 9;
pub const DAV2D_SAR_18_11: Dav2dAspectRatio = 10;
pub const DAV2D_SAR_15_11: Dav2dAspectRatio = 11;
pub const DAV2D_SAR_64_33: Dav2dAspectRatio = 12;
pub const DAV2D_SAR_160_99: Dav2dAspectRatio = 13;
pub const DAV2D_SAR_4_3: Dav2dAspectRatio = 14;
pub const DAV2D_SAR_3_2: Dav2dAspectRatio = 15;
pub const DAV2D_SAR_2_1: Dav2dAspectRatio = 16;
pub const DAV2D_SAR_EXPLICIT: Dav2dAspectRatio = 255;
pub type Dav2dAspectRatio = c_uint;

pub const DAV2D_SCAN_TYPE_UNKNOWN: Dav2dScanType = 0;
pub const DAV2D_SCAN_TYPE_PROGRESSIVE: Dav2dScanType = 1;
pub const DAV2D_SCAN_TYPE_INTERLACE: Dav2dScanType = 2;
pub const DAV2D_SCAN_TYPE_INTERLACE_COMPLEMENTARY: Dav2dScanType = 3;
pub type Dav2dScanType = c_uint;

pub const DAV2D_INLOOPFILTER_DEBLOCK: Dav2dInloopFilterType = 1;
pub const DAV2D_INLOOPFILTER_CDEF: Dav2dInloopFilterType = 2;
pub const DAV2D_INLOOPFILTER_CCSO: Dav2dInloopFilterType = 4;
pub const DAV2D_INLOOPFILTER_WIENER: Dav2dInloopFilterType = 8;
pub const DAV2D_INLOOPFILTER_GDF: Dav2dInloopFilterType = 16;
pub const DAV2D_INLOOPFILTER_ALL: Dav2dInloopFilterType = 31;
pub type Dav2dInloopFilterType = c_uint;

pub const DAV2D_EVENT_FLAG_NEW_SEQUENCE: Dav2dEventFlags = 1;
pub const DAV2D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav2dEventFlags = 2;
pub type Dav2dEventFlags = c_uint;

pub const DAV2D_MAX_THREADS: c_int = 256;
pub const DAV2D_MAX_FRAME_DELAY: c_int = 256;

pub const DAV2D_MAX_CDEF_STRENGTHS: usize = 8;
pub const DAV2D_MAX_OPERATING_POINTS: usize = 64;
pub const DAV2D_MAX_TILE_COLS: usize = 64;
pub const DAV2D_MAX_TILE_ROWS: usize = 64;
pub const DAV2D_MAX_SEGMENTS: usize = 16;
pub const DAV2D_NUM_REF_FRAMES: usize = 8;
pub const DAV2D_PRIMARY_REF_NONE: usize = 7;
pub const DAV2D_REFS_PER_FRAME: usize = 7;
pub const DAV2D_TOTAL_REFS_PER_FRAME: usize = DAV2D_REFS_PER_FRAME + 1;

pub const DAV2D_DECODEFRAMETYPE_ALL: Dav2dDecodeFrameType = 0;
pub const DAV2D_DECODEFRAMETYPE_REFERENCE: Dav2dDecodeFrameType = 1;
pub const DAV2D_DECODEFRAMETYPE_INTRA: Dav2dDecodeFrameType = 2;
pub const DAV2D_DECODEFRAMETYPE_KEY: Dav2dDecodeFrameType = 3;
pub type Dav2dDecodeFrameType = c_uint;

pub const fn dav2d_err(errno: c_int) -> c_int {
    if libc::EPERM < 0 {
        errno
    } else {
        -errno
    }
}

pub const DAV2D_ERR_AGAIN: c_int = dav2d_err(libc::EAGAIN);
pub const DAV2D_ERR_INVAL: c_int = dav2d_err(libc::EINVAL);
pub const DAV2D_ERR_NOMEM: c_int = dav2d_err(libc::ENOMEM);
pub const DAV2D_ERR_NOPROTOOPT: c_int = dav2d_err(libc::ENOPROTOOPT);
pub const DAV2D_ERR_NOENT: c_int = dav2d_err(libc::ENOENT);
pub const DAV2D_EOF: c_int = -('E' as c_int | ('O' as c_int) << 8 | ('F' as c_int) << 16);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dUserData {
    pub data: *const u8,
    pub ref_: *mut Dav2dRef,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: i64,
    pub size: usize,
    pub user_data: Dav2dUserData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav2dWarpedMotionParams {
    pub type_: Dav2dWarpedMotionType,
    pub matrix: [i32; 6usize],
    pub u: Dav2dWarpedMotionParamsU,
    pub affine: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Dav2dWarpedMotionParamsU {
    pub p: Dav2dWarpedMotionParamsUP,
    pub abcd: [i16; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dWarpedMotionParamsUP {
    pub alpha: i16,
    pub beta: i16,
    pub gamma: i16,
    pub delta: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dContentLightLevel {
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dMasteringDisplay {
    pub primaries: [[u16; 2usize]; 3usize],
    pub white_point: [u16; 2usize],
    pub max_luminance: u32,
    pub min_luminance: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload_size: usize,
    pub payload: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dSegmentationDataSet {
    pub delta_q: [i16; DAV2D_MAX_SEGMENTS],
    pub delta_q_mask: u16,
    pub skip_mask: u16,
    pub globalmv_mask: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dContentInterpretation {
    pub scan_type: u8,
    pub color_description_present: u8,
    pub chroma_sample_position_present: u8,
    pub aspect_ratio_info_present: u8,
    pub timing_info_present: u8,
    pub extension_present: u8,
    pub chr: [u8; 2usize],
    pub color: Dav2dContentInterpretationColor,
    pub sar: Dav2dContentInterpretationSar,
    pub timing: Dav2dContentInterpretationTiming,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dContentInterpretationColor {
    pub type_: u8,
    pub pri: u8,
    pub trc: u8,
    pub mtrx: u8,
    pub range: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dContentInterpretationSar {
    pub type_: u8,
    pub w: u32,
    pub h: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dContentInterpretationTiming {
    pub num_units_in_display_tick: u32,
    pub time_scale: u32,
    pub equal_elemental_interval: u8,
    pub num_ticks_per_elemental_duration: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dTileInfo {
    pub uniform: u8,
    pub min_log2_cols: u8,
    pub max_log2_cols: u8,
    pub log2_cols: u8,
    pub cols: u8,
    pub min_log2_rows: u8,
    pub max_log2_rows: u8,
    pub log2_rows: u8,
    pub rows: u8,
    pub col_start_sb: [u16; DAV2D_MAX_TILE_COLS + 1],
    pub row_start_sb: [u16; DAV2D_MAX_TILE_ROWS + 1],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dSequenceHeader {
    pub id: u8,
    pub profile: u8,
    pub reduced_still_picture_header: u8,
    pub level: u8,
    pub tier: u8,
    pub layout: u8,
    pub ss_hor: u8,
    pub ss_ver: u8,
    pub hbd: u8,
    pub lcr_id: u8,
    pub still_picture: u8,
    pub max_tlayer_id: u8,
    pub max_mlayer_id: u8,
    pub monotonic: u8,
    pub max_width: c_int,
    pub max_height: c_int,
    pub width_n_bits: u8,
    pub height_n_bits: u8,
    pub crop: Dav2dSequenceHeaderCrop,
    pub max_display_model_info_present: u8,
    pub max_initial_display_delay: u8,
    pub decoder_model_info_present: u8,
    pub max_decoder_model_present: u8,
    pub num_units_in_decoding_tick: u32,
    pub max_decoder_buffer_delay: u32,
    pub max_encoder_buffer_delay: u32,
    pub max_low_delay_mode: u8,
    pub tlayer_dependency_present: u8,
    pub mlayer_dependency_present: u8,
    pub tlayer_dependencies: [u8; 8],
    pub mlayer_dependencies: [u8; 8],
    pub sb128: u8,
    pub sdp: u8,
    pub ext_sdp: u8,
    pub ext_partitions: u8,
    pub uneven_4way_partitions: u8,
    pub max_pb_aspect_ratio_log2: u8,
    pub segmentation: Dav2dSequenceHeaderSegmentation,
    pub intra_dip: u8,
    pub intra_edge_filter: u8,
    pub mrls: u8,
    pub cfl: u8,
    pub cfl_ds_filter_index: u8,
    pub mhccp: u8,
    pub ibp: u8,
    pub motion_modes: u8,
    pub frame_motion_modes_present: u8,
    pub six_param_warp_delta: u8,
    pub masked_compound: u8,
    pub ref_frame_mvs: u8,
    pub reduced_ref_frame_mvs_mode: u8,
    pub order_hint_n_bits: u8,
    pub refmv_bank: u8,
    pub drl_reorder: u8,
    pub explicit_ref_frame_map: u8,
    pub ref_frames: u8,
    pub ref_frames_log2: u8,
    pub number_of_bits_for_lt_frame_id: u8,
    pub def_max_drl_bits: u8,
    pub allow_frame_max_drl_bits: u8,
    pub def_max_bvp_drl_bits: u8,
    pub allow_max_bvp_drl_bits: u8,
    pub num_same_ref_comp: u8,
    pub tip: u8,
    pub tip_hole_fill: u8,
    pub mv_traj: u8,
    pub bawp: u8,
    pub cwp: u8,
    pub imp_msk_bld: u8,
    pub db_sub_pu: u8,
    pub tip_explicit_qp: u8,
    pub opfl_refine: u8,
    pub refine_mv: u8,
    pub tip_refine_mv: u8,
    pub bru: u8,
    pub adaptive_mvd: u8,
    pub mvd_sign_derive: u8,
    pub flex_mvres: u8,
    pub global_motion: u8,
    pub short_refresh_frame_flags: u8,
    pub screen_content_tools: u8,
    pub force_integer_mv: u8,
    pub fsc: u8,
    pub idtx_intra: u8,
    pub ist: [u8; 2],
    pub chroma_dctonly: u8,
    pub inter_ddt: u8,
    pub reduced_tx_part_set: u8,
    pub cctx: u8,
    pub tcq: u8,
    pub parity_hiding: u8,
    pub avg_cdf: u8,
    pub avg_cdf_type: u8,
    pub disable_loopfilters_across_tiles: u8,
    pub cdef: u8,
    pub gdf: u8,
    pub gdf_unit_matches_sbsz: u8,
    pub restoration: u8,
    pub rst_disable_mask: [u8; 2],
    pub ccso: u8,
    pub ccso_unit_matches_sbsz: u8,
    pub cdef_on_skiptx: u8,
    pub df_par_bits: u8,
    pub separate_uv_delta_q: u8,
    pub equal_ac_dc_q: u8,
    pub base_ydc_dq: i8,
    pub ydc_dq_enabled: i8,
    pub base_uvdc_dq: u8,
    pub uvdc_dq_enabled: u8,
    pub base_uvac_dq: u8,
    pub uvac_dq_enabled: u8,
    pub tiling: Dav2dSequenceHeaderTiling,
    pub film_grain_present: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dSequenceHeaderCrop {
    pub enabled: u8,
    pub left: c_uint,
    pub right: c_uint,
    pub top: c_uint,
    pub bottom: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dSequenceHeaderSegmentation {
    pub ext: u8,
    pub info_present: u8,
    pub adaptive: u8,
    pub d: Dav2dSegmentationDataSet,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dSequenceHeaderTiling {
    pub present: u8,
    pub t: Dav2dTileInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFilmGrainData {
    pub chroma_scaling_from_luma: c_int,
    pub num_points: [c_int; 3],
    pub points: [[[u8; 2]; 14]; 3],
    pub scaling_shift: c_int,
    pub ar_coeff_lag: c_int,
    pub ar_coeffs: [[i8; 28]; 3],
    pub ar_coeff_shift: u64,
    pub grain_scale_shift: c_int,
    pub uv_mult: [c_int; 2],
    pub uv_luma_mult: [c_int; 2],
    pub uv_offset: [c_int; 2],
    pub overlap_flag: c_int,
    pub clip_to_restricted_range: c_int,
    pub mc_identity: c_int,
    pub block_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav2dFrameHeader {
    pub id: u8,
    pub frame_type: Dav2dFrameType,
    pub width: c_int,
    pub height: c_int,
    pub frame_offset: u8,
    pub tlayer_id: u8,
    pub mlayer_id: u8,
    pub xlayer_id: u8,
    pub show_existing_frame: u8,
    pub existing_frame_idx: i8,
    pub ltr_id: i8,
    pub frame_presentation_delay: u32,
    pub show_immediate: u8,
    pub show_implicit: u8,
    pub cross_frame_context: u8,
    pub disable_cdf_update: u8,
    pub allow_screen_content_tools: u8,
    pub force_integer_mv: u8,
    pub frame_size_override: u8,
    pub primary_ref_signaled: u8,
    pub primary_ref_frame: u8,
    pub secondary_ref_frame: u8,
    pub n_ref_frames: u8,
    pub refresh_frame_flags: u8,
    pub allow_intrabc: u8,
    pub allow_global_intrabc: u8,
    pub allow_local_intrabc: u8,
    pub max_bvp_drl_bits: u8,
    pub max_drl_bits: u8,
    pub refidx: [i8; DAV2D_REFS_PER_FRAME],
    pub has_future_refs: u8,
    pub has_past_refs: u8,
    pub has_bothside_refs: u8,
    pub mv_precision: u8,
    pub subpel_filter_mode: Dav2dFilterMode,
    pub motion_modes: u8,
    pub use_ref_frame_mvs: u8,
    pub tmvp_sample_step: u8,
    pub opfl_refine_type: u8,
    pub tip: Dav2dFrameHeaderTip,
    pub sb128: u8,
    pub tiling: Dav2dFrameHeaderTiling,
    pub quant: Dav2dFrameHeaderQuant,
    pub segmentation: Dav2dFrameHeaderSegmentation,
    pub delta: Dav2dFrameHeaderDelta,
    pub all_lossless: u8,
    pub any_lossless: u8,
    pub tcq: u8,
    pub parity_hiding: u8,
    pub deblock: Dav2dFrameHeaderDeblock,
    pub gdf: Dav2dFrameHeaderGdf,
    pub cdef: Dav2dFrameHeaderCdef,
    pub restoration: Dav2dFrameHeaderRestoration,
    pub ccso: Dav2dFrameHeaderCcso,
    pub txfm_mode: Dav2dTxfmMode,
    pub switchable_comp_refs: u8,
    pub skip_mode_enabled: u8,
    pub bawp: u8,
    pub warp_motion: u8,
    pub reduced_txtp_set: u8,
    pub gmv: Dav2dFrameHeaderGmv,
    pub film_grain: Dav2dFrameHeaderFilmGrain,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderTip {
    pub frame_mode: u8,
    pub hole_fill: u8,
    pub global_wtd_idx: u8,
    pub apply_filter: u8,
    pub gmv: Dav2dFrameHeaderTipGmv,
    pub subpel_filter: u8,
    pub ref_: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderTipGmv {
    pub y: i8,
    pub x: i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderTiling {
    pub t: Dav2dTileInfo,
    pub n_bytes: u8,
    pub update: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderQuant {
    pub yac: u16,
    pub ydc_delta: i8,
    pub udc_delta: i8,
    pub uac_delta: i8,
    pub vdc_delta: i8,
    pub vac_delta: i8,
    pub qm: Dav2dFrameHeaderQuantQm,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderQuantQm {
    pub enabled: u8,
    pub num: u8,
    pub y: [u8; 4],
    pub u: [u8; 4],
    pub v: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderSegmentation {
    pub enabled: u8,
    pub update_map: u8,
    pub temporal: u8,
    pub d: Dav2dSegmentationDataSet,
    pub preskip: u8,
    pub last_active_segid: i8,
    pub lossless: [u8; DAV2D_MAX_SEGMENTS],
    pub qidx: [u8; DAV2D_MAX_SEGMENTS],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderDelta {
    pub q: Dav2dFrameHeaderDeltaQ,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderDeltaQ {
    pub present: u8,
    pub res_log2: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderDeblock {
    pub sub_pu: u8,
    pub level_y: [u8; 2],
    pub level_u: u8,
    pub level_v: u8,
    pub delta_q_y: [i8; 2],
    pub delta_q_u: i8,
    pub delta_q_v: i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderGdf {
    pub enabled: Dav2dAdaptiveBoolean,
    pub qp_idx: u8,
    pub scale: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderCdef {
    pub enabled: u8,
    pub damping: u8,
    pub n_strengths: u8,
    pub on_skiptx: u8,
    pub y_strength: [u8; DAV2D_MAX_CDEF_STRENGTHS],
    pub uv_strength: [u8; DAV2D_MAX_CDEF_STRENGTHS],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dNSWienerPlane {
    pub frame_filters_on: u8,
    pub num_classes_idx: u8,
    pub num_classes: u8,
    pub temporal: u8,
    pub refidx: u8,
    pub filter: [[i8; 18]; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav2dRestorationPlane {
    pub type_: u8,
    pub ns: Dav2dNSWienerPlane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav2dFrameHeaderRestoration {
    pub p: [Dav2dRestorationPlane; 3],
    pub unit_size: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dCcsoPlane {
    pub enabled: u8,
    pub reuse: u8,
    pub sb_reuse: u8,
    pub refidx: u8,
    pub bo_only: u8,
    pub scale_idx: u8,
    pub quant_idx: u8,
    pub ext_filter_support: u8,
    pub edge_clf: u8,
    pub max_band_log2: u8,
    pub filter_off: [u8; 64],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderCcso {
    pub enabled: u8,
    pub p: [Dav2dCcsoPlane; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav2dFrameHeaderGmv {
    pub ref_: u8,
    pub refref: u8,
    pub m: [Dav2dWarpedMotionParams; DAV2D_REFS_PER_FRAME],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dFrameHeaderFilmGrain {
    pub present: u8,
    pub id: u8,
    pub seed: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav2dPixelLayout,
    pub bpc: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dPicture {
    pub seq_hdr: *mut Dav2dSequenceHeader,
    pub frame_hdr: *mut Dav2dFrameHeader,
    pub data: [*mut c_void; 3usize],
    pub stride: [isize; 2usize],
    pub p: Dav2dPictureParameters,
    pub m: Dav2dDataProps,
    pub content_light: *mut Dav2dContentLightLevel,
    pub mastering_display: *mut Dav2dMasteringDisplay,
    pub itut_t35: *mut Dav2dITUTT35,
    pub fgm: *mut Dav2dFilmGrainData,
    pub ci: *mut Dav2dContentInterpretation,
    pub n_itut_t35: usize,
    pub reserved: [usize; 4usize],
    pub frame_hdr_ref: *mut Dav2dRef,
    pub seq_hdr_ref: *mut Dav2dRef,
    pub content_light_ref: *mut Dav2dRef,
    pub mastering_display_ref: *mut Dav2dRef,
    pub itut_t35_ref: *mut Dav2dRef,
    pub fgm_ref: *mut Dav2dRef,
    pub ci_ref: *mut Dav2dRef,
    pub reserved_ref: [usize; 4usize],
    pub ref_: *mut Dav2dRef,
    pub allocator_data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dPicAllocator {
    pub cookie: *mut c_void,
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav2dPicture, cookie: *mut c_void) -> c_int>,
    pub release_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav2dPicture, cookie: *mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dData {
    pub data: *const u8,
    pub sz: usize,
    pub ref_: *mut Dav2dRef,
    pub m: Dav2dDataProps,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Dav2dContext(c_void);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dLogger {
    pub cookie: *mut c_void,
    pub callback: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav2dSettings {
    pub n_threads: c_int,
    pub max_frame_delay: c_int,
    pub apply_grain: c_int,
    pub operating_point: c_int,
    pub all_layers: c_int,
    pub frame_size_limit: c_uint,
    pub allocator: Dav2dPicAllocator,
    pub logger: Dav2dLogger,
    pub strict_std_compliance: c_int,
    pub output_invisible_frames: c_int,
    pub inloop_filters: Dav2dInloopFilterType,
    pub decode_frame_type: Dav2dDecodeFrameType,
    pub reserved: [u8; 16usize],
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Dav2dRef(c_void);

extern "C" {
    pub fn dav2d_version() -> *const c_char;

    pub fn dav2d_version_api() -> u32;

    pub fn dav2d_default_settings(s: *mut Dav2dSettings);

    pub fn dav2d_parse_sequence_header(
        out: *mut Dav2dSequenceHeader,
        buf: *const u8,
        sz: usize,
    ) -> c_int;

    pub fn dav2d_open(c_out: *mut *mut Dav2dContext, s: *const Dav2dSettings) -> c_int;

    pub fn dav2d_send_data(c: *mut Dav2dContext, in_: *mut Dav2dData) -> c_int;

    pub fn dav2d_flush(c: *mut Dav2dContext);

    pub fn dav2d_get_picture(c: *mut Dav2dContext, out: *mut Dav2dPicture) -> c_int;

    pub fn dav2d_get_decode_error_data_props(
        c: *mut Dav2dContext,
        out: *mut Dav2dDataProps,
    ) -> c_int;

    pub fn dav2d_get_frame_delay(s: *const Dav2dSettings) -> c_int;

    pub fn dav2d_apply_grain(
        c: *mut Dav2dContext,
        out: *mut Dav2dPicture,
        in_: *const Dav2dPicture,
    ) -> c_int;

    pub fn dav2d_get_event_flags(c: *mut Dav2dContext, flags: *mut Dav2dEventFlags) -> c_int;

    pub fn dav2d_close(c_out: *mut *mut Dav2dContext);

    pub fn dav2d_picture_unref(p: *mut Dav2dPicture);

    pub fn dav2d_data_props_unref(props: *mut Dav2dDataProps);

    pub fn dav2d_data_create(data: *mut Dav2dData, sz: usize) -> *mut u8;

    pub fn dav2d_data_wrap(
        data: *mut Dav2dData,
        buf: *const u8,
        sz: usize,
        free_callback: Option<unsafe extern "C" fn(buf: *const u8, cookie: *mut c_void)>,
        cookie: *mut c_void,
    ) -> c_int;

    pub fn dav2d_data_wrap_user_data(
        data: *mut Dav2dData,
        user_data: *const u8,
        free_callback: Option<unsafe extern "C" fn(user_data: *const u8, cookie: *mut c_void)>,
        cookie: *mut c_void,
    ) -> c_int;

    pub fn dav2d_data_unref(data: *mut Dav2dData);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    macro_rules! assert_size (
        ($t:ty, $sz:expr) => (
            assert_eq!(::std::mem::size_of::<$t>(), $sz);
        );
    );

    #[test]
    fn size() {
        #[cfg(target_pointer_width = "64")]
        assert_size!(Dav2dSettings, 96);
        #[cfg(target_pointer_width = "32")]
        assert_size!(Dav2dSettings, 76);
    }

    #[test]
    fn version() {
        println!("{}", unsafe {
            CStr::from_ptr(dav2d_version()).to_string_lossy()
        });
    }
}
