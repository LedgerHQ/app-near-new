use crate::{
    utils::crypto::{public_key::NoSecpAllowed, PathBip32, PublicKeyBe},
    AppSW,
};
use fmt_buffer::Buffer;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
use ledger_device_sdk::ecc::Ed25519;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{
    CenteredInfo, CenteredInfoStyle, Field, InfoButton, NbglGenericReview, NbglGlyph,
    NbglPageContent, NbglStatus, TagValueList, TuneIndex,
};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE},
    gadgets::{Field, MultiFieldReview},
};

pub fn validate(
    tx_public_key: Result<PublicKeyBe, NoSecpAllowed>,
    path: &PathBip32,
) -> Result<(), AppSW> {
    let matching_private_key = {
        let pk = Ed25519::derive_from_path_slip10(&path.0)
            .public_key()
            .map_err(|_| AppSW::KeyDeriveFail)?;
        PublicKeyBe::from_little_endian(pk)
    };
    let info = match tx_public_key {
        Ok(transaction_field) => {
            if transaction_field == matching_private_key {
                return Ok(());
            }
            KeyMismatchInfo::KeyMismatch {
                transaction_field,
                matching_private_key,
            }
        }
        Err(_err) => KeyMismatchInfo::NoSecpAllowed {
            matching_private_key,
        },
    };
    let _confirm = ui_display(&info)?;

    Err(AppSW::PublicKeyMismatch)
}

enum KeyMismatchInfo {
    NoSecpAllowed {
        matching_private_key: PublicKeyBe,
    },
    KeyMismatch {
        transaction_field: PublicKeyBe,
        matching_private_key: PublicKeyBe,
    },
}

fn ui_display(info: &KeyMismatchInfo) -> Result<bool, AppSW> {
    let mut key_buf1 = Buffer::<60>::new();
    let mut key_buf2 = Buffer::<60>::new();
    match info {
        KeyMismatchInfo::NoSecpAllowed {
            matching_private_key,
        } => {
            key_buf1.write_str("SECP256K1 curve was used");
            matching_private_key.display_str_base58(&mut key_buf2)?;
        }
        KeyMismatchInfo::KeyMismatch {
            transaction_field,
            matching_private_key,
        } => {
            transaction_field.display_str_base58(&mut key_buf1)?;
            matching_private_key.display_str_base58(&mut key_buf2)?;
        }
    }

    let my_fields = [
        Field {
            name: "Transaction Field",
            value: key_buf1.as_str(),
        },
        Field {
            name: "Requested BIP32",
            value: key_buf2.as_str(),
        },
    ];

    let msg_before = "Pub Key Mismatch";
    let msg_after = "Error!";

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let binding = [msg_before];

        let my_review = MultiFieldReview::new(
            &my_fields,
            &binding,
            Some(&EYE),
            msg_after,
            Some(&CROSSMARK),
            msg_after,
            Some(&CROSSMARK),
        );

        Ok(my_review.show())
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

        let info_button =
            InfoButton::new(msg_after, Some(&NEAR_LOGO), "Confirm", TuneIndex::Success);

        let tag_values_list = TagValueList::new(&my_fields, 2, false, false);

        let review: NbglGenericReview = NbglGenericReview::new()
            .add_content(NbglPageContent::CenteredInfo(centered_info))
            .add_content(NbglPageContent::TagValueList(tag_values_list))
            .add_content(NbglPageContent::InfoButton(info_button));

        let res = review.show("Reject");
        let status: NbglStatus = NbglStatus::new();
        match res {
            true => {
                status.text("Confirmed").show(true);
            }
            false => {
                status.text("Transaction rejected").show(false);
            }
        }
        Ok(res)
    }
}
