#!/bin/sh
# Generate keys for server
openssl genpkey  -algorithm RSA -pkeyopt rsa_keygen_bits:2048 --out tuwunel-key.pem
openssl req -x509 -new -key tuwunel-key.pem -out tuwunel-cert.pem  