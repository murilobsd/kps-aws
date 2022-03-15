#![allow(dead_code)]
use rusoto_core::Region;
use rusoto_iam::{CreateAccessKeyRequest, CreateUserRequest, Iam, IamClient};
use rusoto_ecr::{Ecr, EcrClient, CreateRepositoryRequest};

/// Create IAM User
async fn create_user(
    client: &IamClient,
    user_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let input: CreateUserRequest = CreateUserRequest {
        user_name: user_name.into(),
        ..Default::default()
    };

    match client.create_user(input.clone()).await {
        Ok(output) => {
            match output.user {
                Some(user) => {
                    println!(
                        "created user name {} id: {}",
                        user.user_name, user.user_id
                    );
                }
                None => println!("failed create {}", input.user_name),
            };
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };

    Ok(())
}

/// Create Access key to the user
async fn create_access_key(
    client: &IamClient,
    user_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let input: CreateAccessKeyRequest = CreateAccessKeyRequest {
        user_name: Some(user_name.to_string())
    };

    match client.create_access_key(input).await {
        Ok(output) => {
            println!(
                "Access Key Id: {}\nAccess Key Secret: {}",
                output.access_key.access_key_id,
                output.access_key.secret_access_key
            );
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };

    Ok(())
}

/// Create ECR Repo (private)
async fn create_ecr_repo(
    client: &EcrClient,
    repo_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let input: CreateRepositoryRequest = CreateRepositoryRequest {
        repository_name: repo_name.into(),
        ..Default::default()
    };

    match client.create_repository(input.clone()).await {
        Ok(output) => {
            match output.repository {
                Some(repository) => {
                    println!("name {}, arn: {}", 
                        repository.repository_name.unwrap(), 
                        repository.repository_arn.unwrap());
                }
                None => println!("failed create {}", input.repository_name),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = IamClient::new(Region::UsEast1);
    let client = EcrClient::new(Region::UsEast1);
    let user_name = "kps-user-test";

    // create_user(&client, user_name).await?;
    // create_access_key(&client, user_name).await?;
    create_ecr_repo(&client, user_name).await?;

    Ok(())
}
