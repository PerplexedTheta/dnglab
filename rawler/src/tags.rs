// SPDX-License-Identifier: MIT
// Copyright 2021 Daniel Vogelbacher <daniel@chaospixel.com>

use std::convert::TryFrom;
use std::fmt::Debug;

macro_rules! tiff_tag_enum {
  ($e:ty) => {
    impl From<$e> for u16 {
      fn from(val: $e) -> u16 {
        val as u16
      }
    }
    impl TryFrom<u16> for $e {
      type Error = String;

      fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        Self::n(value).ok_or(format!("Unable to convert tag: {}, not defined in enum", value))
      }
    }
    impl crate::tags::TiffTag for $e {}
  };
}
pub(crate) use tiff_tag_enum;

/// A TIFF compatible tag identifier
pub trait TiffTag: Into<u16> + TryFrom<u16> + Debug + Sized + Copy {}

impl TiffTag for u16 {}

tiff_tag_enum!(TiffCommonTag);
tiff_tag_enum!(ExifTag);
tiff_tag_enum!(ExifGpsTag);
tiff_tag_enum!(DngTag);

/// Common tags, generally used in root IFD or SubIFDs
#[derive(Debug, Copy, Clone, PartialEq, enumn::N)]
#[repr(u16)]
pub enum TiffCommonTag {
  PanaWidth = 0x0002,
  PanaLength = 0x0003,
  NefWB0 = 0x000C,
  //PanaWBsR = 0x0011,
  //PanaWBsB = 0x0012,
  NrwWB = 0x0014,
  NefSerial = 0x001d,
  //PanaWBs2R = 0x0024,
  //PanaWBs2G = 0x0025,
  //PanaWBs2B = 0x0026,
  Cr2PowerShotWB = 0x0029,
  NewSubFileType = 0x00FE,
  Cr2OldOffset = 0x0081,
  NefMeta1 = 0x008c,
  NefMeta2 = 0x0096,
  NefWB1 = 0x0097,
  Cr2OldWB = 0x00A4,
  NefKey = 0x00a7,
  ImageWidth = 0x0100,
  ImageLength = 0x0101,
  BitsPerSample = 0x0102,
  SampleFormat = 339,
  RowsPerStrip = 278,
  XResolution = 282,
  YResolution = 283,
  ResolutionUnit = 296, // TODO add support
  Artist = 315,
  Compression = 0x0103,
  PhotometricInt = 0x0106,
  Make = 0x010F,
  Model = 0x0110,
  StripOffsets = 0x0111,
  Orientation = 0x0112,
  SamplesPerPixel = 0x0115,
  StripByteCounts = 0x0117,
  PanaOffsets = 0x0118,
  GrayResponse = 0x0123,
  Software = 0x0131,
  Predictor = 0x013D,
  TileWidth = 0x0142,
  TileLength = 0x0143,
  TileOffsets = 0x0144,
  TileByteCounts = 0x0145,
  SubIFDs = 0x014A,
  PefBlackLevels = 0x0200,
  PefWB = 0x0201,
  PefHuffman = 0x0220,
  Xmp = 0x02BC,
  DcrWB = 0x03FD,
  DcrLinearization = 0x090D,
  EpsonWB = 0x0E80,
  KodakWB = 0x0F00,
  RafOldWB = 0x2ff0,
  Cr2ColorData = 0x4001,
  SonyCurve = 0x7010,
  SonyOffset = 0x7200,
  SonyLength = 0x7201,
  SonyKey = 0x7221,
  KodakIFD = 0x8290,
  LeafMetadata = 0x8606,
  ExifIFDPointer = 0x8769,
  Makernote = 0x927C,
  SrwSensorAreas = 0xA010,
  Cr2Id = 0xc5d8,
  DNGVersion = 0xC612,
  Linearization = 0xC618,
  BlackLevels = 0xC61A,
  WhiteLevel = 0xC61D,
  ColorMatrix1 = 0xC621,
  ColorMatrix2 = 0xC622,
  AsShotNeutral = 0xC628,
  DNGPrivateArea = 0xC634,
  Cr2StripeWidths = 0xC640,
  ActiveArea = 0xC68D,
  MaskedAreas = 0xC68E,
  TimeCodes = 0xC763,
  FrameFrate = 0xC764,
  TStop = 0xC772,
  RafRawSubIFD = 0xF000,
  RafImageWidth = 0xF001,
  RafImageLength = 0xF002,
  RafBitsPerSample = 0xF003,
  //RafOffsets       = 0xF007,
  RafWBGRB = 0xF00E,
  KdcWB = 0xFA2A,
  KdcWidth = 0xFD00,
  KdcLength = 0xFD01,
  KdcOffset = 0xFD04,
  KdcIFD = 0xFE00,

  // TIFF-EP
  CFAPattern = 0x828E,
  CFARepeatPatternDim = 33421,
}

/// EXIF GPS tags
#[derive(Debug, Copy, Clone, PartialEq, enumn::N)]
#[repr(u16)]
pub enum ExifGpsTag {
  GPSVersionID = 0x0000,
  GPSLatitudeRef = 0x0001,
  GPSLatitude = 0x0002,
  GPSLongitudeRef = 0x0003,
  GPSLongitude = 0x0004,
  GPSAltitudeRef = 0x0005,
  GPSAltitude = 0x0006,
  GPSTimeStamp = 0x0007,
  GPSSatellites = 0x0008,
  GPSStatus = 0x0009,
  GPSMeasureMode = 0x000a,
  GPSDOP = 0x000b,
  GPSSpeedRef = 0x000c,
  GPSSpeed = 0x000d,
  GPSTrackRef = 0x000e,
  GPSTrack = 0x000f,
  GPSImgDirectionRef = 0x0010,
  GPSImgDirection = 0x0011,
  GPSMapDatum = 0x0012,
  GPSDestLatitudeRef = 0x0013,
  GPSDestLatitude = 0x0014,
  GPSDestLongitudeRef = 0x0015,
  GPSDestLongitude = 0x0016,
  GPSDestBearingRef = 0x0017,
  GPSDestBearing = 0x0018,
  GPSDestDistanceRef = 0x0019,
  GPSDestDistance = 0x001a,
  GPSProcessingMethod = 0x001b,
  GPSAreaInformation = 0x001c,
  GPSDateStamp = 0x001d,
  GPSDifferential = 0x001e,
  GPSHPositioningError = 0x001f,
}

/// EXIF tags
#[derive(Debug, Copy, Clone, PartialEq, enumn::N)]
#[repr(u16)]
pub enum ExifTag {
  InteropIndex = 0x0001,
  InteropVersion = 0x0002,
  ProcessingSoftware = 0x000b,
  SubfileType = 0x00fe,
  OldSubfileType = 0x00ff,
  ImageWidth = 0x100,
  ImageHeight = 0x101,
  BitsPerSample = 0x0102,
  Compression = 0x0103,
  PhotometricInterpretation = 0x0106,
  Thresholding = 0x0107,
  CellWidth = 0x0108,
  CellLength = 0x0109,
  FillOrder = 0x010a,
  DocumentName = 0x010d,
  ImageDescription = 0x010e,
  Make = 0x010f,
  Model = 0x0110,
  StripOffsets = 0x0111,
  Orientation = 0x0112,
  SamplesPerPixel = 0x0115,
  RowsPerStrip = 0x0116,
  StripByteCounts = 0x0117,
  MinSampleValue = 0x0118,
  MaxSampleValue = 0x0119,
  XResolution = 0x011a,
  YResolution = 0x011b,
  PlanarConfiguration = 0x011c,
  PageName = 0x011d,
  XPosition = 0x011e,
  YPosition = 0x011f,
  FreeOffsets = 0x0120,
  FreeByteCounts = 0x0121,
  GrayResponseUnit = 0x0122,
  GrayResponseCurve = 0x0123,
  T4Options = 0x0124,
  T6Options = 0x0125,
  ResolutionUnit = 0x0128,
  PageNumber = 0x0129,
  ColorResponseUnit = 0x012c,
  TransferFunction = 0x012d,
  Software = 0x0131,
  ModifyDate = 0x0132,
  Artist = 0x013b,
  HostComputer = 0x0103c,
  Predictor = 0x013d,
  WhitePoint = 0x013e,
  PrimaryChromaticities = 0x013f,
  ColorMap = 0x0140,
  HalftoneHints = 0x0141,
  TileWidth = 0x0142,
  TileLength = 0x0143,
  TileOffsets = 0x0144,
  TileByteCounts = 0x0145,
  BadFaxLines = 0x0146,
  CleanFaxData = 0x0147,
  ConsecutiveBadFaxLines = 0x0148,
  A100DataOffset = 0x014a,
  InkSet = 0x014c,
  InkNames = 0x014d,
  NumberofInks = 0x014e,
  DotRange = 0x0150,
  TargetPrinter = 0x0151,
  ExtraSamples = 0x0152,
  SampleFormat = 0x0153,
  SMinSampleValue = 0x0154,
  SMaxSampleValue = 0x0155,
  TransferRange = 0x0156,
  ClipPath = 0x0157,
  XClipPathUnits = 0x0158,
  YClipPathUnits = 0x0159,
  Indexed = 0x015a,
  JPEGTables = 0x015b,
  OPIProxy = 0x015f,
  GlobalParametersIFD = 0x0190,
  ProfileType = 0x0191,
  FaxProfile = 0x0192,
  CodingMethods = 0x0193,
  VersionYear = 0x0194,
  ModeNumber = 0x0195,
  Decode = 0x01b1,
  DefaultImageColor = 0x01b2,
  T82Options = 0x01b3,
  JPEGTables2 = 0x01b5,
  JPEGProc = 0x0200,
  JPEGInterchangeFormat = 0x0201,
  JPEGInterchangeFormatLength = 0x0202,
  JPEGRestartInterval = 0x0203,
  JPEGLosslessPredictors = 0x0205,
  JPEGPointTransforms = 0x0206,
  JPEGQTables = 0x0207,
  JPEGDCTables = 0x0208,
  JPEGACTables = 0x0209,
  YCbCrCoefficients = 0x0211,
  YCbCrSubSampling = 0x0212,
  YCbCrPositioning = 0x0213,
  ReferenceBlackWhite = 0x0214,
  StripRowCounts = 0x022f,
  ApplicationNotes = 0x02bc,
  USPTOMiscellaneous = 0x03e7,
  RelatedImageFileFormat = 0x1000,
  RelatedImageWidth = 0x1001,
  RelatedImageHeight = 0x1002,
  Rating = 0x4746,
  XpDipXml = 0x4747,
  StitchInfo = 0x4748,
  RatingPercent = 0x4749,
  SonyRawFileType = 0x7000,
  SonyToneCurve = 0x7010,
  VignettingCorrection = 0x7031,
  VignettingCorrParams = 0x7032,
  ChromaticAberrationCorrection = 0x7034,
  ChromaticAberrationCorrParams = 0x7035,
  DistortionCorrection = 0x7036,
  DistortionCorrParams = 0x7037,
  SonyRawImageSize = 0x7038,
  SonyCropTopLeft = 0x74c7,
  SonyCropSize = 0x74c8,
  ImageID = 0x800d,
  WangTag1 = 0x80a3,
  WangAnnotation = 0x80a4,
  WangTag3 = 0x80a5,
  WangTag4 = 0x80a6,
  ImageReferencePoints = 0x80b9,
  RegionXformTackPoint = 0x80ba,
  WarpQuadrilateral = 0x80bb,
  AffineTransformMat = 0x80bc,
  Matteing = 0x80e3,
  DataType = 0x80e4,
  ImageDepth = 0x80e5,
  TileDepth = 0x80e6,
  ImageFullWidth = 0x8214,
  ImageFullHeight = 0x8215,
  TextureFormat = 0x8216,
  WrapModes = 0x8217,
  FovCot = 0x8218,
  MatrixWorldToScreen = 0x8219,
  MatrixWorldToCamera = 0x821a,
  Model2 = 0x827d,
  CFARepeatPatternDim = 0x828d,
  CFAPattern2 = 0x828e,
  BatteryLevel = 0x828f,
  KodakIFD = 0x8290,
  Copyright = 0x8298,
  ExposureTime = 0x829a,
  FNumber = 0x829d,
  MDFileTag = 0x82a5,
  MDScalePixel = 0x82a6,
  MDColorTable = 0x82a7,
  MDLabName = 0x82a8,
  MDSampleInfo = 0x82a9,
  MDPrepDate = 0x82aa,
  MDPrepTime = 0x82ab,
  MDFileUnits = 0x82ac,
  PixelScale = 0x830e,
  AdventScale = 0x8335,
  AdventRevision = 0x8336,
  UIC1Tag = 0x835c,
  UIC2Tag = 0x835d,
  UIC3Tag = 0x835e,
  UIC4Tag = 0x835f,
  IptcNaa = 0x83bb,
  IntergraphPacketData = 0x847e,
  IntergraphFlagRegisters = 0x847f,
  IntergraphMatrix = 0x8480,
  INGRReserved = 0x8481,
  ModelTiePoint = 0x8482,
  Site = 0x84e0,
  ColorSequence = 0x84e1,
  IT8Header = 0x84e2,
  RasterPadding = 0x84e3,
  BitsPerRunLength = 0x84e4,
  BitsPerExtendedRunLength = 0x84e5,
  ColorTable = 0x84e6,
  ImageColorIndicator = 0x84e7,
  BackgroundColorIndicator = 0x84e8,
  ImageColorValue = 0x84e9,
  BackgroundColorValue = 0x84ea,
  PixelIntensityRange = 0x84eb,
  TransparencyIndicator = 0x84ec,
  ColorCharacterization = 0x84ed,
  HCUsage = 0x84ee,
  TrapIndicator = 0x84ef,
  CMYKEquivalent = 0x84f0,
  SEMInfo = 0x8546,
  AfcpIPTC = 0x8568,
  PixelMagicJBIGOptions = 0x85b8,
  JPLCartoIFD = 0x85d7,
  ModelTransform = 0x85d8,
  WbGRGBLevels = 0x8602,
  LeafData = 0x8606,
  PhotoshopSettings = 0x8649,
  ExifOffset = 0x8769,
  IccProfile = 0x8773,
  TiffFXExtensions = 0x877f,
  MultiProfiles = 0x8780,
  SharedData = 0x8781,
  T88Options = 0x8782,
  ImageLayer = 0x87ac,
  GeoTiffDirectory = 0x87af,
  GeoTiffDoubleParams = 0x87b0,
  GeoTiffAsciiParams = 0x87b1,
  JBIGOptions = 0x87be,
  ExposureProgram = 0x8822,
  SpectralSensitivity = 0x8824,
  GPSInfo = 0x8825,
  ISOSpeedRatings = 0x8827,
  ElectricConvFactor = 0x8828,
  Interlace = 0x8829,
  TimeZoneOffset = 0x882a,
  SelfTimerMode = 0x882b,

  SensitivityType = 0x8830,
  StandardOutputSensitivity = 0x8831,
  RecommendedExposureIndex = 0x8832,
  ISOSpeed = 0x8833,
  ISOSpeedLatitudeyyy = 0x8834,
  ISOSpeedLatitudezzz = 0x8835,
  LeafSubIFD = 0x888a,
  ExifVersion = 0x9000,
  DateTimeOriginal = 0x9003,
  CreateDate = 0x9004,
  GooglePlusUploadCode = 0x9009,
  OffsetTime = 0x9010,
  OffsetTimeOriginal = 0x9011,
  OffsetTimeDigitized = 0x9012,
  ComponentsConfiguration = 0x9101,
  CompressedBitsPerPixel = 0x9102,
  ShutterSpeedValue = 0x9201,
  ApertureValue = 0x9202,
  BrightnessValue = 0x9203,
  ExposureBiasValue = 0x9204,
  MaxApertureValue = 0x9205,
  SubjectDistance = 0x9206,
  MeteringMode = 0x9207,
  LightSource = 0x9208,
  Flash = 0x9209,
  FocalLength = 0x920a,
  FlashEnergy = 0x920b,
  SpatialFrequencyResponse = 0x920c,
  Noise2 = 0x920d,
  FocalPlaneXResolution = 0x920e,
  FocalPlaneYResolution = 0x920f,
  FocalPlaneResolutionUnit = 0x9210,
  ImageNumber = 0x9211,
  SecurityClassification = 0x9212,
  ImageHistory = 0x9213,
  SubjectArea = 0x9214,
  ExposureIndex = 0x9215,
  TiffEPStandardID = 0x9216,
  SensingMethod = 0x9217,
  CIP3DataFile = 0x923a,
  CIP3Sheet = 0x923b,
  CIP3Side = 0x923c,
  StoNits = 0x923f,
  MakerNotes = 0x927c,
  UserComment = 0x9286,
  SubSecTime = 0x9290,
  SubSecTimeOriginal = 0x9291,
  SubSecTimeDigitized = 0x9292,
  MSDocumentText = 0x932f,
  MSPropertySetStorage = 0x9330,
  MSDocumentTextPosition = 0x9331,
  ImageSourceData = 0x935c,
  AmbientTemperature = 0x9400,
  Humidity = 0x9401,
  Pressure = 0x9402,
  WaterDepth = 0x9403,
  Acceleration = 0x9404,
  CameraElevationAngle = 0x9405,
  XPTitle = 0x9c9b,
  XPComment = 0x9c9c,
  XPAuthor = 0x9c9d,
  XPKeywords = 0x9c9e,
  XPSubject = 0x9c9f,
  FlashpixVersion = 0xa000,
  ColorSpace = 0xa001,
  ExifImageWidth = 0xa002,
  ExifImageHeight = 0xa003,
  RelatedSoundFile = 0xa004,
  InteropOffset = 0xa005,
  SamsungRawPointersOffset = 0xa010,
  SamsungRawPointersLength = 0xa011,
  SamsungRawByteOrder = 0xa101,
  SamsungRawUnknown = 0xa102,
  FlashEnergy2 = 0xa20b,
  SpatialFrequencyResponse2 = 0xa20c,
  Noise = 0xa20d,
  FocalPlaneXResolution2 = 0xa20e,
  FocalPlaneYResolution2 = 0xa20f,
  FocalPlaneResolutionUnit2 = 0xa210,
  ImageNumber2 = 0xa211,
  SecurityClassification2 = 0xa212,
  ImageHistory2 = 0xa213,
  SubjectLocation = 0xa214,
  ExposureIndex2 = 0xa215,
  TiffEPStandardID2 = 0xa216,
  SensingMethod2 = 0xa217,
  FileSource = 0xa300,
  SceneType = 0xa301,
  CFAPattern = 0xa302,
  CustomRendered = 0xa401,
  ExposureMode = 0xa402,
  WhiteBalance = 0xa403,
  DigitalZoomRatio = 0xa404,
  FocalLengthIn35mmFormat = 0xa405,
  SceneCaptureType = 0xa406,
  GainControl = 0xa407,
  Contrast = 0xa408,
  Saturation = 0xa409,
  Sharpness = 0xa40a,
  DeviceSettingDescription = 0xa40b,
  SubjectDistanceRange = 0xa40c,
  ImageUniqueID = 0xa420,
  OwnerName = 0xa430,
  SerialNumber = 0xa431,
  LensSpecification = 0xa432,
  LensMake = 0xa433,
  LensModel = 0xa434,
  LensSerialNumber = 0xa435,
  CompositeImage = 0xa460,
  CompositeImageCount = 0xa461,
  CompositeImageExposureTimes = 0xa462,
  GDALMetadata = 0xa480,
  GDALNoData = 0xa481,
  Gamma = 0xa500,
  ExpandSoftware = 0xafc0,
  ExpandLens = 0xafc1,
  ExpandFilm = 0xafc2,
  ExpandFilterLens = 0xafc3,
  ExpandScanner = 0xafc4,
  ExpandFlashLamp = 0xafc5,
  HasselbladRawImage = 0xb4c3,
  PixelFormat = 0xbc01,
  Transformation = 0xbc02,
  Uncompressed = 0xbc03,
  ImageType = 0xbc04,
  Annotations = 0xc44f,
  PrintIM = 0xc4a5,
  HasselbladExif = 0xc51b,
  OriginalFileName = 0xc573,
  //ExifExposureTime = 0x829a,
  //ExifFNumber = 0x829d,
  //ExifISOSpeedRatings = 0x8827,
  //ExifVersion = 0x9000,
  //ExifLensModel = 0xa434,
  //ExifFocalLen = 0x920a,
}

/// DNG specific tags
#[derive(Debug, Copy, Clone, PartialEq, enumn::N)]
#[repr(u16)]
pub enum DngTag {
  DNGVersion = 50706,
  DNGBackwardVersion = 50707,
  UniqueCameraModel = 50708,
  LocalizedCameraModel = 50709,
  CFAPlaneColor = 50710,
  CFALayout = 50711,
  LinearizationTable = 50712,
  BlackLevelRepeatDim = 50713,
  BlackLevel = 50714,
  BlackLevelDeltaH = 50715,
  BlackLevelDeltaV = 50716,
  WhiteLevel = 50717,
  DefaultScale = 50718,
  BestQualityScale = 50780,
  DefaultCropOrigin = 50719,
  DefaultCropSize = 50720,
  CalibrationIlluminant1 = 50778,
  CalibrationIlluminant2 = 50779,
  ColorMatrix1 = 50721,
  ColorMatrix2 = 50722,
  CameraCalibration1 = 50723,
  CameraCalibration2 = 50724,
  ReductionMatrix1 = 50725,
  ReductionMatrix2 = 50726,
  AnalogBalance = 50727,
  AsShotNeutral = 50728,
  AsShotWhiteXY = 50729,
  BaselineExposure = 50730,
  BaselineNoise = 50731,
  BaselineSharpness = 50732,
  BayerGreenSplit = 50733,
  LinearResponseLimit = 50734,
  CameraSerialNumber = 50735,
  LensInfo = 50736,
  ChromaBlurRadius = 50737,
  AntiAliasStrength = 50738,
  ShadowScale = 50739,
  DNGPrivateData = 50740,
  MakerNoteSafety = 50741,
  RawDataUniqueID = 50781,
  OriginalRawFileName = 50827,
  OriginalRawFileData = 50828,
  ActiveArea = 50829,
  MaskedAreas = 50830,
  AsShotICCProfile = 50831,
  AsShotPreProfileMatrix = 50832,
  CurrentICCProfile = 50833,
  CurrentPreProfileMatrix = 50834,
  // 1.2.0
  ColorimetricReference = 50879,
  CameraCalibrationSignature = 50931,
  ProfileCalibrationSignature = 50932,
  ExtraCameraProfiles = 50933,
  AsShotProfileName = 50934,
  NoiseReductionApplied = 50935,
  ProfileName = 50936,
  ProfileHueSatMapDims = 50937,
  ProfileHueSatMapData1 = 50938,
  ProfileHueSatMapData2 = 50939,
  ProfileToneCurve = 50940,
  ProfileEmbedPolicy = 50941,
  ProfileCopyright = 50942,
  ForwardMatrix1 = 50964,
  ForwardMatrix2 = 50965,
  PreviewApplicationName = 50966,
  PreviewApplicationVersion = 50967,
  PreviewSettingsName = 50968,
  PreviewSettingsDigest = 50969,
  PreviewColorSpace = 50970,
  PreviewDateTime = 50971,
  RawImageDigest = 50972,
  OriginalRawFileDigest = 50973,
  SubTileBlockSize = 50974,
  RowInterleaveFactor = 50975,
  ProfileLookTableDims = 50981,
  ProfileLookTableData = 50982,
  // 1.3.0
  OpcodeList1 = 51008,
  OpcodeList2 = 51009,
  OpcodeList3 = 51022,
  NoiseProfile = 51041,
  // 1.4.0
  DefaultUserCrop = 51125,
  DefaultBlackRender = 51110,
  BaselineExposureOffset = 51109,
  ProfileLookTableEncoding = 51108,
  ProfileHueSatMapEncoding = 51107,
  OriginalDefaultFinalSize = 51089,
  OriginalBestQualityFinalSize = 51090,
  OriginalDefaultCropSize = 51091,
  NewRawImageDigest = 51111,
  RawToPreviewGain = 51112,
  // 1.5.0
  DepthFormat = 51177,
  DepthNear = 51178,
  DepthFar = 51179,
  DepthUnits = 51180,
  DepthMeasureType = 51181,
  EnhanceParams = 51182,

  // 1.6.0
  ProfileGainTableMap = 52525,
  SemanticName = 52526,
  SemanticInstanceID = 52528,
  MaskSubArea = 52536,
  RGBTables = 52543,

  CalibrationIlluminant3 = 52529,
  ColorMatrix3 = 52531,
  CameraCalibration3 = 52530,
  ReductionMatrix3 = 52538,
  ProfileHueSatMapData3 = 52537,
  ForwardMatrix3 = 52532,
  IlluminantData1 = 52533,
  IlluminantData2 = 52534,
  IlluminantData3 = 52535,

  // 1.7.1
  ColumnInterleaveFactor = 52547,
}
