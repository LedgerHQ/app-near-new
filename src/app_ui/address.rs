/*****************************************************************************
 *   Ledger App Near Rust.
 *   (c) 2023 Ledger SAS.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/

use crate::utils::crypto;
use crate::AppSW;
use fmt_buffer::Buffer;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{NbglAddressReview, NbglGlyph, NbglReviewStatus, StatusType};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::bitmaps::{CROSSMARK, EYE, VALIDATE_14};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::{Field, MultiFieldReview};

pub fn ui_display_pk_base58(public_key: &crypto::PublicKeyBe) -> Result<bool, AppSW> {
    let mut out_buf = Buffer::<60>::new();
    public_key.display_str_base58(&mut out_buf)?;

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_field = [Field {
            name: "Public Key",
            value: out_buf.as_str(),
        }];

        let my_review = MultiFieldReview::new(
            &my_field,
            &["Confirm Address"],
            Some(&EYE),
            "Approve",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );
        Ok(my_review.show())
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        const NEAR_LOGO: NbglGlyph =
            NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));

        let review: NbglAddressReview = NbglAddressReview::new()
            .glyph(&NEAR_LOGO)
            .verify_str("Confirm Public Key");

        let res = review.show(out_buf.as_str());
        let status = NbglReviewStatus::new();
        match res {
            true => {
                status.status_type(StatusType::Address).show(true);
            }
            false => {
                status.status_type(StatusType::Address).show(false);
            }
        }

        Ok(res)
    }
}

pub fn ui_display_hex(public_key: &crypto::PublicKeyBe) -> Result<bool, AppSW> {
    let mut out_buf = [0u8; 64];
    let pbkey_str = public_key.display_str_hex(&mut out_buf);

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_field = [Field {
            name: "Wallet ID",
            value: pbkey_str,
        }];
        let my_review = MultiFieldReview::new(
            &my_field,
            &["Confirm Address"],
            Some(&EYE),
            "Approve",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );
        Ok(my_review.show())
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        const NEAR_LOGO: NbglGlyph =
            NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));

        let review: NbglAddressReview = NbglAddressReview::new()
            .glyph(&NEAR_LOGO)
            .verify_str("Confirm Wallet ID");

        let res = review.show(pbkey_str);
        let status = NbglReviewStatus::new();
        match res {
            true => {
                status.status_type(StatusType::Address).show(true);
            }
            false => {
                status.status_type(StatusType::Address).show(false);
            }
        }
        Ok(res)
    }
}
