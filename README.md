 DynDNS Service for GoDaddy
===========================

[![Chat](https://img.shields.io/badge/chat-%23ddsg:matrix.org-%2346BC99?logo=Matrix)](https://matrix.to/#/#ddsg:matrix.org)


Login data example configuration:
```JSON
[
  {
    "GoDaddy": {
      "api_key": "Your API-key",
      "api_secret": "Your API-secret",
      "api_url": {
     // "CustomUrl": "https://matrix.org/"
        "PredefinedUrl": "Production"
      }
    }
  }
]
```

DNS-record specification example configuration:
```JSON
[
  {
    "domain_name": "github.com",
    "specifications": [
      {
        "host_name": "@",
        "specifications": [
          {
            "record_type": "AAAA",
            "ttl": 600
          },
          {
            "record_type": "A",
            "ttl": 600
          }
        ]
      },
      {
        "host_name": "test",
        "specifications": [
          {
            "record_type": "AAAA",
            "ttl": 600
          },
          {
            "record_type": "A",
            "ttl": 600
          }
        ]
      }
    ]
  },
  {
    "domain_name": "matrix.org",
    "specifications": [
      {
        "host_name": "@",
        "specifications": [
          {
            "record_type": "AAAA",
            "ttl": 600
          }
        ]
      }
    ]
  }
]
```
