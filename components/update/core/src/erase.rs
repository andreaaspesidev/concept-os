use crate::messages::*;
use crate::utils::*;
use storage_api::*;
use uart_channel_api::*;
use userlib::flash::BlockType;

pub fn component_erase(
    channel: &mut UartChannel
) -> Result<(), MessageError> {
    // Ask component ID
    channel_write_single(
        channel,
        ComponentEraseCommand::SendComponentID as u8,
    )?;
    // Read component ID
    let mut component_id_buff: [u8; ComponentIDPacket::get_size()] =
        [0x00; ComponentIDPacket::get_size()];
    channel_read(channel, &mut component_id_buff)?;
    // Validate
    let component_id_pkt = ComponentIDPacket::from(&component_id_buff)?;
    // Search the component
    let mut storage = Storage::new();
    let search_result = search_component(
        component_id_pkt.get_component_id(),
        Some(component_id_pkt.get_component_version()),
        &storage,
    );
    if search_result.is_err() {
        return match search_result.unwrap_err() {
            SearchError::CannotFindComponent => {
                channel_write_single(
                    channel,
                    ComponentEraseResponse::CannotFindComponent as u8,
                )?;
                Err(MessageError::CannotFindComponent)
            }
            SearchError::CannotFindVersion => {
                channel_write_single(
                    channel,
                    ComponentEraseResponse::CannotFindVersion as u8,
                )?;
                Err(MessageError::CannotFindVersion)
            },
            SearchError::FlashError => {
                channel_write_single(
                    channel,
                    ComponentEraseResponse::GenericFailure as u8,
                )?;
                Err(MessageError::FlashError)
            }
        };
    }
    // TODO: Dependencies checks

    // TODO: Disable component

    // Erase block
    if storage
        .deallocate_block(search_result.unwrap().block_base_address)
        .is_err()
    {
        // Respond
        channel_write_single(
            channel,
            ComponentEraseResponse::GenericFailure as u8,
        )?;
        return Err(MessageError::FlashError);
    }
    // Respond
    channel_write_single(channel, ComponentEraseResponse::Success as u8 as u8)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum SearchError {
    CannotFindComponent,
    CannotFindVersion,
    FlashError,
}

fn search_component(
    component_id: u16,
    component_version: Option<u32>,
    storage: &Storage,
) -> Result<GetNthBlockResponse, SearchError> {
    // Get block stats
    let status_res = storage.report_status();
    if status_res.is_err() {
        return Err(SearchError::FlashError);
    }
    let status = status_res.unwrap();
    let mut component_found: bool = false;
    // Iterate all blocks
    for block_num in 0..status.blocks {
        // Get block
        let block = storage.get_nth_block(block_num).unwrap();

        if block.block_type == BlockType::COMPONENT {
            // Read hbf header
            let mut buff: [u8; hbf_lite::HBF_HEADER_MIN_SIZE] =
                [0x00; hbf_lite::HBF_HEADER_MIN_SIZE];
            if storage
                .read_stream(block.block_base_address, 0, &mut buff)
                .is_err()
            {
                return Err(SearchError::FlashError);
            }
            // Try parse it
            let reader = hbf_lite::BufferReaderImpl::from(&buff);
            let hbf_res = hbf_lite::HbfFile::from_reader(&reader);
            if hbf_res.is_ok() {
                let hbf = hbf_res.unwrap();
                // Check if the id is correct
                let hbf_base_res = hbf.header_base();
                if hbf_base_res.is_err() {
                    continue; // Skip this block
                }
                let hbf_base = hbf_base_res.unwrap();
                // Compare component id
                if hbf_base.component_id() == component_id {
                    component_found = true;
                    if component_version.is_some() {
                        // Compare version
                        if hbf_base.component_version()
                            == component_version.unwrap()
                        {
                            return Ok(block);
                        }
                    } else {
                        return Ok(block);
                    }
                }
            }
        }
    }
    // No block found, return none
    return match component_found {
        true => Err(SearchError::CannotFindVersion),
        false => Err(SearchError::CannotFindComponent),
    };
}
