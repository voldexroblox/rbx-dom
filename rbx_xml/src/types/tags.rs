use std::io::{Read, Write};

use rbx_dom_weak::types::{Ref, Tags, Variant};

use crate::{
    deserializer::ParseState,
    deserializer_core::{XmlEventReader, XmlReadEvent},
    error::{DecodeError, DecodeErrorKind},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    EncodeError,
};

pub const XML_TAG_NAME: &str = "Tags";

pub fn write_tags<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &Tags,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;

    for tag in value.iter() {
        writer.write(XmlWriteEvent::start_element("tag"))?;
        writer.write_string(tag)?;
        writer.write(XmlWriteEvent::end_element())?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn read_tags<R: Read>(
    reader: &mut XmlEventReader<R>,
    _state: &mut ParseState,
    _instance_id: Ref,
) -> Result<Variant, DecodeError> {
    let mut tags = Tags::new();

    reader.expect_start_with_name(XML_TAG_NAME)?;

    loop {
        match reader.expect_peek()? {
            XmlReadEvent::StartElement { name, .. } => {
                if name.local_name != "tag" {
                    let err = DecodeErrorKind::UnexpectedXmlEvent(reader.expect_next()?);
                    return Err(reader.error(err));
                }

                let tag_name = reader.read_tag_contents("tag")?;
                tags.push(&tag_name);
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

    Ok(Variant::Tags(tags))
}
