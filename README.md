no_std 下非常简易的json解析库 ，使用示例：

~~~ 
    fn test_long_complex(){
        let json = r#"{
    location: {
        id: "WT3Q0FW9ZJ3Q",
        name: "武汉",
        country: "CN",
        path: "武汉,武汉,湖北,中国",
        timezone: "Asia/Shanghai",
        timezone_offset: "+08:00"
    },
    daily: [
        {
            date: "2024-05-31",
            text_day: "晴",
            code_day: "0",
            text_night: "晴",
            code_night: "1",
            high: "26",
            low: "18",
            rainfall: "0.00",
            precip: "0.00",
            wind_direction: "无持续风向",
            wind_direction_degree: "",
            wind_speed: "8.4",
            wind_scale: "2",
            humidity: "81"
        },
        {
            date: "2024-05-31",
            text_day: "晴",
            code_day: "0",
            text_night: "晴",
            code_night: "1",
            high: "26",
            low: "18",
            rainfall: "0.00",
            precip: "0.00",
            wind_direction: "无持续风向",
            wind_direction_degree: "",
            wind_speed: "8.4",
            wind_scale: "2",
            humidity: "81"
        },
        {
            date: "2024-05-31",
            text_day: "晴",
            code_day: "0",
            text_night: "晴",
            code_night: "1",
            high: "26",
            low: "18",
            rainfall: "0.00",
            precip: "0.00",
            wind_direction: "无持续风向",
            wind_direction_degree: "",
            wind_speed: "8.4",
            wind_scale: "2",
            humidity: "81"
        }
    ]
}
"#;

        let mut is_object = false;
        match parse_json(json){
            Ok(result) => {


                //use get function
                assert_eq!(3 ,result.get("daily").unwrap().get_array().unwrap().len());

                assert_eq!( 6,result.get("location").unwrap().get_object().unwrap().len());


                if let JsonValue::Object(result) = result {
                    is_object = true;
                    for member in result {
                        if member.0 == "location".to_string() {
                            let expected = vec![
                                ("id".to_string(), JsonValue::String("WT3Q0FW9ZJ3Q".to_string())),
                                ("name".to_string(), JsonValue::String("武汉".to_string())),
                                ("country".to_string(), JsonValue::String("CN".to_string())),
                                ("path".to_string(), JsonValue::String("武汉,武汉,湖北,中国".to_string())),
                                ("timezone".to_string(), JsonValue::String("Asia/Shanghai".to_string())),
                                ("timezone_offset".to_string(), JsonValue::String("+08:00".to_string()))
                            ];
                            assert_eq!(*member.1.get_object().unwrap() ,expected );
                        }

                        if member.0 == "daily".to_string() {
                            assert_eq!(3 ,member.1.get_array().unwrap().len() );
                        }
                    }

                }
            }

            Err(e) => {

            }
        }
        assert_eq!(is_object, true);
    }