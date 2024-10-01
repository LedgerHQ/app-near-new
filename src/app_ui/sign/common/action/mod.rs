use crate::app_ui::aliases::{FnCallCappedString, FnCallHexDisplay, U32Buffer};
use crate::{app_ui::fields_writer::FieldsWriter, handlers::common::action::ActionParams, parsing};
use fmt_buffer::Buffer;

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use crate::Instruction;
use ledger_device_sdk::io::Comm;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{
    CenteredInfo, CenteredInfoStyle, InfoButton, InfoLongPress, NbglGenericReview, NbglGlyph,
    NbglPageContent, NbglStatus, TagValueList, TuneIndex,
};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::{
    buttons::ButtonEvent,
    io::Event,
    ui::{
        bitmaps::{CROSSMARK, EYE, VALIDATE_14, WARNING},
        gadgets::{clear_screen, MultiFieldReview},
        layout::{Layout, Location, StringPlace},
        screen_util::screen_update,
    },
};

#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
use numtoa::NumToA;

use super::tx_public_key_context;

mod add_key_common;
mod create_account;
mod delete_account;
mod delete_key;
mod deploy_contract;
mod function_call_bin;
mod function_call_common;
mod function_call_permission;
mod function_call_str;
mod stake;
mod transfer;

pub fn ui_display_transfer(transfer: &parsing::types::Transfer, params: ActionParams) -> bool {
    let mut field_context: transfer::FieldsContext = transfer::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    transfer::format(transfer, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_create_account(
    create_account: &parsing::types::CreateAccount,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();

    create_account::format(create_account, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_delete_account(
    delete_account: &mut parsing::types::DeleteAccount,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();
    let mut field_context: delete_account::FieldsContext = delete_account::FieldsContext::new();

    delete_account::format(delete_account, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_delete_key(delete_key: &parsing::types::DeleteKey, params: ActionParams) -> bool {
    let mut field_context: tx_public_key_context::FieldsContext =
        tx_public_key_context::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    delete_key::format(delete_key, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_stake(stake: &parsing::types::Stake, params: ActionParams) -> bool {
    let mut field_context: stake::FieldsContext = stake::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    stake::format(stake, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

/// action type (1) + Public Key (1) + Access Key Nonce (1) +
/// Access Permission (1)
const ADD_KEY_FULL_ACCESS_MAX_FIELDS: usize = 4;

pub fn ui_display_add_key_fullaccess(
    add_key: &parsing::types::AddKey,
    params: ActionParams,
) -> bool {
    let mut field_context: add_key_common::FieldsContext = add_key_common::FieldsContext::new();
    let mut writer: FieldsWriter<'_, ADD_KEY_FULL_ACCESS_MAX_FIELDS> = FieldsWriter::new();

    add_key_common::format(add_key, &mut field_context, &mut writer, "Full Access");

    ui_display_common(&mut writer, params)
}

pub fn ui_display_add_key_functioncall(
    add_key: &parsing::types::AddKey,
    function_call_per: &mut parsing::types::FunctionCallPermission,
    params: ActionParams,
) -> bool {
    let mut common_field_context: add_key_common::FieldsContext =
        add_key_common::FieldsContext::new();
    let mut func_call_field_context: function_call_permission::FieldsContext =
        function_call_permission::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    add_key_common::format(
        add_key,
        &mut common_field_context,
        &mut writer,
        "Function Call",
    );
    function_call_permission::format(function_call_per, &mut func_call_field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_deploy_contract(
    deploy_contract: &parsing::types::DeployContract,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();

    deploy_contract::format(deploy_contract, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_function_call_str(
    func_call_common: &mut parsing::types::FunctionCallCommon,
    args: &mut FnCallCappedString,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();
    let mut common_field_context: function_call_common::FieldsContext =
        function_call_common::FieldsContext::new();

    function_call_common::format(func_call_common, &mut common_field_context, &mut writer);
    let mut args_field_context: function_call_str::FieldsContext =
        function_call_str::FieldsContext::new();
    function_call_str::format(args, &mut args_field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_function_call_bin(
    func_call_common: &mut parsing::types::FunctionCallCommon,
    args: &FnCallHexDisplay,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();
    let mut common_field_context: function_call_common::FieldsContext =
        function_call_common::FieldsContext::new();

    function_call_common::format(func_call_common, &mut common_field_context, &mut writer);
    let mut args_field_context: function_call_bin::FieldsContext =
        function_call_bin::FieldsContext::new();
    function_call_bin::format(args, &mut args_field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_delegate_error(#[allow(unused)] comm: &mut Comm) {
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        clear_screen();

        // Add icon and text to match the C SDK equivalent.
        if cfg!(target_os = "nanos") {
            "Sign delegate action".place(Location::Custom(2), Layout::Centered, true);
            "not supported...".place(Location::Custom(14), Layout::Centered, true);
        } else {
            WARNING.draw(57, 10);
            "Sign delegate action".place(Location::Custom(28), Layout::Centered, true);
            "not supported...".place(Location::Custom(42), Layout::Centered, true);
        }

        screen_update();
        loop {
            {
                if let Event::Button(ButtonEvent::BothButtonsRelease) =
                    comm.next_event::<Instruction>()
                {
                    return;
                };
            }
        }
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        const NEAR_LOGO: NbglGlyph =
            NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));

        let info_button = InfoButton::new(
            "Delegate action is not supported",
            Some(&NEAR_LOGO),
            "Reject",
            TuneIndex::Success,
        );

        let review: NbglGenericReview =
            NbglGenericReview::new().add_content(NbglPageContent::InfoButton(info_button));

        let res = review.show("Reject");
        let status: NbglStatus = NbglStatus::new();
        match res {
            true => {
                status.text("Transaction rejected").show(true);
            }
            false => {
                status.text("Transaction rejected").show(false);
            }
        }
    }
}

pub fn ui_display_common<const N: usize>(
    writer: &mut FieldsWriter<'_, N>,
    params: ActionParams,
) -> bool {
    let mut ordinal_fmt_buf = OrdinalStringBuffer::new();
    let is_last = ordinal_string(&mut ordinal_fmt_buf, params);

    let msg_before = ordinal_fmt_buf.as_str();

    let next_msg = if params.is_nested_delegate {
        "Next Subaction"
    } else {
        "Next Action"
    };

    let last_msg = if params.is_nested_delegate {
        "To NEP366 suffix"
    } else {
        "Sign"
    };

    let msg_after = if is_last { last_msg } else { next_msg };

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let binding = [msg_before];

        let my_review = MultiFieldReview::new(
            writer.get_fields(),
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
        let tag_values_list = TagValueList::new(writer.get_fields(), 2, false, false);

        let info_button = InfoButton::new(
            msg_after,
            Some(&NEAR_LOGO),
            "Confirm action",
            TuneIndex::Success,
        );

        let info_longpress = InfoLongPress::new(
            msg_after,
            Some(&NEAR_LOGO),
            "Hold to sign",
            TuneIndex::Error,
        );

        let mut review: NbglGenericReview = NbglGenericReview::new()
            .add_content(NbglPageContent::CenteredInfo(centered_info))
            .add_content(NbglPageContent::TagValueList(tag_values_list));

        let last_screen: &str;

        if is_last && !params.is_nested_delegate {
            review = review.add_content(NbglPageContent::InfoLongPress(info_longpress));
            last_screen = "Transaction signed";
        } else {
            review = review.add_content(NbglPageContent::InfoButton(info_button));
            last_screen = "Action confirmed";
        }

        let res = review.show("Reject");
        let status: NbglStatus = NbglStatus::new();
        match res {
            true => {
                status.text(last_screen).show(true);
            }
            false => {
                status.text("Transaction rejected").show(false);
            }
        }
        res
    }
}

/// a buffer, large enough to fit description string and
/// 2 u32 numbers as strings
type OrdinalStringBuffer = Buffer<40>;

fn ordinal_string(fmt_buf: &mut OrdinalStringBuffer, params: ActionParams) -> bool {
    let mut num_out = U32Buffer::default();
    let header = if params.is_nested_delegate {
        "View subaction "
    } else {
        "View action "
    };
    fmt_buf.write_str(header);
    // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
    fmt_buf.write_str(params.ordinal_action.numtoa_str(10, &mut num_out));
    fmt_buf.write_str(" / ");
    // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
    fmt_buf.write_str(params.total_actions.numtoa_str(10, &mut num_out));

    params.ordinal_action == params.total_actions
}
