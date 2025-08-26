# Test data

## Generation commands

Password

```
echo test_password > password
```

SSH keypairs

```
ssh-keygen -t rsa   -b 2048 -N ""                -f id_rsa
ssh-keygen -t ecdsa -b 256  -N ""                -f id_ecdsa
ssh-keygen -t ed25519       -N ""                -f id_ed25519
ssh-keygen -t ed25519       -N "test_passphrase" -f enc_ed25519
```

SSH certificate CA

```
ssh-keygen -t ed25519 -N "" -f ca
```

SSH certificates

```
ssh-keygen -s ca -I test_identity -n test_user -V 0x1:0x2000000000 id_ed25519.pub
ssh-keygen -s ca -I test_identity -n test_user -V 0x1:0x2000000000 enc_ed25519.pub
```