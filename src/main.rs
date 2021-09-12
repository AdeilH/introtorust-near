use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn insert_transaction(transaction_map: &mut HashMap<String, u64>, id_value: (String, u64)) -> bool {
    if transaction_map.contains_key(&id_value.0)
    {
        return false;
    }
    else{
        transaction_map.insert(id_value.0, id_value.1);
        return true;
    }
}

pub fn transaction_exists(transaction_map: &mut HashMap<String, u64>, transactionid: String) -> bool {
    if transaction_map.contains_key(&transactionid)
    {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
  //  #[should_panic(expected = "Divide result is zero")]
    fn test_transaction() {
        let mut transaction_map: HashMap<String, u64> = HashMap::new();
        let transaction:(String, u64) = (String::from("ABC"), 30000);
        assert_eq!(insert_transaction(&mut transaction_map, transaction), true);
        let transaction2:(String, u64) = (String::from("ABC"), 30000);
        assert_eq!(insert_transaction(&mut transaction_map, transaction2), false);

        let transaction3:(String, u64) = (String::from("XYZ"), 30000);
        assert_eq!(insert_transaction(&mut transaction_map, transaction3), true);
    }
}
