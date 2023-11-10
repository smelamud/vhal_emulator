// SPDX-FileCopyrightText: Red Hat, Inc.
// SPDX-License-Identifier: GPL-2.0-or-later

// This file is generated by gen_vhal_const.py. Do not edit
// .hal file is parsed by hidl_parser.py
// Source file copied from hardware/interfaces/automotive/vehicle/2.0/types.hal
// File generated on: 2023-11-10

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehiclePropertyType {
    STRING = 0x100000,
    BOOLEAN = 0x200000,
    INT32 = 0x400000,
    INT32_VEC = 0x410000,
    INT64 = 0x500000,
    INT64_VEC = 0x510000,
    FLOAT = 0x600000,
    FLOAT_VEC = 0x610000,
    BYTES = 0x700000,
    MIXED = 0xe00000,
    MASK = 0xff0000,
}

impl VehiclePropertyType {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::STRING)
    }

    pub fn is_bytes(&self) -> bool {
        matches!(self, Self::BYTES)
    }

    pub fn is_int32(&self) -> bool {
        matches!(self, Self::BOOLEAN) || matches!(self, Self::INT32)
    }

    pub fn is_int64(&self) -> bool {
        matches!(self, Self::INT64)
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::FLOAT)
    }

    pub fn is_int32s(&self) -> bool {
        matches!(self, Self::INT32_VEC)
    }

    pub fn is_int64s(&self) -> bool {
        matches!(self, Self::INT64_VEC)
    }

    pub fn is_floats(&self) -> bool {
        matches!(self, Self::FLOAT_VEC)
    }

    pub fn is_mixed(&self) -> bool {
        matches!(self, Self::MIXED)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleArea {
    GLOBAL = 0x1000000,
    WINDOW = 0x3000000,
    MIRROR = 0x4000000,
    SEAT = 0x5000000,
    DOOR = 0x6000000,
    WHEEL = 0x7000000,
    MASK = 0xf000000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(u32)]
pub enum VehiclePropertyGroup {
    SYSTEM = 0x10000000,
    VENDOR = 0x20000000,
    MASK = 0xf0000000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(Hash)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleProperty {
    INVALID = 0x0,
    INFO_VIN = 0x11100100,
    INFO_MAKE = 0x11100101,
    INFO_MODEL = 0x11100102,
    INFO_MODEL_YEAR = 0x11400103,
    INFO_FUEL_CAPACITY = 0x11600104,
    INFO_FUEL_TYPE = 0x11410105,
    INFO_EV_BATTERY_CAPACITY = 0x11600106,
    INFO_EV_CONNECTOR_TYPE = 0x11410107,
    INFO_FUEL_DOOR_LOCATION = 0x11400108,
    INFO_EV_PORT_LOCATION = 0x11400109,
    INFO_DRIVER_SEAT = 0x1540010a,
    INFO_EXTERIOR_DIMENSIONS = 0x1141010b,
    INFO_MULTI_EV_PORT_LOCATIONS = 0x1141010c,
    PERF_ODOMETER = 0x11600204,
    PERF_VEHICLE_SPEED = 0x11600207,
    PERF_VEHICLE_SPEED_DISPLAY = 0x11600208,
    PERF_STEERING_ANGLE = 0x11600209,
    PERF_REAR_STEERING_ANGLE = 0x11600210,
    ENGINE_COOLANT_TEMP = 0x11600301,
    ENGINE_OIL_LEVEL = 0x11400303,
    ENGINE_OIL_TEMP = 0x11600304,
    ENGINE_RPM = 0x11600305,
    WHEEL_TICK = 0x11510306,
    FUEL_LEVEL = 0x11600307,
    FUEL_DOOR_OPEN = 0x11200308,
    EV_BATTERY_LEVEL = 0x11600309,
    EV_CHARGE_PORT_OPEN = 0x1120030a,
    EV_CHARGE_PORT_CONNECTED = 0x1120030b,
    EV_BATTERY_INSTANTANEOUS_CHARGE_RATE = 0x1160030c,
    RANGE_REMAINING = 0x11600308,
    TIRE_PRESSURE = 0x17600309,
    CRITICALLY_LOW_TIRE_PRESSURE = 0x1760030a,
    GEAR_SELECTION = 0x11400400,
    CURRENT_GEAR = 0x11400401,
    PARKING_BRAKE_ON = 0x11200402,
    PARKING_BRAKE_AUTO_APPLY = 0x11200403,
    FUEL_LEVEL_LOW = 0x11200405,
    NIGHT_MODE = 0x11200407,
    TURN_SIGNAL_STATE = 0x11400408,
    IGNITION_STATE = 0x11400409,
    ABS_ACTIVE = 0x1120040a,
    TRACTION_CONTROL_ACTIVE = 0x1120040b,
    HVAC_FAN_SPEED = 0x15400500,
    HVAC_FAN_DIRECTION = 0x15400501,
    HVAC_TEMPERATURE_CURRENT = 0x15600502,
    HVAC_TEMPERATURE_SET = 0x15600503,
    HVAC_DEFROSTER = 0x13200504,
    HVAC_AC_ON = 0x15200505,
    HVAC_MAX_AC_ON = 0x15200506,
    HVAC_MAX_DEFROST_ON = 0x15200507,
    HVAC_RECIRC_ON = 0x15200508,
    HVAC_DUAL_ON = 0x15200509,
    HVAC_AUTO_ON = 0x1520050a,
    HVAC_SEAT_TEMPERATURE = 0x1540050b,
    HVAC_SIDE_MIRROR_HEAT = 0x1440050c,
    HVAC_STEERING_WHEEL_HEAT = 0x1140050d,
    HVAC_TEMPERATURE_DISPLAY_UNITS = 0x1140050e,
    HVAC_ACTUAL_FAN_SPEED_RPM = 0x1540050f,
    HVAC_POWER_ON = 0x15200510,
    HVAC_FAN_DIRECTION_AVAILABLE = 0x15410511,
    HVAC_AUTO_RECIRC_ON = 0x15200512,
    HVAC_SEAT_VENTILATION = 0x15400513,
    HVAC_ELECTRIC_DEFROSTER_ON = 0x13200514,
    HVAC_TEMPERATURE_VALUE_SUGGESTION = 0x11610515,
    DISTANCE_DISPLAY_UNITS = 0x11400600,
    FUEL_VOLUME_DISPLAY_UNITS = 0x11400601,
    TIRE_PRESSURE_DISPLAY_UNITS = 0x11400602,
    EV_BATTERY_DISPLAY_UNITS = 0x11400603,
    FUEL_CONSUMPTION_UNITS_DISTANCE_OVER_VOLUME = 0x11200604,
    VEHICLE_SPEED_DISPLAY_UNITS = 0x11400605,
    EPOCH_TIME = 0x11500606,
    STORAGE_ENCRYPTION_BINDING_SEED = 0x11700607,
    ENV_OUTSIDE_TEMPERATURE = 0x11600703,
    AP_POWER_STATE_REQ = 0x11410a00,
    AP_POWER_STATE_REPORT = 0x11410a01,
    AP_POWER_BOOTUP_REASON = 0x11400a02,
    DISPLAY_BRIGHTNESS = 0x11400a03,
    HW_KEY_INPUT = 0x11410a10,
    HW_ROTARY_INPUT = 0x11410a20,
    HW_CUSTOM_INPUT = 0x11410a30,
    DOOR_POS = 0x16400b00,
    DOOR_MOVE = 0x16400b01,
    DOOR_LOCK = 0x16200b02,
    MIRROR_Z_POS = 0x14400b40,
    MIRROR_Z_MOVE = 0x14400b41,
    MIRROR_Y_POS = 0x14400b42,
    MIRROR_Y_MOVE = 0x14400b43,
    MIRROR_LOCK = 0x11200b44,
    MIRROR_FOLD = 0x11200b45,
    SEAT_MEMORY_SELECT = 0x15400b80,
    SEAT_MEMORY_SET = 0x15400b81,
    SEAT_BELT_BUCKLED = 0x15200b82,
    SEAT_BELT_HEIGHT_POS = 0x15400b83,
    SEAT_BELT_HEIGHT_MOVE = 0x15400b84,
    SEAT_FORE_AFT_POS = 0x15400b85,
    SEAT_FORE_AFT_MOVE = 0x15400b86,
    SEAT_BACKREST_ANGLE_1_POS = 0x15400b87,
    SEAT_BACKREST_ANGLE_1_MOVE = 0x15400b88,
    SEAT_BACKREST_ANGLE_2_POS = 0x15400b89,
    SEAT_BACKREST_ANGLE_2_MOVE = 0x15400b8a,
    SEAT_HEIGHT_POS = 0x15400b8b,
    SEAT_HEIGHT_MOVE = 0x15400b8c,
    SEAT_DEPTH_POS = 0x15400b8d,
    SEAT_DEPTH_MOVE = 0x15400b8e,
    SEAT_TILT_POS = 0x15400b8f,
    SEAT_TILT_MOVE = 0x15400b90,
    SEAT_LUMBAR_FORE_AFT_POS = 0x15400b91,
    SEAT_LUMBAR_FORE_AFT_MOVE = 0x15400b92,
    SEAT_LUMBAR_SIDE_SUPPORT_POS = 0x15400b93,
    SEAT_LUMBAR_SIDE_SUPPORT_MOVE = 0x15400b94,
    SEAT_HEADREST_HEIGHT_POS = 0x11400b95,
    SEAT_HEADREST_HEIGHT_MOVE = 0x15400b96,
    SEAT_HEADREST_ANGLE_POS = 0x15400b97,
    SEAT_HEADREST_ANGLE_MOVE = 0x15400b98,
    SEAT_HEADREST_FORE_AFT_POS = 0x15400b99,
    SEAT_HEADREST_FORE_AFT_MOVE = 0x15400b9a,
    SEAT_OCCUPANCY = 0x15400bb0,
    WINDOW_POS = 0x13400bc0,
    WINDOW_MOVE = 0x13400bc1,
    WINDOW_LOCK = 0x13200bc4,
    VEHICLE_MAP_SERVICE = 0x11e00c00,
    OBD2_LIVE_FRAME = 0x11e00d00,
    OBD2_FREEZE_FRAME = 0x11e00d01,
    OBD2_FREEZE_FRAME_INFO = 0x11e00d02,
    OBD2_FREEZE_FRAME_CLEAR = 0x11e00d03,
    HEADLIGHTS_STATE = 0x11400e00,
    HIGH_BEAM_LIGHTS_STATE = 0x11400e01,
    FOG_LIGHTS_STATE = 0x11400e02,
    HAZARD_LIGHTS_STATE = 0x11400e03,
    HEADLIGHTS_SWITCH = 0x11400e10,
    HIGH_BEAM_LIGHTS_SWITCH = 0x11400e11,
    FOG_LIGHTS_SWITCH = 0x11400e12,
    HAZARD_LIGHTS_SWITCH = 0x11400e13,
    CABIN_LIGHTS_STATE = 0x11400f01,
    CABIN_LIGHTS_SWITCH = 0x11400f02,
    READING_LIGHTS_STATE = 0x15400f03,
    READING_LIGHTS_SWITCH = 0x15400f04,
    SUPPORT_CUSTOMIZE_VENDOR_PERMISSION = 0x11200f05,
    DISABLED_OPTIONAL_FEATURES = 0x11100f06,
    INITIAL_USER_INFO = 0x11e00f07,
    SWITCH_USER = 0x11e00f08,
    CREATE_USER = 0x11e00f09,
    REMOVE_USER = 0x11e00f0a,
    USER_IDENTIFICATION_ASSOCIATION = 0x11e00f0b,
    EVS_SERVICE_REQUEST = 0x11410f10,
    POWER_POLICY_REQ = 0x11100f21,
    POWER_POLICY_GROUP_REQ = 0x11100f22,
    CURRENT_POWER_POLICY = 0x11100f23,
    WATCHDOG_ALIVE = 0x11500f31,
    WATCHDOG_TERMINATED_PROCESS = 0x11e00f32,
    VHAL_HEARTBEAT = 0x11500f33,
    CLUSTER_SWITCH_UI = 0x11400f34,
    CLUSTER_DISPLAY_STATE = 0x11410f35,
    CLUSTER_REPORT_STATE = 0x11e00f36,
    CLUSTER_REQUEST_DISPLAY = 0x11400f37,
    CLUSTER_NAVIGATION_STATE = 0x11700f38,
    ELECTRONIC_TOLL_COLLECTION_CARD_TYPE = 0x11400f39,
    ELECTRONIC_TOLL_COLLECTION_CARD_STATUS = 0x11400f3a,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum ElectronicTollCollectionCardType {
    UNKNOWN = 0x0,
    JP_ELECTRONIC_TOLL_COLLECTION_CARD = 0x1,
    JP_ELECTRONIC_TOLL_COLLECTION_CARD_V2 = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum ElectronicTollCollectionCardStatus {
    UNKNOWN = 0x0,
    ELECTRONIC_TOLL_COLLECTION_CARD_VALID = 0x1,
    ELECTRONIC_TOLL_COLLECTION_CARD_INVALID = 0x2,
    ELECTRONIC_TOLL_COLLECTION_CARD_NOT_INSERTED = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(u32)]
pub enum VehicleVendorPermission {
    PERMISSION_DEFAULT = 0x0,
    PERMISSION_SET_VENDOR_CATEGORY_WINDOW = 0x1,
    PERMISSION_GET_VENDOR_CATEGORY_WINDOW = 0x2,
    PERMISSION_SET_VENDOR_CATEGORY_DOOR = 0x3,
    PERMISSION_GET_VENDOR_CATEGORY_DOOR = 0x4,
    PERMISSION_SET_VENDOR_CATEGORY_SEAT = 0x5,
    PERMISSION_GET_VENDOR_CATEGORY_SEAT = 0x6,
    PERMISSION_SET_VENDOR_CATEGORY_MIRROR = 0x7,
    PERMISSION_GET_VENDOR_CATEGORY_MIRROR = 0x8,
    PERMISSION_SET_VENDOR_CATEGORY_INFO = 0x9,
    PERMISSION_GET_VENDOR_CATEGORY_INFO = 0xa,
    PERMISSION_SET_VENDOR_CATEGORY_ENGINE = 0xb,
    PERMISSION_GET_VENDOR_CATEGORY_ENGINE = 0xc,
    PERMISSION_SET_VENDOR_CATEGORY_HVAC = 0xd,
    PERMISSION_GET_VENDOR_CATEGORY_HVAC = 0xe,
    PERMISSION_SET_VENDOR_CATEGORY_LIGHT = 0xf,
    PERMISSION_GET_VENDOR_CATEGORY_LIGHT = 0x10,
    PERMISSION_SET_VENDOR_CATEGORY_1 = 0x10000,
    PERMISSION_GET_VENDOR_CATEGORY_1 = 0x11000,
    PERMISSION_SET_VENDOR_CATEGORY_2 = 0x20000,
    PERMISSION_GET_VENDOR_CATEGORY_2 = 0x21000,
    PERMISSION_SET_VENDOR_CATEGORY_3 = 0x30000,
    PERMISSION_GET_VENDOR_CATEGORY_3 = 0x31000,
    PERMISSION_SET_VENDOR_CATEGORY_4 = 0x40000,
    PERMISSION_GET_VENDOR_CATEGORY_4 = 0x41000,
    PERMISSION_SET_VENDOR_CATEGORY_5 = 0x50000,
    PERMISSION_GET_VENDOR_CATEGORY_5 = 0x51000,
    PERMISSION_SET_VENDOR_CATEGORY_6 = 0x60000,
    PERMISSION_GET_VENDOR_CATEGORY_6 = 0x61000,
    PERMISSION_SET_VENDOR_CATEGORY_7 = 0x70000,
    PERMISSION_GET_VENDOR_CATEGORY_7 = 0x71000,
    PERMISSION_SET_VENDOR_CATEGORY_8 = 0x80000,
    PERMISSION_GET_VENDOR_CATEGORY_8 = 0x81000,
    PERMISSION_SET_VENDOR_CATEGORY_9 = 0x90000,
    PERMISSION_GET_VENDOR_CATEGORY_9 = 0x91000,
    PERMISSION_SET_VENDOR_CATEGORY_10 = 0xa0000,
    PERMISSION_GET_VENDOR_CATEGORY_10 = 0xa1000,
    PERMISSION_NOT_ACCESSIBLE = 0xf0000000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleSeatOccupancyState {
    UNKNOWN = 0x0,
    VACANT = 0x1,
    OCCUPIED = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum EvsServiceType {
    REARVIEW = 0x0,
    SURROUNDVIEW = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum EvsServiceState {
    OFF = 0x0,
    ON = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum EvsServiceRequestIndex {
    TYPE = 0x0,
    STATE = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleLightState {
    OFF = 0x0,
    ON = 0x1,
    DAYTIME_RUNNING = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleLightSwitch {
    OFF = 0x0,
    ON = 0x1,
    DAYTIME_RUNNING = 0x2,
    AUTOMATIC = 0x100,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum EvConnectorType {
    UNKNOWN = 0x0,
    IEC_TYPE_1_AC = 0x1,
    IEC_TYPE_2_AC = 0x2,
    IEC_TYPE_3_AC = 0x3,
    IEC_TYPE_4_DC = 0x4,
    IEC_TYPE_1_CCS_DC = 0x5,
    IEC_TYPE_2_CCS_DC = 0x6,
    TESLA_ROADSTER = 0x7,
    TESLA_HPWC = 0x8,
    TESLA_SUPERCHARGER = 0x9,
    GBT_AC = 0xa,
    GBT_DC = 0xb,
    OTHER = 0x65,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum PortLocationType {
    UNKNOWN = 0x0,
    FRONT_LEFT = 0x1,
    FRONT_RIGHT = 0x2,
    REAR_RIGHT = 0x3,
    REAR_LEFT = 0x4,
    FRONT = 0x5,
    REAR = 0x6,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum FuelType {
    FUEL_TYPE_UNKNOWN = 0x0,
    FUEL_TYPE_UNLEADED = 0x1,
    FUEL_TYPE_LEADED = 0x2,
    FUEL_TYPE_DIESEL_1 = 0x3,
    FUEL_TYPE_DIESEL_2 = 0x4,
    FUEL_TYPE_BIODIESEL = 0x5,
    FUEL_TYPE_E85 = 0x6,
    FUEL_TYPE_LPG = 0x7,
    FUEL_TYPE_CNG = 0x8,
    FUEL_TYPE_LNG = 0x9,
    FUEL_TYPE_ELECTRIC = 0xa,
    FUEL_TYPE_HYDROGEN = 0xb,
    FUEL_TYPE_OTHER = 0xc,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleHvacFanDirection {
    UNKNOWN = 0x0,
    FACE = 0x1,
    FLOOR = 0x2,
    FACE_AND_FLOOR = 0x3,
    DEFROST = 0x4,
    DEFROST_AND_FLOOR = 0x6,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleOilLevel {
    CRITICALLY_LOW = 0x0,
    LOW = 0x1,
    NORMAL = 0x2,
    HIGH = 0x3,
    ERROR = 0x4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleApPowerStateConfigFlag {
    ENABLE_DEEP_SLEEP_FLAG = 0x1,
    CONFIG_SUPPORT_TIMER_POWER_ON_FLAG = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleApPowerStateReq {
    ON = 0x0,
    SHUTDOWN_PREPARE = 0x1,
    CANCEL_SHUTDOWN = 0x2,
    FINISHED = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleApPowerStateReqIndex {
    STATE = 0x0,
    ADDITIONAL = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleApPowerStateShutdownParam {
    SHUTDOWN_IMMEDIATELY = 0x1,
    CAN_SLEEP = 0x2,
    SHUTDOWN_ONLY = 0x3,
    SLEEP_IMMEDIATELY = 0x4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleApPowerStateReport {
    WAIT_FOR_VHAL = 0x1,
    DEEP_SLEEP_ENTRY = 0x2,
    DEEP_SLEEP_EXIT = 0x3,
    SHUTDOWN_POSTPONE = 0x4,
    SHUTDOWN_START = 0x5,
    ON = 0x6,
    SHUTDOWN_PREPARE = 0x7,
    SHUTDOWN_CANCELLED = 0x8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleHwKeyInputAction {
    ACTION_DOWN = 0x0,
    ACTION_UP = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleDisplay {
    MAIN = 0x0,
    INSTRUMENT_CLUSTER = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleUnit {
    SHOULD_NOT_USE = 0x0,
    METER_PER_SEC = 0x1,
    RPM = 0x2,
    HERTZ = 0x3,
    PERCENTILE = 0x10,
    MILLIMETER = 0x20,
    METER = 0x21,
    KILOMETER = 0x23,
    MILE = 0x24,
    CELSIUS = 0x30,
    FAHRENHEIT = 0x31,
    KELVIN = 0x32,
    MILLILITER = 0x40,
    LITER = 0x41,
    GALLON = 0x42,
    IMPERIAL_GALLON = 0x43,
    NANO_SECS = 0x50,
    MILLI_SECS = 0x51,
    SECS = 0x53,
    YEAR = 0x59,
    WATT_HOUR = 0x60,
    MILLIAMPERE = 0x61,
    MILLIVOLT = 0x62,
    MILLIWATTS = 0x63,
    AMPERE_HOURS = 0x64,
    KILOWATT_HOUR = 0x65,
    KILOPASCAL = 0x70,
    PSI = 0x71,
    BAR = 0x72,
    DEGREES = 0x80,
    MILES_PER_HOUR = 0x90,
    KILOMETERS_PER_HOUR = 0x91,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehiclePropertyChangeMode {
    STATIC = 0x0,
    ON_CHANGE = 0x1,
    CONTINUOUS = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehiclePropertyAccess {
    NONE = 0x0,
    READ = 0x1,
    WRITE = 0x2,
    READ_WRITE = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehiclePropertyStatus {
    AVAILABLE = 0x0,
    UNAVAILABLE = 0x1,
    ERROR = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleGear {
    GEAR_UNKNOWN = 0x0,
    GEAR_NEUTRAL = 0x1,
    GEAR_REVERSE = 0x2,
    GEAR_PARK = 0x4,
    GEAR_DRIVE = 0x8,
    GEAR_1 = 0x10,
    GEAR_2 = 0x20,
    GEAR_3 = 0x40,
    GEAR_4 = 0x80,
    GEAR_5 = 0x100,
    GEAR_6 = 0x200,
    GEAR_7 = 0x400,
    GEAR_8 = 0x800,
    GEAR_9 = 0x1000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleAreaSeat {
    ROW_1_LEFT = 0x1,
    ROW_1_CENTER = 0x2,
    ROW_1_RIGHT = 0x4,
    ROW_2_LEFT = 0x10,
    ROW_2_CENTER = 0x20,
    ROW_2_RIGHT = 0x40,
    ROW_3_LEFT = 0x100,
    ROW_3_CENTER = 0x200,
    ROW_3_RIGHT = 0x400,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleAreaWindow {
    FRONT_WINDSHIELD = 0x1,
    REAR_WINDSHIELD = 0x2,
    ROW_1_LEFT = 0x10,
    ROW_1_RIGHT = 0x40,
    ROW_2_LEFT = 0x100,
    ROW_2_RIGHT = 0x400,
    ROW_3_LEFT = 0x1000,
    ROW_3_RIGHT = 0x4000,
    ROOF_TOP_1 = 0x10000,
    ROOF_TOP_2 = 0x20000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleAreaDoor {
    ROW_1_LEFT = 0x1,
    ROW_1_RIGHT = 0x4,
    ROW_2_LEFT = 0x10,
    ROW_2_RIGHT = 0x40,
    ROW_3_LEFT = 0x100,
    ROW_3_RIGHT = 0x400,
    HOOD = 0x10000000,
    REAR = 0x20000000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleAreaMirror {
    DRIVER_LEFT = 0x1,
    DRIVER_RIGHT = 0x2,
    DRIVER_CENTER = 0x4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleTurnSignal {
    NONE = 0x0,
    RIGHT = 0x1,
    LEFT = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleIgnitionState {
    UNDEFINED = 0x0,
    LOCK = 0x1,
    OFF = 0x2,
    ACC = 0x3,
    ON = 0x4,
    START = 0x5,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum SubscribeFlags {
    UNDEFINED = 0x0,
    EVENTS_FROM_CAR = 0x1,
    EVENTS_FROM_ANDROID = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum StatusCode {
    OK = 0x0,
    TRY_AGAIN = 0x1,
    INVALID_ARG = 0x2,
    NOT_AVAILABLE = 0x3,
    ACCESS_DENIED = 0x4,
    INTERNAL_ERROR = 0x5,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleAreaWheel {
    UNKNOWN = 0x0,
    LEFT_FRONT = 0x1,
    RIGHT_FRONT = 0x2,
    LEFT_REAR = 0x4,
    RIGHT_REAR = 0x8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum Obd2FuelSystemStatus {
    OPEN_INSUFFICIENT_ENGINE_TEMPERATURE = 0x1,
    CLOSED_LOOP = 0x2,
    OPEN_ENGINE_LOAD_OR_DECELERATION = 0x4,
    OPEN_SYSTEM_FAILURE = 0x8,
    CLOSED_LOOP_BUT_FEEDBACK_FAULT = 0x10,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum Obd2IgnitionMonitorKind {
    SPARK = 0x0,
    COMPRESSION = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum Obd2CommonIgnitionMonitors {
    COMPONENTS_AVAILABLE = 0x1,
    COMPONENTS_INCOMPLETE = 0x2,
    FUEL_SYSTEM_AVAILABLE = 0x4,
    FUEL_SYSTEM_INCOMPLETE = 0x8,
    MISFIRE_AVAILABLE = 0x10,
    MISFIRE_INCOMPLETE = 0x20,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Obd2SparkIgnitionMonitors {
    EGR_AVAILABLE = 0x40,
    EGR_INCOMPLETE = 0x80,
    OXYGEN_SENSOR_HEATER_AVAILABLE = 0x100,
    OXYGEN_SENSOR_HEATER_INCOMPLETE = 0x200,
    OXYGEN_SENSOR_AVAILABLE = 0x400,
    OXYGEN_SENSOR_INCOMPLETE = 0x800,
    AC_REFRIGERANT_AVAILABLE = 0x1000,
    AC_REFRIGERANT_INCOMPLETE = 0x2000,
    SECONDARY_AIR_SYSTEM_AVAILABLE = 0x4000,
    SECONDARY_AIR_SYSTEM_INCOMPLETE = 0x8000,
    EVAPORATIVE_SYSTEM_AVAILABLE = 0x10000,
    EVAPORATIVE_SYSTEM_INCOMPLETE = 0x20000,
    HEATED_CATALYST_AVAILABLE = 0x40000,
    HEATED_CATALYST_INCOMPLETE = 0x80000,
    CATALYST_AVAILABLE = 0x100000,
    CATALYST_INCOMPLETE = 0x200000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Obd2CompressionIgnitionMonitors {
    EGR_OR_VVT_AVAILABLE = 0x40,
    EGR_OR_VVT_INCOMPLETE = 0x80,
    PM_FILTER_AVAILABLE = 0x100,
    PM_FILTER_INCOMPLETE = 0x200,
    EXHAUST_GAS_SENSOR_AVAILABLE = 0x400,
    EXHAUST_GAS_SENSOR_INCOMPLETE = 0x800,
    BOOST_PRESSURE_AVAILABLE = 0x1000,
    BOOST_PRESSURE_INCOMPLETE = 0x2000,
    NOx_SCR_AVAILABLE = 0x4000,
    NOx_SCR_INCOMPLETE = 0x8000,
    NMHC_CATALYST_AVAILABLE = 0x10000,
    NMHC_CATALYST_INCOMPLETE = 0x20000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum Obd2SecondaryAirStatus {
    UPSTREAM = 0x1,
    DOWNSTREAM_OF_CATALYCIC_CONVERTER = 0x2,
    FROM_OUTSIDE_OR_OFF = 0x4,
    PUMP_ON_FOR_DIAGNOSTICS = 0x8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum Obd2FuelType {
    NOT_AVAILABLE = 0x0,
    GASOLINE = 0x1,
    METHANOL = 0x2,
    ETHANOL = 0x3,
    DIESEL = 0x4,
    LPG = 0x5,
    CNG = 0x6,
    PROPANE = 0x7,
    ELECTRIC = 0x8,
    BIFUEL_RUNNING_GASOLINE = 0x9,
    BIFUEL_RUNNING_METHANOL = 0xa,
    BIFUEL_RUNNING_ETHANOL = 0xb,
    BIFUEL_RUNNING_LPG = 0xc,
    BIFUEL_RUNNING_CNG = 0xd,
    BIFUEL_RUNNING_PROPANE = 0xe,
    BIFUEL_RUNNING_ELECTRIC = 0xf,
    BIFUEL_RUNNING_ELECTRIC_AND_COMBUSTION = 0x10,
    HYBRID_GASOLINE = 0x11,
    HYBRID_ETHANOL = 0x12,
    HYBRID_DIESEL = 0x13,
    HYBRID_ELECTRIC = 0x14,
    HYBRID_RUNNING_ELECTRIC_AND_COMBUSTION = 0x15,
    HYBRID_REGENERATIVE = 0x16,
    BIFUEL_RUNNING_DIESEL = 0x17,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum DiagnosticIntegerSensorIndex {
    FUEL_SYSTEM_STATUS = 0x0,
    MALFUNCTION_INDICATOR_LIGHT_ON = 0x1,
    IGNITION_MONITORS_SUPPORTED = 0x2,
    IGNITION_SPECIFIC_MONITORS = 0x3,
    INTAKE_AIR_TEMPERATURE = 0x4,
    COMMANDED_SECONDARY_AIR_STATUS = 0x5,
    NUM_OXYGEN_SENSORS_PRESENT = 0x6,
    RUNTIME_SINCE_ENGINE_START = 0x7,
    DISTANCE_TRAVELED_WITH_MALFUNCTION_INDICATOR_LIGHT_ON = 0x8,
    WARMUPS_SINCE_CODES_CLEARED = 0x9,
    DISTANCE_TRAVELED_SINCE_CODES_CLEARED = 0xa,
    ABSOLUTE_BAROMETRIC_PRESSURE = 0xb,
    CONTROL_MODULE_VOLTAGE = 0xc,
    AMBIENT_AIR_TEMPERATURE = 0xd,
    TIME_WITH_MALFUNCTION_LIGHT_ON = 0xe,
    TIME_SINCE_TROUBLE_CODES_CLEARED = 0xf,
    MAX_FUEL_AIR_EQUIVALENCE_RATIO = 0x10,
    MAX_OXYGEN_SENSOR_VOLTAGE = 0x11,
    MAX_OXYGEN_SENSOR_CURRENT = 0x12,
    MAX_INTAKE_MANIFOLD_ABSOLUTE_PRESSURE = 0x13,
    MAX_AIR_FLOW_RATE_FROM_MASS_AIR_FLOW_SENSOR = 0x14,
    FUEL_TYPE = 0x15,
    FUEL_RAIL_ABSOLUTE_PRESSURE = 0x16,
    ENGINE_OIL_TEMPERATURE = 0x17,
    DRIVER_DEMAND_PERCENT_TORQUE = 0x18,
    ENGINE_ACTUAL_PERCENT_TORQUE = 0x19,
    ENGINE_REFERENCE_PERCENT_TORQUE = 0x1a,
    ENGINE_PERCENT_TORQUE_DATA_IDLE = 0x1b,
    ENGINE_PERCENT_TORQUE_DATA_POINT1 = 0x1c,
    ENGINE_PERCENT_TORQUE_DATA_POINT2 = 0x1d,
    ENGINE_PERCENT_TORQUE_DATA_POINT3 = 0x1e,
    ENGINE_PERCENT_TORQUE_DATA_POINT4 = 0x1f,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum DiagnosticFloatSensorIndex {
    CALCULATED_ENGINE_LOAD = 0x0,
    ENGINE_COOLANT_TEMPERATURE = 0x1,
    SHORT_TERM_FUEL_TRIM_BANK1 = 0x2,
    LONG_TERM_FUEL_TRIM_BANK1 = 0x3,
    SHORT_TERM_FUEL_TRIM_BANK2 = 0x4,
    LONG_TERM_FUEL_TRIM_BANK2 = 0x5,
    FUEL_PRESSURE = 0x6,
    INTAKE_MANIFOLD_ABSOLUTE_PRESSURE = 0x7,
    ENGINE_RPM = 0x8,
    VEHICLE_SPEED = 0x9,
    TIMING_ADVANCE = 0xa,
    MAF_AIR_FLOW_RATE = 0xb,
    THROTTLE_POSITION = 0xc,
    OXYGEN_SENSOR1_VOLTAGE = 0xd,
    OXYGEN_SENSOR1_SHORT_TERM_FUEL_TRIM = 0xe,
    OXYGEN_SENSOR1_FUEL_AIR_EQUIVALENCE_RATIO = 0xf,
    OXYGEN_SENSOR2_VOLTAGE = 0x10,
    OXYGEN_SENSOR2_SHORT_TERM_FUEL_TRIM = 0x11,
    OXYGEN_SENSOR2_FUEL_AIR_EQUIVALENCE_RATIO = 0x12,
    OXYGEN_SENSOR3_VOLTAGE = 0x13,
    OXYGEN_SENSOR3_SHORT_TERM_FUEL_TRIM = 0x14,
    OXYGEN_SENSOR3_FUEL_AIR_EQUIVALENCE_RATIO = 0x15,
    OXYGEN_SENSOR4_VOLTAGE = 0x16,
    OXYGEN_SENSOR4_SHORT_TERM_FUEL_TRIM = 0x17,
    OXYGEN_SENSOR4_FUEL_AIR_EQUIVALENCE_RATIO = 0x18,
    OXYGEN_SENSOR5_VOLTAGE = 0x19,
    OXYGEN_SENSOR5_SHORT_TERM_FUEL_TRIM = 0x1a,
    OXYGEN_SENSOR5_FUEL_AIR_EQUIVALENCE_RATIO = 0x1b,
    OXYGEN_SENSOR6_VOLTAGE = 0x1c,
    OXYGEN_SENSOR6_SHORT_TERM_FUEL_TRIM = 0x1d,
    OXYGEN_SENSOR6_FUEL_AIR_EQUIVALENCE_RATIO = 0x1e,
    OXYGEN_SENSOR7_VOLTAGE = 0x1f,
    OXYGEN_SENSOR7_SHORT_TERM_FUEL_TRIM = 0x20,
    OXYGEN_SENSOR7_FUEL_AIR_EQUIVALENCE_RATIO = 0x21,
    OXYGEN_SENSOR8_VOLTAGE = 0x22,
    OXYGEN_SENSOR8_SHORT_TERM_FUEL_TRIM = 0x23,
    OXYGEN_SENSOR8_FUEL_AIR_EQUIVALENCE_RATIO = 0x24,
    FUEL_RAIL_PRESSURE = 0x25,
    FUEL_RAIL_GAUGE_PRESSURE = 0x26,
    COMMANDED_EXHAUST_GAS_RECIRCULATION = 0x27,
    EXHAUST_GAS_RECIRCULATION_ERROR = 0x28,
    COMMANDED_EVAPORATIVE_PURGE = 0x29,
    FUEL_TANK_LEVEL_INPUT = 0x2a,
    EVAPORATION_SYSTEM_VAPOR_PRESSURE = 0x2b,
    CATALYST_TEMPERATURE_BANK1_SENSOR1 = 0x2c,
    CATALYST_TEMPERATURE_BANK2_SENSOR1 = 0x2d,
    CATALYST_TEMPERATURE_BANK1_SENSOR2 = 0x2e,
    CATALYST_TEMPERATURE_BANK2_SENSOR2 = 0x2f,
    ABSOLUTE_LOAD_VALUE = 0x30,
    FUEL_AIR_COMMANDED_EQUIVALENCE_RATIO = 0x31,
    RELATIVE_THROTTLE_POSITION = 0x32,
    ABSOLUTE_THROTTLE_POSITION_B = 0x33,
    ABSOLUTE_THROTTLE_POSITION_C = 0x34,
    ACCELERATOR_PEDAL_POSITION_D = 0x35,
    ACCELERATOR_PEDAL_POSITION_E = 0x36,
    ACCELERATOR_PEDAL_POSITION_F = 0x37,
    COMMANDED_THROTTLE_ACTUATOR = 0x38,
    ETHANOL_FUEL_PERCENTAGE = 0x39,
    ABSOLUTE_EVAPORATION_SYSTEM_VAPOR_PRESSURE = 0x3a,
    SHORT_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK1 = 0x3b,
    SHORT_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK2 = 0x3c,
    SHORT_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK3 = 0x3d,
    SHORT_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK4 = 0x3e,
    LONG_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK1 = 0x3f,
    LONG_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK2 = 0x40,
    LONG_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK3 = 0x41,
    LONG_TERM_SECONDARY_OXYGEN_SENSOR_TRIM_BANK4 = 0x42,
    RELATIVE_ACCELERATOR_PEDAL_POSITION = 0x43,
    HYBRID_BATTERY_PACK_REMAINING_LIFE = 0x44,
    FUEL_INJECTION_TIMING = 0x45,
    ENGINE_FUEL_RATE = 0x46,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VmsMessageType {
    SUBSCRIBE = 0x1,
    SUBSCRIBE_TO_PUBLISHER = 0x2,
    UNSUBSCRIBE = 0x3,
    UNSUBSCRIBE_TO_PUBLISHER = 0x4,
    OFFERING = 0x5,
    AVAILABILITY_REQUEST = 0x6,
    SUBSCRIPTIONS_REQUEST = 0x7,
    AVAILABILITY_RESPONSE = 0x8,
    AVAILABILITY_CHANGE = 0x9,
    SUBSCRIPTIONS_RESPONSE = 0xa,
    SUBSCRIPTIONS_CHANGE = 0xb,
    DATA = 0xc,
    PUBLISHER_ID_REQUEST = 0xd,
    PUBLISHER_ID_RESPONSE = 0xe,
    PUBLISHER_INFORMATION_REQUEST = 0xf,
    PUBLISHER_INFORMATION_RESPONSE = 0x10,
    START_SESSION = 0x11,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum VmsBaseMessageIntegerValuesIndex {
    MESSAGE_TYPE = 0x0,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VmsStartSessionMessageIntegerValuesIndex {
    SERVICE_ID = 0x1,
    CLIENT_ID = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VmsMessageWithLayerIntegerValuesIndex {
    LAYER_TYPE = 0x1,
    LAYER_SUBTYPE = 0x2,
    LAYER_VERSION = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VmsMessageWithLayerAndPublisherIdIntegerValuesIndex {
    PUBLISHER_ID = 0x4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VmsOfferingMessageIntegerValuesIndex {
    PUBLISHER_ID = 0x1,
    NUMBER_OF_OFFERS = 0x2,
    OFFERING_START = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VmsSubscriptionsStateIntegerValuesIndex {
    SEQUENCE_NUMBER = 0x1,
    NUMBER_OF_LAYERS = 0x2,
    NUMBER_OF_ASSOCIATED_LAYERS = 0x3,
    SUBSCRIPTIONS_START = 0x4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VmsAvailabilityStateIntegerValuesIndex {
    SEQUENCE_NUMBER = 0x1,
    NUMBER_OF_ASSOCIATED_LAYERS = 0x2,
    LAYERS_START = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VmsPublisherInformationIntegerValuesIndex {
    PUBLISHER_ID = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum UserFlags {
    NONE = 0x0,
    SYSTEM = 0x1,
    GUEST = 0x2,
    EPHEMERAL = 0x4,
    ADMIN = 0x8,
    DISABLED = 0x10,
    PROFILE = 0x20,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum InitialUserInfoRequestType {
    FIRST_BOOT = 0x1,
    FIRST_BOOT_AFTER_OTA = 0x2,
    COLD_BOOT = 0x3,
    RESUME = 0x4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum InitialUserInfoResponseAction {
    DEFAULT = 0x0,
    SWITCH = 0x1,
    CREATE = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum SwitchUserMessageType {
    LEGACY_ANDROID_SWITCH = 0x1,
    ANDROID_SWITCH = 0x2,
    VEHICLE_RESPONSE = 0x3,
    VEHICLE_REQUEST = 0x4,
    ANDROID_POST_SWITCH = 0x5,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum SwitchUserStatus {
    SUCCESS = 0x1,
    FAILURE = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum CreateUserStatus {
    SUCCESS = 0x1,
    FAILURE = 0x2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum UserIdentificationAssociationType {
    KEY_FOB = 0x1,
    CUSTOM_1 = 0x65,
    CUSTOM_2 = 0x66,
    CUSTOM_3 = 0x67,
    CUSTOM_4 = 0x68,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum UserIdentificationAssociationValue {
    UNKNOWN = 0x1,
    ASSOCIATED_CURRENT_USER = 0x2,
    ASSOCIATED_ANOTHER_USER = 0x3,
    NOT_ASSOCIATED_ANY_USER = 0x4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum UserIdentificationAssociationSetValue {
    ASSOCIATE_CURRENT_USER = 0x1,
    DISASSOCIATE_CURRENT_USER = 0x2,
    DISASSOCIATE_ALL_USERS = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum RotaryInputType {
    ROTARY_INPUT_TYPE_SYSTEM_NAVIGATION = 0x0,
    ROTARY_INPUT_TYPE_AUDIO_VOLUME = 0x1,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum ProcessTerminationReason {
    NOT_RESPONDING = 0x1,
    IO_OVERUSE = 0x2,
    MEMORY_OVERUSE = 0x3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(i32)]
pub enum CustomInputType {
    CUSTOM_EVENT_F1 = 0x3e9,
    CUSTOM_EVENT_F2 = 0x3ea,
    CUSTOM_EVENT_F3 = 0x3eb,
    CUSTOM_EVENT_F4 = 0x3ec,
    CUSTOM_EVENT_F5 = 0x3ed,
    CUSTOM_EVENT_F6 = 0x3ee,
    CUSTOM_EVENT_F7 = 0x3ef,
    CUSTOM_EVENT_F8 = 0x3f0,
    CUSTOM_EVENT_F9 = 0x3f1,
    CUSTOM_EVENT_F10 = 0x3f2,
}

