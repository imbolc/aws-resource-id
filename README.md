# aws-resource-id

[![License](https://img.shields.io/crates/l/aws-resource-id.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/aws-resource-id.svg)](https://crates.io/crates/aws-resource-id)
[![Docs.rs](https://docs.rs/aws-resource-id/badge.svg)](https://docs.rs/aws-resource-id)

`Copy`-able stack-only AWS resource IDs

## General format IDs

18 bytes stack-only IDs following the general `[prefix]-[8-or-17-chars-unique-string]` format.

| Type                                 | Prefix        | Details                           |
|--------------------------------------|---------------|-----------------------------------|
| [`AwsAmiId`]                         | `ami-`        | AMI (Amazon Machine Image)        |
| [`AwsNetworkAclId`]                  | `acl-`        | Network ACL (Access Control List) |
| [`AwsCustomerGatewayId`]             | `cgw-`        | Customer Gateway                  |
| [`AwsElasticIpId`]                   | `eipalloc-`   | Elastic IP                        |
| [`AwsEfsFileSystemId`]               | `fs-`         | EFS (Elastic File System)         |
| [`AwsEfsMountTargetId`]              | `fsmt-`       | EFS Mount Target ID               |
| [`AwsCloudFormationStackId`]         | `stack-`      | CloudFormation Stack              |
| [`AwsElasticBeanstalkEnvironmentId`] | `e-`          | Elastic Beanstalk Environment     |
| [`AwsInstanceId`]                    | `i-`          | EC2 Instance                      |
| [`AwsInternetGatewayId`]             | `igw-`        | Internet Gateway                  |
| [`AwsKeyPairId`]                     | `key-`        | Key Pair                          |
| [`AwsLoadBalancerId`]                | `elbv2-`      | Elastic Load Balancer             |
| [`AwsNatGatewayId`]                  | `nat-`        | NAT Gateway                       |
| [`AwsNetworkInterfaceId`]            | `eni-`        | Network Interface                 |
| [`AwsPlacementGroupId`]              | `pg-`         | Placement Group                   |
| [`AwsRdsInstanceId`]                 | `db-`         | RDS Instance                      |
| [`AwsRedshiftClusterId`]             | `redshift-`   | Redshift Cluster                  |
| [`AwsRouteTableId`]                  | `rtb-`        | Route Table                       |
| [`AwsSecurityGroupId`]               | `sg-`         | Security Group                    |
| [`AwsSnapshotId`]                    | `snap-`       | EBS Snapshot                      |
| [`AwsSubnetId`]                      | `subnet-`     | VPC Subnet                        |
| [`AwsTargetGroupId`]                 | `tg-`         | Target Group                      |
| [`AwsTransitGatewayAttachmentId`]    | `tgw-attach-` | Transit Gateway Attachment        |
| [`AwsTransitGatewayId`]              | `tgw-`        | Transit Gateway                   |
| [`AwsVolumeId`]                      | `vol-`        | EBS Volume                        |
| [`AwsVpcId`]                         | `vpc-`        | VPC (Virtual Private Cloud)       |
| [`AwsVpnConnectionId`]               | `vpn-`        | VPN Connection                    |
| [`AwsVpnGatewayId`]                  | `vgw-`        | VPN Gateway                       |


## Specific format ids

| Type                    | Size    | Stack-only | Example         | Details         |
|-------------------------|---------|------------|-----------------|-----------------|
| [`AwsRegionId`]         | 1 byte  | yes        | `eu-central-1`  | Region          |
| [`AwsAvailabilityZone`] | 6 bytes | yes        | `eu-central-1a` | Zone name or ID |

`AwsAvailabilityZone` parses zone names and IDs for supported regions.

- Standard Availability Zone name `us-east-1a` or ID `use1-az1`
- Local Zone name `us-west-2-lax-1a` or ID `usw2-lax1-az1`
- Wavelength Zone name `us-east-1-wl1-bos-wlz-1` or ID `use1-wl1-bos-wlz1`

Parsing validates syntax only. Location codes use three lowercase ASCII letters.
Numeric suffixes use decimal digits, start with a nonzero digit, and have a value
from 1 through 255.

Equality compares the exact identifier spelling. Ordering compares the region,
then format, then location and suffix.


## Contributing

- please run [.pre-commit.sh] before sending a PR, it will check everything


## License

This project is licensed under the [MIT license][license].

[.pre-commit.sh]: https://github.com/imbolc/aws-resource-id/blob/main/.pre-commit.sh
[license]: https://github.com/imbolc/aws-resource-id/blob/main/LICENSE
