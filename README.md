# aws-resource-id

[![License](https://img.shields.io/crates/l/aws-resource-id.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/aws-resource-id.svg)](https://crates.io/crates/aws-resource-id)
[![Docs.rs](https://docs.rs/aws-resource-id/badge.svg)](https://docs.rs/aws-resource-id)

`Copy`-able AWS resource IDs

## General format IDs

18 bytes stack-only IDs following the general `[prefix]-[8-or-17-chars-unique-string]` format.

| Type                                 | Prefix        | Details                               |
|--------------------------------------|---------------|---------------------------------------|
| [`AwsAmiId`]                         | `ami-`        | AWS AMI (Amazon Machine Image)        |
| [`AwsNetworkAclId`]                  | `acl-`        | AWS Network ACL (Access Control List) |
| [`AwsCustomerGatewayId`]             | `cgw-`        | AWS Customer Gateway                  |
| [`AwsElasticIpId`]                   | `eipalloc-`   | AWS Elastic IP                        |
| [`AwsEfsFileSystemId`]               | `fs-`         | AWS EFS (Elastic File System)         |
| [`AwsCloudFormationStackId`]         | `stack-`      | AWS CloudFormation Stack              |
| [`AwsElasticBeanstalkEnvironmentId`] | `e-`          | AWS Elastic Beanstalk Environment     |
| [`AwsInstanceId`]                    | `i-`          | AWS EC2 Instance                      |
| [`AwsInternetGatewayId`]             | `igw-`        | AWS Internet Gateway                  |
| [`AwsKeyPairId`]                     | `key-`        | AWS Key Pair                          |
| [`AwsLoadBalancerId`]                | `elbv2-`      | AWS Elastic Load Balancer             |
| [`AwsNatGatewayId`]                  | `nat-`        | AWS NAT Gateway                       |
| [`AwsNetworkInterfaceId`]            | `eni-`        | AWS Network Interface                 |
| [`AwsPlacementGroupId`]              | `pg-`         | AWS Placement Group                   |
| [`AwsRdsInstanceId`]                 | `db-`         | AWS RDS Instance                      |
| [`AwsRedshiftClusterId`]             | `redshift-`   | AWS Redshift Cluster                  |
| [`AwsRouteTableId`]                  | `rtb-`        | AWS Route Table                       |
| [`AwsSecurityGroupId`]               | `sg-`         | AWS Security Group                    |
| [`AwsSnapshotId`]                    | `snap-`       | AWS EBS Snapshot                      |
| [`AwsSubnetId`]                      | `subnet-`     | AWS VPC Subnet                        |
| [`AwsTargetGroupId`]                 | `tg-`         | AWS Target Group                      |
| [`AwsTransitGatewayAttachmentId`]    | `tgw-attach-` | AWS Transit Gateway Attachment        |
| [`AwsTransitGatewayId`]              | `tgw-`        | AWS Transit Gateway                   |
| [`AwsVolumeId`]                      | `vol-`        | AWS EBS Volume                        |
| [`AwsVpcId`]                         | `vpc-`        | AWS VPC (Virtual Private Cloud)       |
| [`AwsVpnConnectionId`]               | `vpn-`        | AWS VPN Connection                    |
| [`AwsVpnGatewayId`]                  | `vgw-`        | AWS VPN Gateway                       |


## Contributing

- please run [.pre-commit.sh] before sending a PR, it will check everything


## License

This project is licensed under the [MIT license][license].

[.pre-commit.sh]: https://github.com/imbolc/aws-resource-id/blob/main/.pre-commit.sh
[license]: https://github.com/imbolc/aws-resource-id/blob/main/LICENSE
