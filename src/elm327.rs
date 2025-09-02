use anyhow::{Context, Result};
use log::trace;

use crate::command::OBDResponse;
use crate::error::ReadObdError;

/// Take the hex byte string from the ELM327 response and put it into an OBDResponse, which is basically
/// the data byte values that can be used in the formulas to get the final display value.
///
/// The byte string is assumed be space separated pairs with the leading number of [`header_byte_count`]
/// bytes as the header. Multi frame is handled (code 1), but flow control is not (code 3).
pub fn to_values(header_byte_count: usize, response: &str) -> Result<OBDResponse> {
    if response.len() % 3 != 0 {
        return Err(ReadObdError::BadCommandResponseLength {
            response: response.into(),
            length: response.len(),
        })
        .with_context(|| format!("No multiple of 3 - ({response})"));
    };

    if response.len() < 24 {
        // 8 bytes with spaces
        return Err(ReadObdError::BadCommandResponseLength {
            response: response.into(),
            length: response.len(),
        })
        .context("Less than 8 bytes");
    };

    let mut frames = response.split_whitespace();

    // first are can Id
    let can_id: String = frames.by_ref().take(header_byte_count).collect();

    // Then 8 bytes of 'data'
    let mut data_count = 8;

    // First byte in data is frame type code & size
    data_count -= 1;
    let code_size = frames
        .next()
        .ok_or(ReadObdError::BadCommandResponseLength {
            response: response.into(),
            length: response.len(),
        })
        .context("Missing code/size")?;

    if code_size.starts_with('3') {
        return Err(ReadObdError::BadCommandResponseUnsupported {
            response: response.into(),
        })
        .context("Flow control not supported");
    }

    let multi_frame = code_size.starts_with('1');

    let size = if !multi_frame {
        hex_to_u8(code_size)?
    } else {
        // multi frame uses second code size char + next byte for size
        data_count -= 1;

        hex_to_u8(
            &(String::from(
                code_size
                    .chars()
                    .nth(1)
                    .ok_or(ReadObdError::BadCommandResponseLength {
                        response: response.into(),
                        length: response.len(),
                    })
                    .context("Missing code/size second char")?,
            ) + frames
                .next()
                .ok_or(ReadObdError::BadCommandResponseLength {
                    response: response.into(),
                    length: response.len(),
                })
                .context("Missing size")?),
        )?
    };

    // mode
    data_count -= 1;
    let mode: String = frames
        .next()
        .ok_or(ReadObdError::BadCommandResponseLength {
            response: response.into(),
            length: response.len(),
        })
        .context("Missing mode")?
        .into();

    // pid
    let pid: String = match mode.as_str() {
        "41" | "61" => {
            data_count -= 1;
            Ok(frames
                .next()
                .ok_or(ReadObdError::BadCommandResponseLength {
                    response: response.into(),
                    length: response.len(),
                })
                .context("Missing pid")?
                .into())
        }
        "62" => {
            data_count -= 2;
            Ok(String::from(
                frames
                    .next()
                    .ok_or(ReadObdError::BadCommandResponseLength {
                        response: response.into(),
                        length: response.len(),
                    })
                    .context("Missing pid byte 1")?,
            ) + frames
                .next()
                .ok_or(ReadObdError::BadCommandResponseLength {
                    response: response.into(),
                    length: response.len(),
                })
                .context("Missing pid byte 2")?)
        }
        _ => Err(ReadObdError::BadCommandResponseUnsupported {
            response: response.into(),
        })
        .context("Unsupported mode "),
    }?;

    let mut obd_response = OBDResponse::new(can_id, size, mode, pid);

    // the rest of the data count is data
    while data_count > 0 {
        data_count -= 1;

        match frames.next() {
            Some(data) => obd_response.add(hex_to_u8(data)?),
            None => break,
        };
    }

    // and multi frame has consecutive frames
    if multi_frame {
        'CF: loop {
            // first is can Id again
            let can_id: String = frames.by_ref().take(header_byte_count).collect();

            if can_id.is_empty() {
                break 'CF;
            }

            if can_id != obd_response.can_id {
                return Err(ReadObdError::BadCommandResponseLength {
                    response: response.into(),
                    length: response.len(),
                })
                .with_context(|| {
                    format!(
                        "CF can_id ({can_id}) does not match first frame ({})",
                        obd_response.can_id
                    )
                });
            }

            data_count = 8;

            // First byte in data is code & index
            data_count -= 1;
            let code_size = frames
                .next()
                .ok_or(ReadObdError::BadCommandResponseLength {
                    response: response.into(),
                    length: response.len(),
                })
                .context("Missing code/index")?;

            if !code_size.starts_with('2') {
                return Err(ReadObdError::BadCommandResponseUnsupported {
                    response: response.into(),
                })
                .with_context(|| format!("Consecutive Flow control code not 2? ({code_size})"));
            }

            // the rest of the data count is data
            while data_count > 0 {
                data_count -= 1;

                match frames.next() {
                    Some(data) => obd_response.add(hex_to_u8(data)?),
                    None => break 'CF,
                };
            }
        }
    }

    trace!("OBDResponse ({obd_response:?})");

    Ok(obd_response)
}

fn hex_to_u8(hex: &str) -> Result<u8> {
    match u8::from_str_radix(hex, 16) {
        Ok(v) => Ok(v),
        Err(err) => Err(ReadObdError::BadCommandResponseHex {
            response: "".into(),
            hex_str: hex.into(),
            source: err,
        }
        .into()),
    }
}
