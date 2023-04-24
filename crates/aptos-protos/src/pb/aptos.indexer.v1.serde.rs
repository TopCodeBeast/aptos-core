// Copyright © Aptos Foundation

// @generated
impl serde::Serialize for ChainMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.ChainMetadata", len)?;
        if let Some(v) = self.chain_id.as_ref() {
            struct_ser.serialize_field("chainId", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.ChainMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ChainMetadata {
                    chain_id: chain_id__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.ChainMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.last_version_processed.is_some() {
            len += 1;
        }
        if self.last_version_timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.DataStatus", len)?;
        if let Some(v) = self.last_version_processed.as_ref() {
            struct_ser.serialize_field("lastVersionProcessed", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.last_version_timestamp.as_ref() {
            struct_ser.serialize_field("lastVersionTimestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "last_version_processed",
            "lastVersionProcessed",
            "last_version_timestamp",
            "lastVersionTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LastVersionProcessed,
            LastVersionTimestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "lastVersionProcessed" | "last_version_processed" => Ok(GeneratedField::LastVersionProcessed),
                            "lastVersionTimestamp" | "last_version_timestamp" => Ok(GeneratedField::LastVersionTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.DataStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DataStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut last_version_processed__ = None;
                let mut last_version_timestamp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LastVersionProcessed => {
                            if last_version_processed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastVersionProcessed"));
                            }
                            last_version_processed__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LastVersionTimestamp => {
                            if last_version_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastVersionTimestamp"));
                            }
                            last_version_timestamp__ = map.next_value()?;
                        }
                    }
                }
                Ok(DataStatus {
                    last_version_processed: last_version_processed__,
                    last_version_timestamp: last_version_timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.DataStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.starting_version.is_some() {
            len += 1;
        }
        if self.transactions_count.is_some() {
            len += 1;
        }
        if self.batch_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.GetTransactionsRequest", len)?;
        if let Some(v) = self.starting_version.as_ref() {
            struct_ser.serialize_field("startingVersion", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.transactions_count.as_ref() {
            struct_ser.serialize_field("transactionsCount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.batch_size.as_ref() {
            struct_ser.serialize_field("batchSize", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "starting_version",
            "startingVersion",
            "transactions_count",
            "transactionsCount",
            "batch_size",
            "batchSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartingVersion,
            TransactionsCount,
            BatchSize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "startingVersion" | "starting_version" => Ok(GeneratedField::StartingVersion),
                            "transactionsCount" | "transactions_count" => Ok(GeneratedField::TransactionsCount),
                            "batchSize" | "batch_size" => Ok(GeneratedField::BatchSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.GetTransactionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut starting_version__ = None;
                let mut transactions_count__ = None;
                let mut batch_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StartingVersion => {
                            if starting_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startingVersion"));
                            }
                            starting_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TransactionsCount => {
                            if transactions_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionsCount"));
                            }
                            transactions_count__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BatchSize => {
                            if batch_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchSize"));
                            }
                            batch_size__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(GetTransactionsRequest {
                    starting_version: starting_version__,
                    transactions_count: transactions_count__,
                    batch_size: batch_size__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.GetTransactionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.GetTransactionsResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                get_transactions_response::Response::Data(v) => {
                    struct_ser.serialize_field("data", v)?;
                }
                get_transactions_response::Response::Status(v) => {
                    struct_ser.serialize_field("status", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.GetTransactionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(get_transactions_response::Response::Data)
;
                        }
                        GeneratedField::Status => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(get_transactions_response::Response::Status)
;
                        }
                    }
                }
                Ok(GetTransactionsResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.GetTransactionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_transactions_response::GetTransactionsData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.transactions.is_empty() {
            len += 1;
        }
        if self.chain_metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.GetTransactionsResponse.GetTransactionsData", len)?;
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.transactions.is_empty() {
            struct_ser.serialize_field("transactions", &self.transactions)?;
        }
        if let Some(v) = self.chain_metadata.as_ref() {
            struct_ser.serialize_field("chainMetadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_transactions_response::GetTransactionsData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "timestamp",
            "transactions",
            "chain_metadata",
            "chainMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Timestamp,
            Transactions,
            ChainMetadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "transactions" => Ok(GeneratedField::Transactions),
                            "chainMetadata" | "chain_metadata" => Ok(GeneratedField::ChainMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_transactions_response::GetTransactionsData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.GetTransactionsResponse.GetTransactionsData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_transactions_response::GetTransactionsData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut timestamp__ = None;
                let mut transactions__ = None;
                let mut chain_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Transactions => {
                            if transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainMetadata => {
                            if chain_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainMetadata"));
                            }
                            chain_metadata__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_transactions_response::GetTransactionsData {
                    version: version__,
                    timestamp: timestamp__,
                    transactions: transactions__.unwrap_or_default(),
                    chain_metadata: chain_metadata__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.GetTransactionsResponse.GetTransactionsData", FIELDS, GeneratedVisitor)
    }
}
