# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

from typing import List, Union

import generator.model as model

from .rust_commons import TypeData, generate_extras
from .rust_lang_utils import indent_lines, lines_to_doc_comments, to_snake_case, to_upper_camel_case


def _get_enum_docs(enum: Union[model.Enum, model.EnumItem]) -> List[str]:
    doc = enum.documentation.splitlines(keepends=False) if enum.documentation else []
    return lines_to_doc_comments(doc)


def generate_serde(enum: model.Enum) -> List[str]:
    ser = [
        f"impl Serialize for {enum.name} {{",
        "fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer,{",
        "match self {",
    ]

    de = [
        f"impl<'de> Deserialize<'de> for {enum.name} {{",
        f"fn deserialize<D>(deserializer: D) -> Result<{enum.name}, D::Error> where D: serde::Deserializer<'de>,"
        "{",
        "let value = i32::deserialize(deserializer)?;",
        "match value {",
    ]
    for item in enum.values:
        full_name = f"{enum.name}::{to_upper_camel_case(item.name)}"
        ser += [f"{full_name} => serializer.serialize_i32({item.value}),"]
        de += [f"{item.value} => Ok({full_name}),"]
    ser += [
        "}",  # match
        "}",  # fn
        "}",  # impl
    ]
    de += [
        '_ => Err(serde::de::Error::custom("Unexpected value"))',
        "}",  # match
        "}",  # fn
        "}",  # impl
    ]
    return ser + de


def generate_enum(enum: model.Enum, types: TypeData) -> None:
    is_int = all(isinstance(item.value, int) for item in enum.values)

    lines = _get_enum_docs(enum) + generate_extras(enum)
    if is_int and not enum.supportsCustomValues:
        lines += ["#[derive(PartialEq, Debug, Eq, Clone)]"]
    else:
        lines += ["#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]"]


    if enum.supportsCustomValues:
        # This is an open-ended enumeration, which we represent as a newtype with a bunch of associated constants.
        if enum.type.name == "uinteger":
            typ = "u32"
        elif enum.type.name == "integer":
            typ = "i32"
        elif enum.type.name == "string":
            typ = "std::borrow::Cow<'static, str>"
        else:
            raise ValueError(f"Unsupported type for open-ended enumeration: {enum.type}")

        lines += [f"pub struct {enum.name}({typ});"]
        lines += [f"impl {enum.name} {{"]
        for item in enum.values:
            value = f"std::borrow::Cow::Borrowed(\"{item.value}\")" if enum.type.name == "string" else item.value
            lines += [f"    pub const {to_snake_case(item.name).upper()}: Self = Self({value});"]
        lines += ["}"]


    else:
        lines += [f"pub enum {enum.name} {{"]
        for item in enum.values:
            if is_int:
                field = [
                    f"{to_upper_camel_case(item.name)} = {item.value},",
                ]
            else:
                field = [
                    f'#[serde(rename = "{item.value}")]',
                    f"{to_upper_camel_case(item.name)},",
                ]

            lines += indent_lines(
                _get_enum_docs(item) + generate_extras(item) + field + [""]
            )

        lines += ["}"]

        if is_int:
            lines += generate_serde(enum)

    types.add_type_info(enum, enum.name, lines)


def generate_enums(enums: List[model.Enum], types: TypeData) -> None:
    for enum in enums:
        generate_enum(enum, types)
