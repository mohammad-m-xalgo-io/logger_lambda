use rusoto_core::{Region, HttpClient, RusotoError};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, QueryInput, AttributeValue, QueryOutput, QueryError};
use rusoto_credential::ProfileProvider;
use std::collections::HashMap;
use std::env;

pub struct DynamoDbService {
    client: DynamoDbClient,
}

// #[automock]
trait DynamoDbServiceTrait {
    async fn query(&self, query_input: QueryInput) -> Result<QueryOutput, RusotoError<QueryError>>;

}
impl DynamoDbService {
    fn new(profile_name: String, region: Region) -> Self {
        let mut profile_provider = ProfileProvider::new().unwrap();
        profile_provider.set_profile(profile_name);
        let client = DynamoDbClient::new_with(HttpClient::new().unwrap(), profile_provider, region);

        Self {
            client
        }
    }
}

impl DynamoDbServiceTrait for DynamoDbService {
    async fn query(&self, query_input: QueryInput) -> Result<QueryOutput, RusotoError<QueryError>> {
        self.client.query(query_input).await
    }
}



pub async fn query_dynamodb(service: &DynamoDbService, table_name: String, partition_key: String, partition_value: String, sort_key: Option<(String, String)>) {

    let mut query_input: QueryInput = Default::default();
    query_input.table_name = table_name;

    let mut attr: HashMap<String, AttributeValue> = HashMap::new();
    attr.insert(":val1".to_string(), AttributeValue {
        s: Some(partition_value),
        ..Default::default()
    });

    match sort_key {
        Some((sort_key_name, sort_key_value)) => {
            query_input.key_condition_expression = Some(format!("{} = :val1 AND {} = :val2", partition_key, sort_key_name));
            attr.insert(":val2".to_string(), AttributeValue {
                s: Some(sort_key_value),
                ..Default::default()
            });
        },
        None => {
            query_input.key_condition_expression = Some(format!("{} = :val1", partition_key));
        },
    }

    query_input.expression_attribute_values = Some(attr);

    match service.query(query_input).await {
        Ok(output) => {
            match output.items {
                Some(item_list) => {
                    for item in item_list {
                        for (key, value) in item {
                            if let Some(s) = &value.s {
                                println!("{}: {}", key, s);
                            }
                        }
                    }
                },
                None => println!("No items found"),
            }
        },
        Err(error) => {
            if format!("{:?}", error).contains("ExpiredTokenException") {
                println!("The security token included in the request is expired. Please refresh your credentials.");
            }
            else if format!("{:?}", error).contains("AccessDeniedException")  {
                println!("Access Denied: Please check your credentials and permissions.");
            }
            else if format!("{:?}", error).contains("ResourceNotFoundException") {
                println!("Resource not found: Please check the table name and partition key.");
            }
            else {
                println!("Error: {:?}", error);
            }
        },
    }
}