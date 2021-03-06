use std::fmt;

use neon::prelude::*;
use neon_serde;
use serde::de::{Deserializer, Visitor};

use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Data {
  name: Name,
  index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
  page: Page,
  group: Group,
  stuff: Data,
}

#[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Name(usize);

#[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Group(usize);

#[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Page(usize);

struct GroupVisitor;

impl<'de> Visitor<'de> for GroupVisitor {
  type Value = Group;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("an group name")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
    Ok(Group(value[5..].parse().unwrap()))
  }
}

impl<'de> Deserialize<'de> for Group {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(GroupVisitor)
  }
}

struct PageVisitor;

impl<'de> Visitor<'de> for PageVisitor {
  type Value = Page;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("an page name")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
    Ok(Page(value[4..].parse().unwrap()))
  }
}

impl<'de> Deserialize<'de> for Page {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(PageVisitor)
  }
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
  type Value = Name;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("an name")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
    Ok(Name(value[4..].parse().unwrap()))
  }
}

impl<'de> Deserialize<'de> for Name {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(NameVisitor)
  }
}

#[derive(Deserialize)]
struct ObjectTuple<K, V>(#[serde(with = "tuple_vec_map")] Vec<(K, V)>)
where
  K: serde::de::DeserializeOwned,
  V: serde::de::DeserializeOwned;

fn objecttuple(input: String) -> Vec<Link>{
  let object: ObjectTuple<Page, ObjectTuple<Group, Vec<Data>>> =
    serde_json::from_str(&input).unwrap();
  object
  .0
  .iter()
  .map(|(page_key, page)| {
    page.0.iter().map(move |(group_key, group)| {
      group.iter().map(move |data| Link {
        stuff: *data,
        page: *page_key,
        group: *group_key,
      })
    })
  })
  .flatten()
  .flatten()
  .collect()
}

pub fn string_neon_value(mut cx: FunctionContext) -> JsResult<JsValue> {
  let arg0: String = cx.argument::<JsString>(0)?.value();
  let list = objecttuple(arg0);
  let result = neon_serde::to_value(&mut cx, &list)?;
  Ok(result)
}

pub fn string_serde_string(mut cx: FunctionContext) -> JsResult<JsString> {
  let arg0: String = cx.argument::<JsString>(0)?.value();
  let list = objecttuple(arg0);
  let str_data = serde_json::to_string(&list).unwrap();
  Ok(cx.string(str_data))
}
