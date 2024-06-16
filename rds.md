頑張ってたけど全然動かなかった
この rds 自体は awesome-caches でやってる

```
user: root …/Owner/work/private/test/rust-playground on  master [!?] is 📦 v0.1.0 via 🦀 v1.79.0 took 7s
❯ aws rds describe-db-instances --query "DBInstances[*].{DBInstanceIdentifier:DBInstanceIdentifier,Endpoint:Endpoint.Address,Port:Endpoint.Port}" --output table

You must specify a region. You can also configure your region by running "aws configure".

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 5s
❯ aws rds describe-db-instances --query "DBInstances[*].{DBInstanceIdentifier:DBInstanceIdentifier,Endpoint:Endpoint.Address,Port:Endpoint.Port}" --output table --profile ankokuyakusyo
----------------------------------------------------------------------------------------------------------------
|                                              DescribeDBInstances                                             |
+---------------------------+--------------------------------------------------------------------------+-------+
|   DBInstanceIdentifier    |                                Endpoint                                  | Port  |
+---------------------------+--------------------------------------------------------------------------+-------+
|  rdswriterclusterinstence |  rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com  |  3306 |
+---------------------------+--------------------------------------------------------------------------+-------+

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ aws rds describe-db-instances --db-instance-identifier rdswriterclusterinstence --query "DBInstances[0].MasterUsername" --output text --profile ankokuyakusyo
admin

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ aws rds modify-db-instance --db-instance-identifier rdswriterclusterinstence --master-new-password password

usage: aws [options] <command> <subcommand> [<subcommand> ...] [parameters]
To see help text, you can run:

  aws help
  aws <command> help
  aws <command> <subcommand> help

Unknown options: --master-new-password, password


user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ aws rds modify-db-instance --db-instance-identifier rdswriterclusterinstence --master-user-password password

You must specify a region. You can also configure your region by running "aws configure".

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 4s
❯ aws rds modify-db-instance --db-instance-identifier rdswriterclusterinstence --master-user-password password --profile ankokuyakusyo

An error occurred (InvalidParameterCombination) when calling the ModifyDBInstance operation: The specified DB Instance is a member of a cluster. Modify master user password for the DB Cluster using the ModifyDbCluster API

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ aws rds describe-db-instances --db-instance-identifier rdswriterclusterinstence --profile ankokuyakusyo --query "DBInstances[0].DBClusterIdentifier"
"cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx"

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ aws rds modify-db-cluster --db-cluster-identifier cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx --master-user-password password

You must specify a region. You can also configure your region by running "aws configure".

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 4s
❯ aws rds modify-db-cluster --db-cluster-identifier cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx --master-user-password password --profile ankokuyakusyo
{
    "DBCluster": {
        "AllocatedStorage": 1,
        "AvailabilityZones": [
            "ap-northeast-1c",
            "ap-northeast-1a",
            "ap-northeast-1d"
        ],
        "BackupRetentionPeriod": 7,
        "DBClusterIdentifier": "cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx",
        "DBClusterParameterGroup": "cdkstack-rdsparametergroup973ba941-x2ietqb808y9",
        "DBSubnetGroup": "cdkstack-rdsdbclustersubnets6bdaaac1-blr120a3jn4d",
        "Status": "available",
        "EarliestRestorableTime": "2024-06-10T20:52:37.962000+00:00",
        "Endpoint": "cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx.cluster-cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com",
        "ReaderEndpoint": "cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx.cluster-ro-cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com",
        "MultiAZ": false,
        "Engine": "aurora-mysql",
        "EngineVersion": "8.0.mysql_aurora.3.05.2",
        "LatestRestorableTime": "2024-06-16T08:31:55.733000+00:00",
        "Port": 3306,
        "MasterUsername": "admin",
        "PreferredBackupWindow": "18:03-18:33",
        "PreferredMaintenanceWindow": "mon:19:52-mon:20:22",
        "ReadReplicaIdentifiers": [],
        "DBClusterMembers": [
            {
                "DBInstanceIdentifier": "rdswriterclusterinstence",
                "IsClusterWriter": true,
                "DBClusterParameterGroupStatus": "in-sync",
                "PromotionTier": 0
            }
        ],
        "VpcSecurityGroups": [
            {
                "VpcSecurityGroupId": "sg-063dd7e07a377981d",
                "Status": "active"
            }
        ],
        "HostedZoneId": "Z24O6O9L7SGTNB",
        "StorageEncrypted": false,
        "DbClusterResourceId": "cluster-XHSD3CJ4MZCYVEDU4BFLC2ZUDQ",
        "DBClusterArn": "arn:aws:rds:ap-northeast-1:459916669961:cluster:cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx",
        "AssociatedRoles": [],
        "IAMDatabaseAuthenticationEnabled": false,
        "ClusterCreateTime": "2024-06-10T20:51:13.998000+00:00",
        "EngineMode": "provisioned",
        "DeletionProtection": false,
        "HttpEndpointEnabled": false,
        "CopyTagsToSnapshot": true,
        "CrossAccountClone": false,
        "DomainMemberships": [],
        "TagList": [
            {
                "Key": "aws:cloudformation:stack-id",
                "Value": "arn:aws:cloudformation:ap-northeast-1:459916669961:stack/CdkStack/40735b80-f76a-11ee-9643-068dbe80647d"
            },
            {
                "Key": "aws:cloudformation:stack-name",
                "Value": "CdkStack"
            },
            {
                "Key": "aws:cloudformation:logical-id",
                "Value": "RDSDBClusterB9DCA0ED"
            }
        ],
        "PendingModifiedValues": {
            "MasterUserPassword": "****"
        },
        "AutoMinorVersionUpgrade": true,
        "NetworkType": "IPV4",
        "LocalWriteForwardingStatus": "disabled"
    }
}

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 5s
❯ aws rds modify-db-cluster --db-cluster-identifier cdkstack-rdsdbclusterb9dca0ed-5negrjz0xqbx --master-user-password password --profile ankokuyakusyo^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:

^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 35s
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:
^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 8s
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p password
Enter password:
^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 44s
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:
^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 36s
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:

ERROR 2003 (HY000): Can't connect to MySQL server on 'rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com:3306' (110)

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 2m15s
❯

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:
^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 9s
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:
^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 8s
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:
^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 40s
❯ mysql -h rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com -u admin -p
Enter password:
^C

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 2m55s
❯ netstat -an | grep 3306
bash: netstat: command not found

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ netstat -an | grep 3306telnet rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com 3306
^C
telnet rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com 3306

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ telnet rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com 3306
Trying 10.0.200.163...
telnet: Unable to connect to remote host: Connection timed out

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 2m11s
❯ aws rds get-temp-auth-string --profile ankokuyakusyo

usage: aws [options] <command> <subcommand> [<subcommand> ...] [parameters]
To see help text, you can run:

  aws help
  aws <command> help
  aws <command> <subcommand> help

aws: error: argument operation: Invalid choice, valid choices are:

add-role-to-db-cluster                   | add-role-to-db-instance
add-source-identifier-to-subscription    | add-tags-to-resource
apply-pending-maintenance-action         | authorize-db-security-group-ingress
backtrack-db-cluster                     | cancel-export-task
copy-db-cluster-parameter-group          | copy-db-cluster-snapshot
copy-db-parameter-group                  | copy-db-snapshot
copy-option-group                        | create-blue-green-deployment
create-custom-db-engine-version          | create-db-cluster
create-db-cluster-endpoint               | create-db-cluster-parameter-group
create-db-cluster-snapshot               | create-db-instance
create-db-instance-read-replica          | create-db-parameter-group
create-db-proxy                          | create-db-proxy-endpoint
create-db-security-group                 | create-db-snapshot
create-db-subnet-group                   | create-event-subscription
create-global-cluster                    | create-option-group
delete-blue-green-deployment             | delete-custom-db-engine-version
delete-db-cluster                        | delete-db-cluster-automated-backup
delete-db-cluster-endpoint               | delete-db-cluster-parameter-group
delete-db-cluster-snapshot               | delete-db-instance
delete-db-instance-automated-backup      | delete-db-parameter-group
delete-db-proxy                          | delete-db-proxy-endpoint
delete-db-security-group                 | delete-db-snapshot
delete-db-subnet-group                   | delete-event-subscription
delete-global-cluster                    | delete-option-group
deregister-db-proxy-targets              | describe-account-attributes
describe-blue-green-deployments          | describe-certificates
describe-db-cluster-automated-backups    | describe-db-cluster-backtracks
describe-db-cluster-endpoints            | describe-db-cluster-parameter-groups
describe-db-cluster-parameters           | describe-db-cluster-snapshot-attributes
describe-db-cluster-snapshots            | describe-db-clusters
describe-db-engine-versions              | describe-db-instance-automated-backups
describe-db-instances                    | describe-db-log-files
describe-db-parameter-groups             | describe-db-parameters
describe-db-proxies                      | describe-db-proxy-endpoints
describe-db-proxy-target-groups          | describe-db-proxy-targets
describe-db-security-groups              | describe-db-snapshot-attributes
describe-db-snapshots                    | describe-db-subnet-groups
describe-engine-default-cluster-parameters | describe-engine-default-parameters
describe-event-categories                | describe-event-subscriptions
describe-events                          | describe-export-tasks
describe-global-clusters                 | describe-option-group-options
describe-option-groups                   | describe-orderable-db-instance-options
describe-pending-maintenance-actions     | describe-reserved-db-instances
describe-reserved-db-instances-offerings | describe-source-regions
describe-valid-db-instance-modifications | download-db-log-file-portion
failover-db-cluster                      | failover-global-cluster
list-tags-for-resource                   | modify-activity-stream
modify-certificates                      | modify-current-db-cluster-capacity
modify-custom-db-engine-version          | modify-db-cluster
modify-db-cluster-endpoint               | modify-db-cluster-parameter-group
modify-db-cluster-snapshot-attribute     | modify-db-instance
modify-db-parameter-group                | modify-db-proxy
modify-db-proxy-endpoint                 | modify-db-proxy-target-group
modify-db-snapshot                       | modify-db-snapshot-attribute
modify-db-subnet-group                   | modify-event-subscription
modify-global-cluster                    | promote-read-replica
promote-read-replica-db-cluster          | purchase-reserved-db-instances-offering
reboot-db-cluster                        | reboot-db-instance
register-db-proxy-targets                | remove-from-global-cluster
remove-role-from-db-cluster              | remove-role-from-db-instance
remove-source-identifier-from-subscription | remove-tags-from-resource
reset-db-cluster-parameter-group         | reset-db-parameter-group
restore-db-cluster-from-s3               | restore-db-cluster-from-snapshot
restore-db-cluster-to-point-in-time      | restore-db-instance-from-db-snapshot
restore-db-instance-from-s3              | restore-db-instance-to-point-in-time
revoke-db-security-group-ingress         | start-activity-stream
start-db-cluster                         | start-db-instance
start-db-instance-automated-backups-replication | start-export-task
stop-activity-stream                     | stop-db-cluster
stop-db-instance                         | stop-db-instance-automated-backups-replication
switchover-blue-green-deployment         | switchover-read-replica
add-option-to-option-group               | remove-option-from-option-group
generate-db-auth-token                   | wait
help


user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0
❯ telnet rdswriterclusterinstence.cf06sa0co2zy.ap-northeast-1.rds.amazonaws.com 3306
Trying 10.0.200.163...
telnet: Unable to connect to remote host: Connection timed out

user: root …/Owner/work/private/test/rust-playground on  master [!] is 📦 v0.1.0 via 🦀 v1.79.0 took 2m14s
❯
```
