# Modaws

Command line utility for transformation of AWS service models.

Models for AWS services can be found here:

https://github.com/boto/botocore/tree/develop/botocore/data

## Usage

To output JSON without formatting with documentation removed:

    modaws --compact --strip-documentation FILE

output is to standard out.