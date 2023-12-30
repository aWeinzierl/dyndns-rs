 DynDNS Service for GoDaddy
===========================

[![Chat](https://img.shields.io/badge/chat-%23ddsg:matrix.org-%2346BC99?logo=Matrix)](https://matrix.to/#/#ddsg:matrix.org)


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
                "domain_name": "matrix.org",
                "specifications": [
                    {
                        "host_name": "@",
                        "specifications": [
                            {
                                "record_type": "A"
                            },
                            {
                                "record_type": "AAAA"
                            }
                        ]
                    }
                ]
            }
        ]
    }
]
```
The JSON files need to be stored in `C:\Users\%username%\AppData\Roaming\Andreas Weinzierl\DynDns Service\` in Windows with the names `authentication.prefs.json` and `dns-entries.prefs.json`.