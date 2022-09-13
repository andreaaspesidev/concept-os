use crate::messages::*;
use crate::utils::*;
use storage_api::*;
use uart_channel_api::*;

pub fn component_erase(channel: &mut UartChannel) -> Result<(), MessageError> {
    // Ask component ID
    channel_write_single(channel, ComponentEraseCommand::SendComponentID as u8)?;
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
                channel_write_single(channel, ComponentEraseResponse::CannotFindComponent as u8)?;
                Err(MessageError::CannotFindComponent)
            }
            SearchError::CannotFindVersion => {
                channel_write_single(channel, ComponentEraseResponse::CannotFindVersion as u8)?;
                Err(MessageError::CannotFindVersion)
            }
            SearchError::FlashError => {
                channel_write_single(channel, ComponentEraseResponse::GenericFailure as u8)?;
                Err(MessageError::FlashError)
            }
        };
    }
    // TODO: Dependencies checks

    // TODO: Disable component

    // Erase block
    if storage
        .deallocate_block(search_result.unwrap().block.block_base_address)
        .is_err()
    {
        // Respond
        channel_write_single(channel, ComponentEraseResponse::GenericFailure as u8)?;
        return Err(MessageError::FlashError);
    }
    // Respond
    channel_write_single(channel, ComponentEraseResponse::Success as u8 as u8)?;
    Ok(())
}
