// SPDX-FileCopyrightText: Red Hat, Inc.
// SPDX-License-Identifier: GPL-2.0-or-later

// This file is generated by gen_vhal_const.py. Do not edit
// .hal file is parsed by hidl_parser.py
// Source file copied from hardware/interfaces/automotive/vehicle/2.0/types.hal
// File generated on: ${time}

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use num_enum::TryFromPrimitive;
% for group_name in data['enums']:

<% enum_object = data['enums'][group_name] %>\
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
% if group_name == 'VehicleProperty':
#[derive(Hash)]
% endif
% if group_name == 'VehiclePropertyGroup' or group_name == 'VehicleVendorPermission':
#[derive(TryFromPrimitive)]
#[repr(u32)]
% elif "int32" in str(enum_object.header):
#[derive(TryFromPrimitive)]
#[repr(i32)]
% endif
#[non_exhaustive]
pub enum ${group_name} {
<% values = [] %>\
% for case in enum_object.cases:
<% case_val = case.value.resolve(enum_object, data) %>\
% if case_val not in values:
    ${case.name} = ${hex(case_val)},
<% values.append(case_val) %>\
% endif
% endfor
}
% if group_name == 'VehiclePropertyType':

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
% endif
% endfor