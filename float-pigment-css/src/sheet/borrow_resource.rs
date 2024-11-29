#![doc(hidden)]
#[allow(unused_imports)]
use alloc::vec::Vec;

#[cfg(feature = "deserialize")]
use hashbrown::HashMap;
use serde::{de, ser::SerializeTuple, Deserialize, Serialize};

use super::{borrow::Array, str_store::*};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum StyleSheetImportIndex {
    None,
    V1(StyleSheetImportIndexV1),
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct StyleSheetImportIndexV1 {
    buf: StrBuffer,
    deps: Array<(StrRef, Array<StrRef>)>,
}

impl StyleSheetImportIndex {
    #[cfg(feature = "serialize")]
    pub(crate) fn from_sheet(res: &crate::group::StyleSheetImportIndex) -> Self {
        let deps = res
            .deps
            .iter()
            .map(|(k, v)| {
                (
                    StrRef::from(k.clone()),
                    v.0.iter()
                        .map(|s| StrRef::from(s.clone()))
                        .collect::<Vec<_>>()
                        .into(),
                )
            })
            .collect::<Vec<_>>()
            .into();
        let mut str_store = StrBuffer::new();
        str_store.freeze();
        Self::V1(StyleSheetImportIndexV1 {
            buf: str_store,
            deps,
        })
    }

    #[cfg(feature = "deserialize")]
    pub(crate) fn into_sheet(self) -> crate::group::StyleSheetImportIndex {
        let deps = match self {
            StyleSheetImportIndex::None => HashMap::default(),
            StyleSheetImportIndex::V1(this) => this
                .deps
                .into_vec()
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.to_string(),
                        (
                            v.into_iter().map(|x| x.to_string()).collect(),
                            false,
                            core::cell::Cell::new(false),
                        ),
                    )
                })
                .collect(),
        };
        crate::group::StyleSheetImportIndex { deps }
    }

    #[cfg(feature = "deserialize")]
    pub(crate) fn merge_to_sheet(self, res: &mut crate::group::StyleSheetImportIndex) {
        match self {
            StyleSheetImportIndex::None => {}
            StyleSheetImportIndex::V1(this) => {
                for (k, v) in this.deps.into_iter() {
                    res.deps.insert(
                        k.to_string(),
                        (
                            v.into_iter().map(|x| x.to_string()).collect(),
                            false,
                            core::cell::Cell::new(false),
                        ),
                    );
                }
            }
        }
    }
}

impl Serialize for StyleSheetImportIndexV1 {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self { buf: _, deps } = self;
        str_buffer_ser_env(
            || float_pigment_consistent_bincode::serialized_size(&deps),
            |r, buf| match r {
                Ok(_) => {
                    let mut seq = ser.serialize_tuple(2)?;
                    seq.serialize_element(buf.whole_buffer())?;
                    seq.serialize_element(&deps)?;
                    seq.end()
                }
                Err(_) => {
                    use serde::ser::Error;
                    Err(S::Error::custom("Failed preprocessing StrRef"))
                }
            },
        )
    }
}

impl<'de> Deserialize<'de> for StyleSheetImportIndexV1 {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StyleSheetImportIndexVisitor;

        impl<'de> serde::de::Visitor<'de> for StyleSheetImportIndexVisitor {
            type Value = StyleSheetImportIndexV1;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "StyleSheetImportIndex")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let buf = SerdeThreadGlobalState::get_de_optional(|mut de| {
                    if let Some(drop_callback) = de.as_mut().and_then(|de| de.zero_copy.take()) {
                        seq.next_element::<&[u8]>()?
                            .map(|x| {
                                let ptr = x as *const [u8] as *mut [u8];
                                unsafe { StrBuffer::new_static_borrowed(ptr, drop_callback) }
                            })
                            .ok_or_else(|| de::Error::invalid_length(0, &"StyleSheetImportIndex"))
                    } else {
                        seq.next_element::<Vec<u8>>()?
                            .map(StrBuffer::new_with_buf)
                            .ok_or_else(|| de::Error::invalid_length(0, &"StyleSheetImportIndex"))
                    }
                })?;
                let deps = str_buffer_de_env(&buf, || {
                    let deps = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &"StyleSheetImportIndex"))?;
                    Ok(deps)
                })?;
                Ok(StyleSheetImportIndexV1 { buf, deps })
            }
        }

        de.deserialize_tuple(2, StyleSheetImportIndexVisitor)
    }
}
