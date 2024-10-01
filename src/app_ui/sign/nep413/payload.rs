use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing::types::nep413::payload::Payload,
    utils::types::elipsis_fields::{ElipsisFields, EllipsisBuffer},
};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{Field, NbglGlyph, NbglReview, NbglReviewStatus, StatusType};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};

/// length, twice as long as [crate::parsing::types::nep413::payload::NonceBuffer],
/// sufficient to store its representation as hexadecimal string.
/// NOTE: arrays only implement [Default] up to 32 in size
const NONCE_HEX_LENGTH: usize = 64;

struct FieldsContext {
    msg_display_buf: EllipsisBuffer,
    nonce_buffer: [u8; NONCE_HEX_LENGTH],
    recipient_display_buf: EllipsisBuffer,
    callback_url_display_buf: EllipsisBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            msg_display_buf: EllipsisBuffer::default(),
            nonce_buffer: [0u8; NONCE_HEX_LENGTH],
            recipient_display_buf: EllipsisBuffer::default(),
            callback_url_display_buf: EllipsisBuffer::default(),
        }
    }
}

/// Message `ElipsisFields` (1-2) + Nonce (1) +
/// Recipient `ElipsisFields` (1-2) + Callback Url (1-2)
const MAX_FIELDS: usize = 7;

fn format<'b, 'a: 'b>(
    payload: &'b mut Payload,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let message_fields = ElipsisFields::from_capped_string(
        &mut payload.message,
        "Message",
        &mut field_context.msg_display_buf,
    );
    writer.push_fields(message_fields);

    // .unwrap() is ok, as `64 == 32 * 2` holds true
    hex::encode_to_slice(payload.nonce, &mut field_context.nonce_buffer).unwrap();
    writer.push_fields(ElipsisFields::one(Field {
        name: "Nonce",
        // .unwrap() is ok, as buffer contains only bytes, encoding hex chars
        value: core::str::from_utf8(&field_context.nonce_buffer).unwrap(),
    }));

    let recipient_fields = ElipsisFields::from_capped_string(
        &mut payload.recipient,
        "Recipient",
        &mut field_context.recipient_display_buf,
    );
    writer.push_fields(recipient_fields);

    if let Some(callback_url) = payload.callback_url.as_mut() {
        let callback_url_fields = ElipsisFields::from_capped_string(
            callback_url,
            "Callback Url",
            &mut field_context.callback_url_display_buf,
        );
        writer.push_fields(callback_url_fields);
    }
}
pub fn ui_display(payload: &mut Payload) -> bool {
    let mut field_writer = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(payload, &mut field_context, &mut field_writer);

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_review = MultiFieldReview::new(
            field_writer.get_fields(),
            &["View NEP413 msg sign"],
            Some(&EYE),
            "Sign",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );

        my_review.show()
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        const NEAR_LOGO: NbglGlyph =
            NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));

        let review: NbglReview = NbglReview::new()
            .titles("Review NEP413 msg sign", "", "Sign message")
            .glyph(&NEAR_LOGO);

        let res = review.show(field_writer.get_fields());
        let status = NbglReviewStatus::new();
        match res {
            true => {
                status.status_type(StatusType::Message).show(true);
            }
            false => {
                status.status_type(StatusType::Message).show(false);
            }
        }
        res
    }
}
