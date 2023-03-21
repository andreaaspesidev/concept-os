// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use cbf_lite::CbfFile;
use storage_api::*;
use uart_channel_api::*;
use userlib::flash::BlockType;

use crate::{messages::*, utils::*};

pub fn system_info(
    channel: &mut UartChannel
) -> Result<(), MessageError> {
    // Get storage
    let storage = Storage::new();
    // Get block stats
    let status = storage.report_status().map_err(|_| MessageError::FlashError)?;
    // Iterate all blocks
    for block_num in 0..status.blocks {
        // Get block
        let block = storage.get_nth_block(block_num).unwrap();
        if block.block_type == BlockType::COMPONENT {
            // Read cbf
            let flash_reader = FlashReader::from(
                block.block_base_address,
                block.block_size,
            );
            let cbf_res = CbfFile::from_reader(&flash_reader);
            if cbf_res.is_ok() {
                let cbf = cbf_res.unwrap();
                // Read needed fields
                let cbf_base = wrap_cbf_error(cbf.header_base())?;
                let cbf_main = wrap_cbf_error(cbf.header_main())?;
                // Generate message
                let mut component_status: ComponentStatus = ComponentStatus::NONE;
                if wrap_cbf_error(cbf.validate())? {
                    component_status |= ComponentStatus::CBF_VALID;
                }
                let msg = ComponentInfoMessage::new(
                    cbf_base.component_id(),
                    cbf_base.component_version(),
                    block.block_size,
                    cbf_main.component_min_ram(),
                    component_status,
                );
                // Send message
                let mut buff: [u8; ComponentInfoMessage::get_size()] = [0x00; ComponentInfoMessage::get_size()];
                msg.write_to_buffer(&mut buff);
                channel_write(channel, &buff)?;
            }
        }
    }

    channel_write(channel, &NO_MORE_COMPONENTS.to_le_bytes())?;

    Ok(())
}
