#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{
    CenteredInfo, CenteredInfoStyle, Field, InfoButton, NbglGenericReview, NbglGlyph,
    NbglPageContent, NbglStatus, TagValueList, TuneIndex,
};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};
use numtoa::NumToA;

use crate::{
    app_ui::{aliases::U32Buffer, fields_writer::FieldsWriter},
    parsing,
    utils::types::elipsis_fields::{ElipsisFields, EllipsisBuffer},
};

struct FieldsContext {
    display_buf1: EllipsisBuffer,
    display_buf2: EllipsisBuffer,
    numtoa_buf: U32Buffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            display_buf1: EllipsisBuffer::default(),
            display_buf2: EllipsisBuffer::default(),
            numtoa_buf: U32Buffer::default(),
        }
    }
}

/// Signer Id (1-2) + Receiver Id (1-2) + Total actions (1)
const MAX_FIELDS: usize = 5;

fn format<'b, 'a: 'b>(
    prefix: &'b mut parsing::types::transaction::prefix::Prefix,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let signer_id = ElipsisFields::from_capped_string(
        &mut prefix.signer_id,
        "Signer Id",
        &mut field_context.display_buf1,
    );
    writer.push_fields(signer_id);

    let receiver_id = ElipsisFields::from_capped_string(
        &mut prefix.receiver_id,
        "Receiver Id",
        &mut field_context.display_buf2,
    );
    writer.push_fields(receiver_id);

    let num_actions_str = prefix
        .number_of_actions
        // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
        .numtoa_str(10, &mut field_context.numtoa_buf);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Total actions",
        value: num_actions_str,
    }));
}
pub fn ui_display(prefix: &mut parsing::types::transaction::prefix::Prefix) -> bool {
    let mut field_writer = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(prefix, &mut field_context, &mut field_writer);

    let msg_before = "View header";
    let msg_after = "Continue to actions";

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let binding = [msg_before];

        let my_review = MultiFieldReview::new(
            field_writer.get_fields(),
            &binding,
            Some(&EYE),
            msg_after,
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

        let centered_info = CenteredInfo::new(
            msg_before,
            "",
            "",
            Some(&NEAR_LOGO),
            false,
            CenteredInfoStyle::LargeCaseBoldInfo,
            0,
        );

        let info_button = InfoButton::new(
            msg_after,
            Some(&NEAR_LOGO),
            "Confirm header",
            TuneIndex::Success,
        );

        let tag_values_list = TagValueList::new(field_writer.get_fields(), 2, false, false);

        let review: NbglGenericReview = NbglGenericReview::new()
            .add_content(NbglPageContent::CenteredInfo(centered_info))
            .add_content(NbglPageContent::TagValueList(tag_values_list))
            .add_content(NbglPageContent::InfoButton(info_button));

        let res = review.show("Reject");
        let status: NbglStatus = NbglStatus::new();
        match res {
            true => {
                status.text("Header confirmed").show(true);
            }
            false => {
                status.text("Transaction rejected").show(false);
            }
        }
        res
    }
}
