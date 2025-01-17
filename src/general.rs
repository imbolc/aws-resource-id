//! # AWS Resource IDs in a General Format
//!
//! This module handles AWS resource IDs that follow a specific format:
//!
//! 1. Prefix: a short string specific to each resource type (e.g., `ami-` for
//!    AMIs)
//! 2. Identifier: an 8 or 17 character unique string containing only:
//!    - Lowercase letters (a-z)
//!    - Numbers (0-9)
//!
//! ## Resource ID length
//!
//! > Prior to January 2016, the IDs assigned to newly created resources of
//! > certain resource types used 8 characters after the hyphen (for example,
//! > i-1a2b3c4d). From January 2016 to June 2018, we changed the IDs of these
//! > resource types to use 17 characters after the hyphen (for example,
//! > i-1234567890abcdef0). Depending on when your account was created, you
//! > might have some existing resources with short IDs, however, any new
//! > resources will receive the longer IDs.
//! > <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/resource-ids.html>
#[cfg(feature = "sqlx-postgres")]
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres, Type,
};
use std::{convert::TryFrom, fmt, str::FromStr};

/// Error encountered when parsing an AWS resource ID in the general format
#[derive(Debug, thiserror::Error)]
#[error("failed to initialize {target_type} from \"{input}\": {error_detail}")]
pub struct GeneralResourceError {
    /// The AWS resource type being parsed (e.g., [`AwsAmiId`])
    target_type: &'static str,
    /// The input string that failed to parse
    input: String,
    /// Detailed description of the error
    error_detail: GeneralResourceErrorDetail,
}

/// Specific details about errors encountered when parsing AWS resource IDs in
/// the general format
#[derive(Debug, thiserror::Error)]
pub enum GeneralResourceErrorDetail {
    /// Incorrect prefix for the resource type
    #[error("incorrect prefix, expected \"{0}\"")]
    WrongPrefix(&'static str),
    /// Invalid length of the unique identifier part
    #[error("the unique part must be 8 or 17, not {0} characters long")]
    IdLength(usize),
    /// The unique identifier contains invalid characters
    #[error("the unique part contains non ascii alphanumeric characters")]
    NonAsciiAlphanumeric,
}

/// The unique alphanumeric part of an AWS resource id in the general format
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum UniquePart {
    C8([u8; 8]),
    C17([u8; 17]),
}

impl UniquePart {
    fn as_slice(&self) -> &[u8] {
        match self {
            Self::C8(x) => x,
            Self::C17(x) => x,
        }
    }
}

macro_rules! impl_resource_id {
    ($type:ident, $prefix:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type(UniquePart);

        impl $type {
            const PREFIX: &'static str = $prefix;
        }

        impl TryFrom<&str> for $type {
            type Error = $crate::Error;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                if !s.starts_with(Self::PREFIX) {
                    return Err(GeneralResourceError::new(
                        short_type_name::<$type>(),
                        s,
                        GeneralResourceErrorDetail::WrongPrefix(Self::PREFIX),
                    )
                    .into());
                }
                if !s[Self::PREFIX.len()..]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric())
                {
                    return Err(GeneralResourceError::new(
                        short_type_name::<$type>(),
                        s,
                        GeneralResourceErrorDetail::NonAsciiAlphanumeric,
                    )
                    .into());
                }

                let id = &s[Self::PREFIX.len()..];
                if id.len() == 8 {
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(id.as_bytes());
                    Ok($type(UniquePart::C8(arr)))
                } else if id.len() == 17 {
                    let mut arr = [0u8; 17];
                    arr.copy_from_slice(id.as_bytes());
                    Ok($type(UniquePart::C17(arr)))
                } else {
                    Err(GeneralResourceError::new(
                        short_type_name::<$type>(),
                        s,
                        GeneralResourceErrorDetail::IdLength(id.len()),
                    )
                    .into())
                }
            }
        }

        impl TryFrom<String> for $type {
            type Error = $crate::Error;

            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::try_from(s.as_str())
            }
        }

        impl TryFrom<&String> for $type {
            type Error = $crate::Error;

            fn try_from(s: &String) -> Result<Self, Self::Error> {
                Self::try_from(s.as_str())
            }
        }

        impl FromStr for $type {
            type Err = $crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", Self::PREFIX)?;
                write!(
                    f,
                    "{}",
                    std::str::from_utf8(self.0.as_slice()).unwrap_or_default()
                )
            }
        }

        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(short_type_name::<Self>())
                    .field(&self.to_string())
                    .finish()
            }
        }

        impl From<$type> for String {
            fn from(value: $type) -> Self {
                value.to_string()
            }
        }

        #[cfg(feature = "sqlx-postgres")]
        impl Type<Postgres> for $type {
            fn type_info() -> PgTypeInfo {
                <String as Type<Postgres>>::type_info()
            }

            fn compatible(ty: &PgTypeInfo) -> bool {
                <String as Type<Postgres>>::compatible(ty)
            }
        }

        #[cfg(feature = "sqlx-postgres")]
        impl<'q> sqlx::encode::Encode<'q, Postgres> for $type {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                <String as sqlx::encode::Encode<Postgres>>::encode_by_ref(&self.to_string(), buf)
            }
        }

        #[cfg(feature = "sqlx-postgres")]
        impl<'r> sqlx::decode::Decode<'r, Postgres> for $type {
            fn decode(
                value: PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let s = <&str as sqlx::decode::Decode<Postgres>>::decode(value)?;
                Ok($type::try_from(s).map_err(|e| Box::new(sqlx::Error::Decode(e.into())))?)
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                $type::try_from(s).map_err(serde::de::Error::custom)
            }
        }
    };
}

fn short_type_name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}

impl GeneralResourceError {
    fn new(
        target_type: &'static str,
        input: impl Into<String>,
        error_detail: GeneralResourceErrorDetail,
    ) -> Self {
        Self {
            target_type,
            input: input.into(),
            error_detail,
        }
    }
}

impl_resource_id!(
    AwsNetworkAclId,
    "acl-",
    "AWS Network ACL (Access Control List) ID"
);
impl_resource_id!(AwsAmiId, "ami-", "AWS AMI (Amazon Machine Image) ID");
impl_resource_id!(AwsCustomerGatewayId, "cgw-", "AWS Customer Gateway ID");
impl_resource_id!(AwsElasticIpId, "eipalloc-", "AWS Elastic IP ID");
impl_resource_id!(
    AwsEfsFileSystemId,
    "fs-",
    "AWS EFS (Elastic File System) ID"
);
impl_resource_id!(AwsEfsMountTargetId, "fsmt-", "AWS EFS Mount Target ID");
impl_resource_id!(
    AwsCloudFormationStackId,
    "stack-",
    "AWS CloudFormation Stack ID"
);
impl_resource_id!(
    AwsElasticBeanstalkEnvironmentId,
    "e-",
    "AWS Elastic Beanstalk Environment ID"
);
impl_resource_id!(AwsInstanceId, "i-", "AWS EC2 Instance ID");
impl_resource_id!(AwsInternetGatewayId, "igw-", "AWS Internet Gateway ID");
impl_resource_id!(AwsKeyPairId, "key-", "AWS Key Pair ID");
impl_resource_id!(AwsLoadBalancerId, "elbv2-", "AWS Elastic Load Balancer ID");
impl_resource_id!(AwsNatGatewayId, "nat-", "AWS NAT Gateway ID");
impl_resource_id!(AwsNetworkInterfaceId, "eni-", "AWS Network Interface ID");
impl_resource_id!(AwsPlacementGroupId, "pg-", "AWS Placement Group ID");
impl_resource_id!(AwsRdsInstanceId, "db-", "AWS RDS Instance ID");
impl_resource_id!(AwsRedshiftClusterId, "redshift-", "AWS Redshift Cluster ID");
impl_resource_id!(AwsRouteTableId, "rtb-", "AWS Route Table ID");
impl_resource_id!(AwsSecurityGroupId, "sg-", "AWS Security Group ID");
impl_resource_id!(AwsSnapshotId, "snap-", "AWS EBS Snapshot ID");
impl_resource_id!(AwsSubnetId, "subnet-", "AWS VPC Subnet ID");
impl_resource_id!(AwsTargetGroupId, "tg-", "AWS Target Group ID");
impl_resource_id!(
    AwsTransitGatewayAttachmentId,
    "tgw-attach-",
    "AWS Transit Gateway Attachment ID"
);
impl_resource_id!(AwsTransitGatewayId, "tgw-", "AWS Transit Gateway ID");
impl_resource_id!(AwsVolumeId, "vol-", "AWS EBS Volume ID");
impl_resource_id!(AwsVpcId, "vpc-", "AWS VPC (Virtual Private Cloud) ID");
impl_resource_id!(AwsVpnConnectionId, "vpn-", "AWS VPN Connection ID");
impl_resource_id!(AwsVpnGatewayId, "vgw-", "AWS VPN Gateway ID");

#[cfg(test)]
mod tests {
    use super::*;

    fn ami(s: &str) -> AwsAmiId {
        AwsAmiId::try_from(s).unwrap()
    }

    #[test]
    fn test_eq() {
        assert_eq!(ami("ami-12345678"), ami("ami-12345678"));
        assert_ne!(ami("ami-12345678"), ami("ami-abcdefgh"));
    }

    #[test]
    fn test_fmt_display() {
        assert_eq!(format!("{}", ami("ami-12345678")), "ami-12345678");
    }

    #[test]
    fn test_fmt_debug() {
        assert_eq!(
            format!("{:?}", ami("ami-12345678")),
            r#"AwsAmiId("ami-12345678")"#
        );
    }

    #[test]
    fn test_into_string() {
        let s: String = ami("ami-12345678").into();
        assert_eq!(s, "ami-12345678");
    }

    #[test]
    fn test_tryfrom_str() {
        assert!(AwsAmiId::try_from("ami-12345678").is_ok());
    }

    #[test]
    fn test_tryfrom_string() {
        assert!(AwsAmiId::try_from("ami-12345678".to_string()).is_ok());
    }

    #[test]
    fn test_tryfrom_refstring() {
        assert!(AwsAmiId::try_from(&"ami-12345678".to_string()).is_ok());
    }

    #[test]
    fn test_fromstr() {
        assert!("ami-12345678".parse::<AwsAmiId>().is_ok(),);
        assert!("ami-12345678".to_string().parse::<AwsAmiId>().is_ok(),);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ami("ami-12345678")).unwrap(),
            "\"ami-12345678\""
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<AwsAmiId>("\"ami-12345678\"").unwrap(),
            ami("ami-12345678"),
        );
    }

    #[test]
    fn test_wrong_prefix() {
        let result = AwsAmiId::try_from("amx-12345678");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "failed to initialize AwsAmiId from \"amx-12345678\": incorrect prefix, expected \"ami-\""
        );
    }

    #[test]
    fn test_error_wrong_length() {
        let result = AwsAmiId::try_from("ami-1234567");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "failed to initialize AwsAmiId from \"ami-1234567\": the unique part must be 8 or 17, not 7 characters long"
        );

        let result = AwsAmiId::try_from("ami-123456789012345678");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "failed to initialize AwsAmiId from \"ami-123456789012345678\": the unique part must be 8 or 17, not 18 characters long"
        );
    }

    #[test]
    fn test_error_non_alphanumeric() {
        let result = AwsAmiId::try_from("ami-1234567!");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
           "failed to initialize AwsAmiId from \"ami-1234567!\": the unique part contains non ascii alphanumeric characters"
        );
    }

    #[test]
    fn test_valid_short_ids() {
        assert_eq!(
            AwsNetworkAclId::try_from("acl-1234abcd")
                .unwrap()
                .to_string(),
            "acl-1234abcd"
        );
        assert_eq!(
            AwsAmiId::try_from("ami-1234abcd").unwrap().to_string(),
            "ami-1234abcd"
        );
        assert_eq!(
            AwsCustomerGatewayId::try_from("cgw-1234abcd")
                .unwrap()
                .to_string(),
            "cgw-1234abcd"
        );
        assert_eq!(
            AwsElasticIpId::try_from("eipalloc-1234abcd")
                .unwrap()
                .to_string(),
            "eipalloc-1234abcd"
        );
        assert_eq!(
            AwsEfsFileSystemId::try_from("fs-1234abcd")
                .unwrap()
                .to_string(),
            "fs-1234abcd"
        );
        assert_eq!(
            AwsEfsMountTargetId::try_from("fsmt-1234abcd")
                .unwrap()
                .to_string(),
            "fsmt-1234abcd"
        );
        assert_eq!(
            AwsCloudFormationStackId::try_from("stack-1234abcd")
                .unwrap()
                .to_string(),
            "stack-1234abcd"
        );
        assert_eq!(
            AwsElasticBeanstalkEnvironmentId::try_from("e-1234abcd")
                .unwrap()
                .to_string(),
            "e-1234abcd"
        );
        assert_eq!(
            AwsInstanceId::try_from("i-1234abcd").unwrap().to_string(),
            "i-1234abcd"
        );
        assert_eq!(
            AwsInternetGatewayId::try_from("igw-1234abcd")
                .unwrap()
                .to_string(),
            "igw-1234abcd"
        );
        assert_eq!(
            AwsKeyPairId::try_from("key-1234abcd").unwrap().to_string(),
            "key-1234abcd"
        );
        assert_eq!(
            AwsLoadBalancerId::try_from("elbv2-1234abcd")
                .unwrap()
                .to_string(),
            "elbv2-1234abcd"
        );
        assert_eq!(
            AwsNatGatewayId::try_from("nat-1234abcd")
                .unwrap()
                .to_string(),
            "nat-1234abcd"
        );
        assert_eq!(
            AwsNetworkInterfaceId::try_from("eni-1234abcd")
                .unwrap()
                .to_string(),
            "eni-1234abcd"
        );
        assert_eq!(
            AwsPlacementGroupId::try_from("pg-1234abcd")
                .unwrap()
                .to_string(),
            "pg-1234abcd"
        );
        assert_eq!(
            AwsRdsInstanceId::try_from("db-1234abcd")
                .unwrap()
                .to_string(),
            "db-1234abcd"
        );
        assert_eq!(
            AwsRedshiftClusterId::try_from("redshift-1234abcd")
                .unwrap()
                .to_string(),
            "redshift-1234abcd"
        );
        assert_eq!(
            AwsRouteTableId::try_from("rtb-1234abcd")
                .unwrap()
                .to_string(),
            "rtb-1234abcd"
        );
        assert_eq!(
            AwsSecurityGroupId::try_from("sg-1234abcd")
                .unwrap()
                .to_string(),
            "sg-1234abcd"
        );
        assert_eq!(
            AwsSnapshotId::try_from("snap-1234abcd")
                .unwrap()
                .to_string(),
            "snap-1234abcd"
        );
        assert_eq!(
            AwsSubnetId::try_from("subnet-1234abcd")
                .unwrap()
                .to_string(),
            "subnet-1234abcd"
        );
        assert_eq!(
            AwsTargetGroupId::try_from("tg-1234abcd")
                .unwrap()
                .to_string(),
            "tg-1234abcd"
        );
        assert_eq!(
            AwsTransitGatewayAttachmentId::try_from("tgw-attach-1234abcd")
                .unwrap()
                .to_string(),
            "tgw-attach-1234abcd"
        );
        assert_eq!(
            AwsTransitGatewayId::try_from("tgw-1234abcd")
                .unwrap()
                .to_string(),
            "tgw-1234abcd"
        );
        assert_eq!(
            AwsVolumeId::try_from("vol-1234abcd").unwrap().to_string(),
            "vol-1234abcd"
        );
        assert_eq!(
            AwsVpcId::try_from("vpc-1234abcd").unwrap().to_string(),
            "vpc-1234abcd"
        );
        assert_eq!(
            AwsVpnConnectionId::try_from("vpn-1234abcd")
                .unwrap()
                .to_string(),
            "vpn-1234abcd"
        );
        assert_eq!(
            AwsVpnGatewayId::try_from("vgw-1234abcd")
                .unwrap()
                .to_string(),
            "vgw-1234abcd"
        );
    }

    #[test]
    fn test_valid_long_ids() {
        assert_eq!(
            AwsNetworkAclId::try_from("acl-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "acl-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsAmiId::try_from("ami-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "ami-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsCustomerGatewayId::try_from("cgw-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "cgw-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsElasticIpId::try_from("eipalloc-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "eipalloc-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsEfsFileSystemId::try_from("fs-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "fs-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsEfsMountTargetId::try_from("fsmt-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "fsmt-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsCloudFormationStackId::try_from("stack-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "stack-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsElasticBeanstalkEnvironmentId::try_from("e-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "e-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsInstanceId::try_from("i-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "i-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsInternetGatewayId::try_from("igw-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "igw-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsKeyPairId::try_from("key-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "key-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsLoadBalancerId::try_from("elbv2-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "elbv2-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsNatGatewayId::try_from("nat-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "nat-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsNetworkInterfaceId::try_from("eni-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "eni-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsPlacementGroupId::try_from("pg-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "pg-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsRdsInstanceId::try_from("db-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "db-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsRedshiftClusterId::try_from("redshift-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "redshift-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsRouteTableId::try_from("rtb-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "rtb-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsSecurityGroupId::try_from("sg-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "sg-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsSnapshotId::try_from("snap-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "snap-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsSubnetId::try_from("subnet-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "subnet-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsTargetGroupId::try_from("tg-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "tg-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsTransitGatewayAttachmentId::try_from("tgw-attach-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "tgw-attach-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsTransitGatewayId::try_from("tgw-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "tgw-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsVolumeId::try_from("vol-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "vol-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsVpcId::try_from("vpc-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "vpc-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsVpnConnectionId::try_from("vpn-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "vpn-1a2b3c4d5e6f7j8h9"
        );
        assert_eq!(
            AwsVpnGatewayId::try_from("vgw-1a2b3c4d5e6f7j8h9")
                .unwrap()
                .to_string(),
            "vgw-1a2b3c4d5e6f7j8h9"
        );
    }
}

#[cfg(feature = "sqlx-postgres")]
#[cfg(test)]
mod sqlx_tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn serialize_varchar(pool: PgPool) -> sqlx::Result<()> {
        let ami_str = "ami-12345678";
        let ami: AwsAmiId = ami_str.parse().unwrap();
        let serialized = sqlx::query_scalar!("SELECT $1::varchar", ami as _)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(serialized, ami_str);
        Ok(())
    }

    #[sqlx::test]
    async fn serialize_text(pool: PgPool) -> sqlx::Result<()> {
        let ami_str = "ami-12345678";
        let ami: AwsAmiId = ami_str.parse().unwrap();
        let serialized = sqlx::query_scalar!("SELECT $1::text", ami as _)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(serialized, ami_str);
        Ok(())
    }

    #[sqlx::test]
    async fn deserialize_varchar(pool: PgPool) -> sqlx::Result<()> {
        let ami: AwsAmiId = "ami-12345678".parse().unwrap();
        let deserialized =
            sqlx::query_scalar!(r#"SELECT 'ami-12345678'::varchar as "val: AwsAmiId""#)
                .fetch_one(&pool)
                .await?
                .unwrap();
        assert_eq!(deserialized, ami);
        Ok(())
    }

    #[sqlx::test]
    async fn deserialize_text(pool: PgPool) -> sqlx::Result<()> {
        let ami: AwsAmiId = "ami-12345678".parse().unwrap();
        let deserialized = sqlx::query_scalar!(r#"SELECT 'ami-12345678' as "val: AwsAmiId""#)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(deserialized, ami);
        Ok(())
    }
}
