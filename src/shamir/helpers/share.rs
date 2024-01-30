use super::string::str_to_u8;

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct ShareComponent {
    pub(crate) id: u8,
    pub(crate) data: String,
}

pub(crate) fn extract_share_component(share: &str) -> Result<ShareComponent, String> {
    Ok(ShareComponent {
        id: str_to_u8(&share[..2], 16)?,
        data: share[2..].to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use crate::shamir::helpers::share::{extract_share_component, ShareComponent};

    #[test]
    fn test_extract_share_component() {
        assert_eq!(
            ShareComponent {
                id: 1,
                data: "8c120c6f29ec1dbd3a383e9afc8d954f5bbd8d2dde9d225ee09878997640027c"
                    .to_string()
            },
            extract_share_component(
                "018c120c6f29ec1dbd3a383e9afc8d954f5bbd8d2dde9d225ee09878997640027c"
            )
            .unwrap()
        );
        assert_eq!(
            ShareComponent {
                id: 2,
                data: "4e04ca20e3d83c798598ada4b2cb891a2f5df16cbb0bc2fbe7eeb30b8789ecb5"
                    .to_string()
            },
            extract_share_component(
                "024e04ca20e3d83c798598ada4b2cb891a2f5df16cbb0bc2fbe7eeb30b8789ecb5"
            )
            .unwrap()
        );
        assert_eq!(
            ShareComponent {
                id: 3,
                data: "c216c64fca3421c4bfa0933e4e461c5407881d2c0ce4b3c06404aee6a5ac9dbd"
                    .to_string()
            },
            extract_share_component(
                "03c216c64fca3421c4bfa0933e4e461c5407881d2c0ce4b3c06404aee6a5ac9dbd"
            )
            .unwrap()
        );
        assert_eq!(
            ShareComponent {
                id: 4,
                data: "ad88e69f25d9608af4aa245b389ccd11fb3015516e52bdc3301e43508902b988"
                    .to_string()
            },
            extract_share_component(
                "04ad88e69f25d9608af4aa245b389ccd11fb3015516e52bdc3301e43508902b988"
            )
            .unwrap()
        );
        assert_eq!(
            ShareComponent {
                id: 5,
                data: "219aeaf00c357d37ce921ac1c411585fd3e5f911d9bdccf8b3f45ebdab27c880"
                    .to_string()
            },
            extract_share_component(
                "05219aeaf00c357d37ce921ac1c411585fd3e5f911d9bdccf8b3f45ebdab27c880"
            )
            .unwrap()
        );
    }
}
