 DynDNS Service
===============

[![Matrix](https://img.shields.io/matrix/dyndns-rs%3Amatrix.org?style=for-the-badge&logo=matrix&label=dyndns-rs)](https://matrix.to/#/#dyndns-rs:matrix.org)


Login data example configuration:

```JSON
[
    {
        "YDns":{
            "username": "username",
            "secret": "secret"
        }
    }
]
```

DNS-record specification example configuration:

```JSON
[
    {
        "YDns": [
            {
                "domain_name": "test-domain.de",
                "specifications": [
                    {
                        "host_name": "@",
                        "ipv4": null,
                        "ipv6": {
                            "record_specification": {},
                            "custom_interface_id": "::1"
                        }
                    },
                    {
                        "host_name": "test12345",
                        "ipv4": {}
                        "ipv6": {
                            "record_specification": {},
                        }
                    }
                ]
            }
        ]
    }
]
```

The JSON files need to be stored in `C:\Users\%username%\AppData\Roaming\Andreas Weinzierl\DynDns Service\` on Windows with the names `authentication.prefs.json` and `dns-entries.prefs.json`.
Linux requires these files in `/home/user/.config/DynDns Service/`.
