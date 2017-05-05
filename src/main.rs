#[macro_use]
extern crate json;

fn main() {
    let mut parsed = json::parse(r#"[
        {
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ],
                "users": [
                            {"name": "ganily1", "uid": 11},
                            {"name": "ganily2", "uid": 12},
                            {"name": "ganily3", "uid": 13}
                          ]
            }
        }]
        "#).unwrap();

    let instantiated = object!{
        "code" => 201,
        "success" => true,
        "payload" => object!{
            "features" => array![
                "awesome2",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };
    let _a = parsed.push(instantiated);
//    parsed[0]["payload"]["features"][0] = "Hello".into();
//    println!("{}", parsed[0]["payload"]["users"]);
//    println!("{}", parsed.dump());
    println!("{}", find(parsed, r#"{"$op":"and","success": true,"payload.features": "awesome"}"#, "payload.users"));
//    println!("{}", find(parsed, r#"{"$op":"and","success": true,"payload.users.name": "ganily3","payload.users.uid": 13}"#, "payload.users"));
//    println!("{}", find(instantiated, r#"{"$op":"and", "success": true,"payload.features": "awesome2"}"#, "payload.features"));

//    let mut query = json::parse(r#"{"code": 200,"success": true}"#).unwrap();

//    let mut entries = query.entries();
//    let (key, value) = entries.next().unwrap();
//    println!("rrr {}- {}", key, value);

//    for (key, value) in query.entries() {
//        println!("rrr {}- {}", key, value);
//    }
}

fn find(table: json::JsonValue, where_case: &str, return_type: &str) -> String {
    let query = json::parse(where_case).unwrap();
    let op = &query["$op"];
    let where_result: json::JsonValue;
    match table {
        json::JsonValue::Object(ref _value)   => {
            if eq_object(&table, &query, op) {
                where_result = table.clone();
            } else {
                where_result = json::JsonValue::Null;
            }
        },
        json::JsonValue::Array(ref value)  => {
            let mut data = json::JsonValue::new_array();
            for idx in 0..value.len() {
                if eq_object(&value[idx], &query, op) {
                    let _b = data.push(value[idx].clone());
                }
            }
            where_result = data;
        },
        _ => {where_result = json::JsonValue::Null;}
    }

    find_return(where_result, return_type).dump()
}

fn find_return(table: json::JsonValue, return_type: &str) -> json::JsonValue {
    let rs_types: Vec<&str> = return_type.split(".").collect();
//    let rs: &mut Vec<&str> = Vec::new();
    match table {
        json::JsonValue::Object(ref _value)   => {
            let tablex = findloop(&table, &rs_types);

            tablex
        },
        json::JsonValue::Array(ref value)  => {
            let mut data = json::JsonValue::new_array();
            for idx in 0..value.len() {
                let tablex2 = findloop(&value[idx], &rs_types);
                println!("{:?}", tablex2);
                let _a = data.push(tablex2);
            }
//                        println!("{:?}", value.len() );
            data
        },
        _                             => table
    }
}

fn findloop(value: &json::JsonValue, rs_types: &Vec<&str>) -> json::JsonValue {
    let mut tablex2 = value;
    for idx in 0..rs_types.len() {
        let rs_type = rs_types[idx];
        if !rs_type.is_empty() {
//            tablex2 = &tablex2[*rs_type];
            tablex2 = match *tablex2 {
                json::JsonValue::Object(ref _value)   => &tablex2[rs_type],
                json::JsonValue::Array(ref value1)  => {
                    //分两种情况 1=> [1,2,3] 直接return
                    //2=> [{name:'aaa'},{name:'bbb'}]
                    if value1.len() >0 {
                        tablex2 = match value1[0] {
                            json::JsonValue::Object(ref _value2) => {
                                let mut data = json::JsonValue::new_array();
                                for i in 0..value1.len() {
                                    let mut rs_types2: Vec<&str> = vec![];
                                    for j in idx..rs_types.len() {

                                        let _ = rs_types2.push(rs_types[j]);
                                    }
                                    let t = findloop(&value1[i], &rs_types2);
                                    if !t.is_empty() {
                                        let _ = data.push(t);
                                    }
                                }
                                return data;
                            },
                            _  =>  tablex2
                        }
                    }

                    &tablex2[rs_type]
                },
                _                             => &tablex2
            }

        }
    }
    tablex2.clone()
}

fn equals(table: &json::JsonValue, key: &str, value: &json::JsonValue) -> bool {
    let t2 = find_return(table.clone(), key);
//            println!("1 t2in {:?}", table);
    match t2 {
        json::JsonValue::Array(ref value1)  => {
            for i in 0..value1.len() {
                if &value1[i] == value {
                    return true;
                }
            }
        },
        _                             => {

            if &t2 == value {
                return true;
            }
        }
    }
    false
}

fn eq_object(table: &json::JsonValue, query: &json::JsonValue, op: &json::JsonValue) -> bool {
    let mut ok: bool = false;
    for (key, value) in query.entries() {
//                println!("in {}- {}", key, value);
        if key == "$op" {
            continue;
        }
        //判断是否需要进行下一次循环
        if op == "or" {
            if equals(&table, key, &value) {
                ok = true;
                break;
            }

        } else if op == &json::JsonValue::Null || op == "and" {
            //如果此处没有找到,则必定结果为空
            //如果此处循环结果不为空,则下次循环在现有结果的基础上进行
            if !equals(&table, key, &value) {
                ok = false;
                break;
            } else { ok = true; }
        }
    }
    ok
}