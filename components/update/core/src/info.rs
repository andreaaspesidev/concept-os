use hbf_lite::HbfFile;
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
    let status_res = storage.report_status();
    if status_res.is_err() {
        return Err(MessageError::FlashError);
    }
    let status = status_res.unwrap();
    // Iterate all blocks
    for block_num in 0..status.blocks {
        // Get block
        let block = storage.get_nth_block(block_num).unwrap();
        if block.block_type == BlockType::COMPONENT {
            // Read hbf
            let flash_reader = FlashReader::from(
                block.block_base_address,
                block.block_size,
            );
            let hbf_res = HbfFile::from_reader(&flash_reader);
            if hbf_res.is_ok() {
                let hbf = hbf_res.unwrap();
                // Read needed fields
                let hbf_base = wrap_hbf_error(hbf.header_base())?;
                let hbf_main = wrap_hbf_error(hbf.header_main())?;
                // Generate message
                let mut component_status: ComponentStatus = ComponentStatus::NONE;
                if wrap_hbf_error(hbf.validate())? {
                    component_status |= ComponentStatus::HBF_VALID;
                }
                let msg = ComponentInfoMessage::new(
                    hbf_base.component_id(),
                    hbf_base.component_version(),
                    block.block_size,
                    hbf_main.component_min_ram(),
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
