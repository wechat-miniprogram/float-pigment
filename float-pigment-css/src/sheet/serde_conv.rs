// use serde::{
//     de::{self, SeqAccess, Unexpected, Visitor},
//     Deserializer, Serializer,
// };
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;

// use super::Media;

// pub(crate) fn serialize_env<R>(f: impl FnOnce() -> R) -> R {
//     let ret = f();
//     GLOBAL.width(|x| {
//         x.borrow_mut().rc_media_ser_map.borrow_mut().clear();
//     });
//     ret
// }

// pub(crate) fn deserialize_env<R>(f: impl FnOnce() -> R) -> R {
//     let ret = f();
//     GLOBAL.with(|x| {
//         x.borrow_mut().rc_media_ser_arr.borrow_mut().clear();
//     });
//     ret
// }

// pub(crate) mod option_rc_media {
//     use super::*;
//     use std::fmt;

//     struct OptionRcMediaVisitor;

//     impl<'de> Visitor<'de> for OptionRcMediaVisitor {
//         type Value = Option<Rc<Media>>;

//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             write!(formatter, "media query id")
//         }

//         fn visit_u64<E>(self, x: u64) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             RC_MEDIA_SERDE_ARR.with(|arr| {
//                 if x == 0 {
//                     Ok(None)
//                 } else {
//                     match arr.borrow().get(x as usize - 1) {
//                         Some(x) => Ok(Some(x.clone())),
//                         None => Err(de::Error::invalid_value(
//                             Unexpected::Unsigned(x as u64),
//                             &self,
//                         )),
//                     }
//                 }
//             })
//         }
//     }

//     pub(crate) fn deserialize<'de, D>(de: D) -> Result<Option<Rc<Media>>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         de.deserialize_u64(OptionRcMediaVisitor)
//     }

//     pub(crate) fn serialize<S>(x: &Option<Rc<Media>>, ser: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         if let Some(x) = x {
//             let media: &Media = x;
//             let addr = media as *const Media as usize;
//             let id = RC_MEDIA_SER_MAP.with(|map| {
//                 *map.borrow_mut().get(&addr).unwrap_or_else(|| {
//                     error!("No proper media index. Ignored a media query limit.");
//                     &0
//                 })
//             });
//             ser.serialize_u32(id)
//         } else {
//             ser.serialize_u32(0)
//         }
//     }
// }

// pub(crate) mod vec_rc_media {
//     use super::*;
//     use std::fmt;

//     struct VecRcMediaVisitor;

//     impl<'de> Visitor<'de> for VecRcMediaVisitor {
//         type Value = Vec<Rc<Media>>;

//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             write!(formatter, "media query")
//         }

//         fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//         where
//             A: SeqAccess<'de>,
//         {
//             let mut ret = vec![];
//             if let Some(len) = seq.size_hint() {
//                 RC_MEDIA_SERDE_ARR.with(|arr| {
//                     let mut arr = arr.borrow_mut();
//                     arr.reserve(len);
//                     ret.reserve(len);
//                 })
//             }
//             while let Some(item) = seq.next_element::<Box<Media>>()? {
//                 let item: Rc<Media> = item.into();
//                 RC_MEDIA_SERDE_ARR.with(|arr| {
//                     let mut arr = arr.borrow_mut();
//                     arr.push(item.clone());
//                 });
//                 ret.push(item);
//             }
//             Ok(ret)
//         }
//     }

//     pub(crate) fn deserialize<'de, D>(de: D) -> Result<Vec<Rc<Media>>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         de.deserialize_seq(VecRcMediaVisitor)
//     }

//     pub(crate) fn serialize<S>(x: &Vec<Rc<Media>>, ser: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         ser.collect_seq(x.iter().enumerate().map(|(i, x)| {
//             let id = (i + 1) as u32;
//             let media: &Media = x;
//             let addr = media as *const Media as usize;
//             RC_MEDIA_SER_MAP.with(|map| {
//                 map.borrow_mut().insert(addr, id);
//             });
//             let ret: Box<Media> = Box::new(media.clone());
//             ret
//         }))
//     }
// }
