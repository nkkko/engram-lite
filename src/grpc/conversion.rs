#[cfg(feature = "grpc")]
use crate::grpc::proto::{
    Embedding as ProtoEmbedding, Engram as ProtoEngram, Connection as ProtoConnection,
    Collection as ProtoCollection, EmbeddingModel as ProtoEmbeddingModel,
    CreateEngramRequest, CombinationMethod as ProtoCombinationMethod,
};
use crate::schema::{Engram, Connection, Collection};
use crate::embedding::{Embedding, EmbeddingModel};
use crate::vector_search::CombinationMethod;
use crate::error::EngramError;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tonic::Status;
use serde_json::Value as JsonValue;
use prost_types::{Timestamp, Struct, Value as ProstValue};
use uuid::Uuid;

#[cfg(feature = "grpc")]
pub fn engram_to_proto(engram: &Engram) -> Result<ProtoEngram, Status> {
    let created_at = chrono_to_timestamp(engram.created_at)?;
    let updated_at = match engram.updated_at {
        Some(updated) => Some(chrono_to_timestamp(updated)?),
        None => None,
    };
    
    let metadata = serde_value_to_struct(&engram.metadata)?;
    
    Ok(ProtoEngram {
        id: engram.id.clone(),
        content: engram.content.clone(),
        source: engram.source.clone(),
        confidence: engram.confidence,
        created_at: Some(created_at),
        updated_at,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn proto_to_engram(proto: &ProtoEngram) -> Result<Engram, Status> {
    let created_at = proto.created_at
        .as_ref()
        .map(timestamp_to_chrono)
        .transpose()?
        .unwrap_or_else(Utc::now);
        
    let updated_at = proto.updated_at
        .as_ref()
        .map(timestamp_to_chrono)
        .transpose()?;
        
    let metadata = proto.metadata
        .as_ref()
        .map(|s| struct_to_serde_value(s))
        .transpose()?
        .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new()));
    
    Ok(Engram {
        id: proto.id.clone(),
        content: proto.content.clone(),
        source: proto.source.clone(),
        confidence: proto.confidence,
        created_at,
        updated_at,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn create_engram_request_to_engram(req: CreateEngramRequest) -> Result<Engram, Status> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now();
    let metadata = req.metadata
        .map(struct_to_serde_value)
        .transpose()?
        .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new()));
    
    Ok(Engram {
        id,
        content: req.content,
        source: req.source,
        confidence: req.confidence,
        created_at,
        updated_at: None,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn connection_to_proto(connection: &Connection) -> Result<ProtoConnection, Status> {
    let created_at = chrono_to_timestamp(connection.created_at)?;
    let updated_at = match connection.updated_at {
        Some(updated) => Some(chrono_to_timestamp(updated)?),
        None => None,
    };
    
    let metadata = serde_value_to_struct(&connection.metadata)?;
    
    Ok(ProtoConnection {
        id: connection.id.clone(),
        source_id: connection.source_id.clone(),
        target_id: connection.target_id.clone(),
        relationship_type: connection.relationship_type.clone(),
        weight: connection.weight,
        created_at: Some(created_at),
        updated_at,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn proto_to_connection(proto: &ProtoConnection) -> Result<Connection, Status> {
    let created_at = proto.created_at
        .as_ref()
        .map(timestamp_to_chrono)
        .transpose()?
        .unwrap_or_else(Utc::now);
        
    let updated_at = proto.updated_at
        .as_ref()
        .map(timestamp_to_chrono)
        .transpose()?;
        
    let metadata = proto.metadata
        .as_ref()
        .map(|s| struct_to_serde_value(s))
        .transpose()?
        .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new()));
    
    Ok(Connection {
        id: proto.id.clone(),
        source_id: proto.source_id.clone(),
        target_id: proto.target_id.clone(),
        relationship_type: proto.relationship_type.clone(),
        weight: proto.weight,
        created_at,
        updated_at,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn collection_to_proto(collection: &Collection) -> Result<ProtoCollection, Status> {
    let created_at = chrono_to_timestamp(collection.created_at)?;
    let updated_at = match collection.updated_at {
        Some(updated) => Some(chrono_to_timestamp(updated)?),
        None => None,
    };
    
    let metadata = serde_value_to_struct(&collection.metadata)?;
    
    Ok(ProtoCollection {
        id: collection.id.clone(),
        name: collection.name.clone(),
        description: collection.description.clone(),
        engram_ids: collection.engram_ids.clone(),
        created_at: Some(created_at),
        updated_at,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn proto_to_collection(proto: &ProtoCollection) -> Result<Collection, Status> {
    let created_at = proto.created_at
        .as_ref()
        .map(timestamp_to_chrono)
        .transpose()?
        .unwrap_or_else(Utc::now);
        
    let updated_at = proto.updated_at
        .as_ref()
        .map(timestamp_to_chrono)
        .transpose()?;
        
    let metadata = proto.metadata
        .as_ref()
        .map(|s| struct_to_serde_value(s))
        .transpose()?
        .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new()));
    
    Ok(Collection {
        id: proto.id.clone(),
        name: proto.name.clone(),
        description: proto.description.clone(),
        engram_ids: proto.engram_ids.clone(),
        created_at,
        updated_at,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn embedding_to_proto(embedding: &Embedding) -> Result<ProtoEmbedding, Status> {
    let metadata = serde_value_to_struct(&JsonValue::Object(
        embedding.metadata.iter().map(|(k, v)| (k.clone(), JsonValue::String(v.clone()))).collect()
    ))?;
    
    Ok(ProtoEmbedding {
        vector: embedding.vector.clone(),
        model: embedding.model.clone(),
        dimensions: embedding.dimensions as u32,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn proto_to_embedding(proto: &ProtoEmbedding) -> Result<Embedding, Status> {
    let serde_value = proto.metadata
        .as_ref()
        .map(struct_to_serde_value)
        .transpose()?
        .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new()));
    
    // Convert metadata
    let mut metadata = HashMap::new();
    if let JsonValue::Object(obj) = serde_value {
        for (k, v) in obj {
            let value_str = match v {
                JsonValue::String(s) => s,
                _ => v.to_string(),
            };
            metadata.insert(k, value_str);
        }
    }
    
    Ok(Embedding {
        vector: proto.vector.clone(),
        model: proto.model.clone(),
        dimensions: proto.dimensions as usize,
        metadata,
    })
}

#[cfg(feature = "grpc")]
pub fn proto_to_embedding_model(proto_model: i32) -> Option<EmbeddingModel> {
    match proto_model {
        0 => Some(EmbeddingModel::E5MultilingualLargeInstruct),
        1 => Some(EmbeddingModel::GteModernBertBase),
        2 => Some(EmbeddingModel::JinaEmbeddingsV3),
        _ => None,
    }
}

#[cfg(feature = "grpc")]
pub fn embedding_model_to_proto(model: EmbeddingModel) -> i32 {
    match model {
        EmbeddingModel::E5MultilingualLargeInstruct => 0,
        EmbeddingModel::GteModernBertBase => 1,
        EmbeddingModel::JinaEmbeddingsV3 => 2,
        EmbeddingModel::Custom => 3,
    }
}

#[cfg(feature = "grpc")]
pub fn combination_method_to_proto(method: CombinationMethod) -> i32 {
    match method {
        CombinationMethod::Sum => 0,
        CombinationMethod::Max => 1,
        CombinationMethod::Weighted => 2,
    }
}

#[cfg(feature = "grpc")]
pub fn proto_to_combination_method(proto_method: i32) -> Option<CombinationMethod> {
    match proto_method {
        0 => Some(CombinationMethod::Sum),
        1 => Some(CombinationMethod::Max),
        2 => Some(CombinationMethod::Weighted),
        _ => None,
    }
}

// Helper functions for timestamp conversions
#[cfg(feature = "grpc")]
pub fn chrono_to_timestamp(datetime: DateTime<Utc>) -> Result<Timestamp, Status> {
    Ok(Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_subsec_nanos() as i32,
    })
}

#[cfg(feature = "grpc")]
pub fn timestamp_to_chrono(timestamp: &Timestamp) -> Result<DateTime<Utc>, Status> {
    DateTime::from_timestamp(timestamp.seconds, timestamp.nanos as u32)
        .ok_or_else(|| Status::invalid_argument("Invalid timestamp"))
}

// Helper functions for struct/value conversions
#[cfg(feature = "grpc")]
pub fn serde_value_to_struct(value: &JsonValue) -> Result<Option<Struct>, Status> {
    match value {
        JsonValue::Object(map) => {
            let mut fields = HashMap::new();
            
            for (k, v) in map {
                fields.insert(k.clone(), json_to_prost_value(v)?);
            }
            
            Ok(Some(Struct { fields }))
        }
        _ => Err(Status::invalid_argument("Expected object for struct conversion")),
    }
}

#[cfg(feature = "grpc")]
fn json_to_prost_value(value: &JsonValue) -> Result<ProstValue, Status> {
    match value {
        JsonValue::Null => Ok(ProstValue {
            kind: Some(prost_types::value::Kind::NullValue(0)),
        }),
        JsonValue::Bool(b) => Ok(ProstValue {
            kind: Some(prost_types::value::Kind::BoolValue(*b)),
        }),
        JsonValue::Number(n) => {
            if let Some(int) = n.as_i64() {
                Ok(ProstValue {
                    kind: Some(prost_types::value::Kind::NumberValue(int as f64)),
                })
            } else if let Some(float) = n.as_f64() {
                Ok(ProstValue {
                    kind: Some(prost_types::value::Kind::NumberValue(float)),
                })
            } else {
                Err(Status::invalid_argument("Invalid number format"))
            }
        }
        JsonValue::String(s) => Ok(ProstValue {
            kind: Some(prost_types::value::Kind::StringValue(s.clone())),
        }),
        JsonValue::Array(arr) => {
            let values = arr
                .iter()
                .map(json_to_prost_value)
                .collect::<Result<Vec<ProstValue>, Status>>()?;
                
            Ok(ProstValue {
                kind: Some(prost_types::value::Kind::ListValue(
                    prost_types::ListValue { values },
                )),
            })
        }
        JsonValue::Object(obj) => {
            let mut fields = HashMap::new();
            
            for (k, v) in obj {
                fields.insert(k.clone(), json_to_prost_value(v)?);
            }
            
            Ok(ProstValue {
                kind: Some(prost_types::value::Kind::StructValue(Struct { fields })),
            })
        }
    }
}

#[cfg(feature = "grpc")]
pub fn struct_to_serde_value(proto_struct: &Struct) -> Result<JsonValue, Status> {
    let mut map = serde_json::Map::new();
    
    for (k, v) in &proto_struct.fields {
        map.insert(k.clone(), prost_value_to_json(v)?);
    }
    
    Ok(JsonValue::Object(map))
}

#[cfg(feature = "grpc")]
fn prost_value_to_json(value: &ProstValue) -> Result<JsonValue, Status> {
    match &value.kind {
        Some(prost_types::value::Kind::NullValue(_)) => Ok(JsonValue::Null),
        Some(prost_types::value::Kind::BoolValue(b)) => Ok(JsonValue::Bool(*b)),
        Some(prost_types::value::Kind::NumberValue(n)) => {
            let number = serde_json::Number::from_f64(*n)
                .ok_or_else(|| Status::invalid_argument("Invalid number"))?;
            Ok(JsonValue::Number(number))
        }
        Some(prost_types::value::Kind::StringValue(s)) => Ok(JsonValue::String(s.clone())),
        Some(prost_types::value::Kind::ListValue(list)) => {
            let values = list
                .values
                .iter()
                .map(prost_value_to_json)
                .collect::<Result<Vec<JsonValue>, Status>>()?;
                
            Ok(JsonValue::Array(values))
        }
        Some(prost_types::value::Kind::StructValue(s)) => struct_to_serde_value(s),
        None => Err(Status::invalid_argument("No value kind specified")),
    }
}