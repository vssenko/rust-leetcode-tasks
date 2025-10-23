#[cfg(test)]
mod test {
    use json_get_macros::get;

    #[test]
    fn json_get_macros_1() {
        let json_value = serde_json::json!({
            "a": 5,
            "b": {
                "c": 10
            }
        });

        let result: Option<&serde_json::Value> = get!(json_value.b.c);

        assert_eq!(
            result.unwrap().as_number(),
            Some(&serde_json::Number::from_i128(10).unwrap())
        );
    }

    #[test]
    fn json_get_macros_2() {
        let json_value = serde_json::json!({
            "a": 5,
            "arr": [{"c": 15}]
        });

        let result: Option<&serde_json::Value> = get!(json_value.arr[0].c);

        assert_eq!(
            result.unwrap().as_number(),
            Some(&serde_json::Number::from_i128(15).unwrap())
        );
    }
}
