use std::collections::HashMap;
#[derive(Debug)]
#[allow(dead_code)]
pub struct Query<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}
// impl<'buf> Query<'buf> {
//     pub fn get(&self, key: &str) -> Option<&'buf Value> {
//         self.data.get(key)
//     }
// }
impl<'buf> From<&'buf str> for Query<'buf> {
    fn from(query: &'buf str) -> Self {
        let mut data: HashMap<&'buf str, Value<'buf>> = HashMap::new();
        query.split('&').for_each(|elem: &str| {
            if let Some((key, value)) = elem.split_once('=') {
                data.entry(key)
                    .and_modify(|old_value| match old_value {
                        Value::Single(single_val) => {
                            *old_value = Value::Multiple(vec![single_val, value]);
                        }
                        Value::Multiple(multiple_val) => multiple_val.push(value),
                    })
                    .or_insert(Value::Single(value));
            } else {
                data.entry(elem)
                    .and_modify(|old_value| match old_value {
                        Value::Single(single_val) => {
                            *old_value = Value::Multiple(vec![single_val, ""]);
                        }
                        Value::Multiple(multiple_val) => multiple_val.push(""),
                    })
                    .or_insert(Value::Single(""));
            }
        });
        Self { data }
    }
}
// Some query specify a pair of key value while some other have a pair of key: multiple values
#[derive(Debug, PartialEq, Eq)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
