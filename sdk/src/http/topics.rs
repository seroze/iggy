use crate::client::TopicClient;
use crate::compression::compression_algorithm::CompressionAlgorithm;
use crate::error::IggyError;
use crate::http::client::HttpClient;
use crate::http::HttpTransport;
use crate::identifier::Identifier;
use crate::models::topic::{Topic, TopicDetails};
use crate::topics::create_topic::CreateTopic;
use crate::topics::delete_topic::DeleteTopic;
use crate::topics::get_topic::GetTopic;
use crate::topics::get_topics::GetTopics;
use crate::topics::purge_topic::PurgeTopic;
use crate::topics::update_topic::UpdateTopic;
use crate::utils::byte_size::IggyByteSize;
use crate::utils::expiry::IggyExpiry;
use async_trait::async_trait;

#[async_trait]
impl TopicClient for HttpClient {
    async fn get_topic(
        &self,
        stream_id: &Identifier,
        topic_id: &Identifier,
    ) -> Result<TopicDetails, IggyError> {
        get_topic(
            self,
            &GetTopic {
                stream_id: stream_id.clone(),
                topic_id: topic_id.clone(),
            },
        )
        .await
    }

    async fn get_topics(&self, stream_id: &Identifier) -> Result<Vec<Topic>, IggyError> {
        get_topics(
            self,
            &GetTopics {
                stream_id: stream_id.clone(),
            },
        )
        .await
    }

    async fn create_topic(
        &self,
        stream_id: &Identifier,
        name: &str,
        partitions_count: u32,
        compression_algorithm: CompressionAlgorithm,
        replication_factor: Option<u8>,
        topic_id: Option<u32>,
        message_expiry: IggyExpiry,
        max_topic_size: Option<IggyByteSize>,
    ) -> Result<(), IggyError> {
        create_topic(
            self,
            &CreateTopic {
                stream_id: stream_id.clone(),
                name: name.to_string(),
                partitions_count,
                compression_algorithm,
                replication_factor,
                topic_id,
                message_expiry: message_expiry.into(),
                max_topic_size,
            },
        )
        .await
    }

    async fn update_topic(
        &self,
        stream_id: &Identifier,
        topic_id: &Identifier,
        name: &str,
        compression_algorithm: CompressionAlgorithm,
        replication_factor: Option<u8>,
        message_expiry: IggyExpiry,
        max_topic_size: Option<IggyByteSize>,
    ) -> Result<(), IggyError> {
        update_topic(
            self,
            &UpdateTopic {
                stream_id: stream_id.clone(),
                topic_id: topic_id.clone(),
                name: name.to_string(),
                compression_algorithm,
                replication_factor,
                message_expiry: message_expiry.into(),
                max_topic_size,
            },
        )
        .await
    }

    async fn delete_topic(
        &self,
        stream_id: &Identifier,
        topic_id: &Identifier,
    ) -> Result<(), IggyError> {
        delete_topic(
            self,
            &DeleteTopic {
                stream_id: stream_id.clone(),
                topic_id: topic_id.clone(),
            },
        )
        .await
    }

    async fn purge_topic(
        &self,
        stream_id: &Identifier,
        topic_id: &Identifier,
    ) -> Result<(), IggyError> {
        purge_topic(
            self,
            &PurgeTopic {
                stream_id: stream_id.clone(),
                topic_id: topic_id.clone(),
            },
        )
        .await
    }
}

async fn get_topic<T: HttpTransport>(
    transport: &T,
    command: &GetTopic,
) -> Result<TopicDetails, IggyError> {
    let response = transport
        .get(&format!(
            "{}/{}",
            get_path(&command.stream_id.as_cow_str()),
            command.topic_id
        ))
        .await?;
    let topic = response.json().await?;
    Ok(topic)
}

async fn get_topics<T: HttpTransport>(
    transport: &T,
    command: &GetTopics,
) -> Result<Vec<Topic>, IggyError> {
    let response = transport
        .get(&get_path(&command.stream_id.as_cow_str()))
        .await?;
    let topics = response.json().await?;
    Ok(topics)
}

async fn create_topic<T: HttpTransport>(
    transport: &T,
    command: &CreateTopic,
) -> Result<(), IggyError> {
    transport
        .post(&get_path(&command.stream_id.as_cow_str()), &command)
        .await?;
    Ok(())
}

async fn update_topic<T: HttpTransport>(
    transport: &T,
    command: &UpdateTopic,
) -> Result<(), IggyError> {
    transport
        .put(
            &get_details_path(
                &command.stream_id.as_cow_str(),
                &command.topic_id.as_cow_str(),
            ),
            command,
        )
        .await?;
    Ok(())
}

async fn delete_topic<T: HttpTransport>(
    transport: &T,
    command: &DeleteTopic,
) -> Result<(), IggyError> {
    transport
        .delete(&get_details_path(
            &command.stream_id.as_cow_str(),
            &command.topic_id.as_cow_str(),
        ))
        .await?;
    Ok(())
}

async fn purge_topic<T: HttpTransport>(
    transport: &T,
    command: &PurgeTopic,
) -> Result<(), IggyError> {
    transport
        .delete(&format!(
            "{}/purge",
            &get_details_path(
                &command.stream_id.as_cow_str(),
                &command.topic_id.as_cow_str(),
            )
        ))
        .await?;
    Ok(())
}

fn get_path(stream_id: &str) -> String {
    format!("streams/{stream_id}/topics")
}

fn get_details_path(stream_id: &str, topic_id: &str) -> String {
    format!("{}/{topic_id}", get_path(stream_id))
}
