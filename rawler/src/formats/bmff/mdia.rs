// SPDX-License-Identifier: MIT
// Copyright 2020 Alfred Gutierrez
// Copyright 2021 Daniel Vogelbacher <daniel@chaospixel.com>

use super::{BmffError, BoxHeader, FourCC, ReadBox, Result, hdlr::HdlrBox, mdhd::MdhdBox, minf::MinfBox, vendor::VendorBox};
use log::debug;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MdiaBox {
  pub header: BoxHeader,
  pub mdhd: MdhdBox,
  pub hdlr: HdlrBox,
  pub minf: MinfBox,
  pub vendor: Vec<VendorBox>,
}

impl MdiaBox {
  pub const TYP: FourCC = FourCC::with(['m', 'd', 'i', 'a']);
}

impl<R: Read + Seek> ReadBox<&mut R> for MdiaBox {
  fn read_box(mut reader: &mut R, header: BoxHeader) -> Result<Self> {
    let mut mdhd = None;
    let mut hdlr = None;
    let mut minf = None;

    let mut vendors = Vec::new();

    let mut current = reader.stream_position()?;

    while current < header.end_offset() {
      // get box?

      let header = BoxHeader::parse(&mut reader)?;

      //let ftyp = Some(FtypBox::read_box(&mut file, header)?);

      match header.typ {
        MdhdBox::TYP => {
          mdhd = Some(MdhdBox::read_box(&mut reader, header)?);
        }
        HdlrBox::TYP => {
          hdlr = Some(HdlrBox::read_box(&mut reader, header)?);
        }
        MinfBox::TYP => {
          minf = Some(MinfBox::read_box(&mut reader, header)?);
        }

        _ => {
          debug!("Vendor box found in mdia: {:?}", header.typ);
          let vendor = VendorBox::read_box(&mut reader, header)?;
          vendors.push(vendor);
        }
      }

      current = reader.stream_position()?;
    }

    reader.seek(SeekFrom::Start(header.end_offset()))?;

    Ok(Self {
      header,
      mdhd: mdhd.ok_or_else(|| BmffError::Parse("mdhd box not found, corrupt file?".into()))?,
      hdlr: hdlr.ok_or_else(|| BmffError::Parse("hdlr box not found, corrupt file?".into()))?,
      minf: minf.ok_or_else(|| BmffError::Parse("minf box not found, corrupt file?".into()))?,
      vendor: vendors,
    })
  }
}
