pub fn convert(input: &str) -> String {
    let value: serde_json::Value = quick_xml::de::from_str(&input).unwrap();

    serde_json::to_string_pretty(&value).unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::convert;

    #[test]
    fn test() {
        let actual = convert(
            r#"<?xml version="1.0" encoding="UTF-8"?>
        <note>
          <to>Dove</to>
          <from><name>Jane</name></from>
          <heading>Reminder</heading>
          <body>Learn Rust this weekend!</body>
        </note>"#,
        );

        let expected = r#"{
  "body": {
    "$text": "Learn Rust this weekend!"
  },
  "from": {
    "name": {
      "$text": "Jane"
    }
  },
  "heading": {
    "$text": "Reminder"
  },
  "to": {
    "$text": "Dove"
  }
}"#;

        assert_eq!(actual, expected);
    }
}
