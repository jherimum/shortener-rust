use std::fmt::Display;

const MAX_LENGTH: u8 = 7;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Id too long")]
    IdTooLong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkId {
    value: [char; 7],
}

impl LinkId {
    pub fn from_id<I: Into<u128>>(id: I) -> Result<Self, Error> {
        let encoded = base62::encode(id);
        if encoded.len() > MAX_LENGTH as usize {
            return Err(Error::IdTooLong);
        }

        let left = vec!['0'; 7 - encoded.len()];
        let rigth = encoded.chars().collect::<Vec<_>>();

        Ok(Self {
            value: [left, rigth].concat().try_into().unwrap(),
        })
    }
}

impl Display for LinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.iter().collect::<String>().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            LinkId {
                value: ['0', '0', '0', '0', '0', '0', '0']
            },
            LinkId::from_id(0_u64).unwrap()
        );
        assert_eq!(
            LinkId {
                value: ['0', '0', '0', '0', '0', '0', '1']
            },
            LinkId::from_id(1_u64).unwrap()
        );

        assert_eq!(
            LinkId {
                value: ['0', '0', '0', '0', '0', '1', '1']
            },
            LinkId::from_id(63_u64).unwrap()
        );

        let max = base62::decode("zzzzzzz").unwrap();

        assert_eq!(
            LinkId {
                value: ['z', 'z', 'z', 'z', 'z', 'z', 'z']
            },
            LinkId::from_id(max).unwrap()
        );

        let exceed = LinkId::from_id(max + 1);
        assert!(exceed.unwrap_err() == Error::IdTooLong);
    }
}
