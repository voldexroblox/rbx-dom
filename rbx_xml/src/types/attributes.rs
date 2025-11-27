use std::io::{Read, Write};

use rbx_dom_weak::types::{Attributes, Ref, Variant};

use crate::{
    core::XmlType,
    deserializer::ParseState,
    deserializer_core::{XmlEventReader, XmlReadEvent},
    error::{DecodeError, DecodeErrorKind, EncodeErrorKind},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    types::read_value_xml,
    EncodeError,
};

pub const XML_TAG_NAME: &str = "Attributes";

pub fn write_attributes<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    attributes: &Attributes,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;

    for (attr_name, attr_value) in attributes.iter() {
        match attr_value {
            Variant::Bool(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::BrickColor(v) => (*v as i32).write_outer_xml(attr_name, writer)?,
            Variant::Color3(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::ColorSequence(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Enum(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Float32(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Float64(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Int32(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::NumberRange(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::NumberSequence(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Rect(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::String(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::BinaryString(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::CFrame(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::UDim(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::UDim2(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Vector2(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Vector3(v) => v.write_outer_xml(attr_name, writer)?,
            Variant::Font(v) => v.write_outer_xml(attr_name, writer)?,
            _ => return Err(writer.error(EncodeErrorKind::UnsupportedPropertyType(attr_value.ty()))),
        }
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn read_attributes<R: Read>(
    reader: &mut XmlEventReader<R>,
    state: &mut ParseState,
    instance_id: Ref,
) -> Result<Variant, DecodeError> {
    let mut attributes = Attributes::new();

    reader.expect_start_with_name(XML_TAG_NAME)?;

    loop {
        match reader.expect_peek()? {
            XmlReadEvent::StartElement { name, attributes: xml_attributes, .. } => {
                let xml_type_name = name.local_name.to_owned();

                let mut attr_name = None;
                for attribute in xml_attributes {
                    if attribute.name.local_name.as_str() == "name" {
                        attr_name = Some(attribute.value.to_owned());
                        break;
                    }
                }

                let attr_name = match attr_name {
                    Some(value) => value,
                    None => return Err(reader.error(DecodeErrorKind::MissingAttribute("name"))),
                };

                if let Some(value) = read_value_xml(reader, state, &xml_type_name, instance_id, &attr_name)? {
                    attributes.insert(attr_name, value);
                }
            }
            XmlReadEvent::EndElement { name } => {
                if name.local_name == XML_TAG_NAME {
                    reader.expect_next()?;
                    break;
                } else {
                    let err = DecodeErrorKind::UnexpectedXmlEvent(reader.expect_next()?);
                    return Err(reader.error(err));
                }
            }
            _ => {
                let err = DecodeErrorKind::UnexpectedXmlEvent(reader.expect_next()?);
                return Err(reader.error(err));
            }
        }
    }

    Ok(Variant::Attributes(attributes))
}
