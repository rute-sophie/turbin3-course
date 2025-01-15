use solana_idlgen::idlgen; 

idlgen!({
    "version": "0.1.0",
    "name": "turbin3_prereq",
    "metadata" : {
        "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
    },
    "instructions": [
      {
        "name": "complete",
        "discriminator": [
            0,
            77,
            224,
            147,
            136,
            25,
            88,
            76
        ],
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "prereq",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "github",
            "type": "bytes"
          }
        ]
      },
      {
        "name": "update",
        "discriminator": [
            219,
            200,
            88,
            176,
            158,
            63,
            253,
            127
        ],
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "prereq",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "github",
            "type": "bytes"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "PrereqAccount",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "github",
              "type": "bytes"
            },
            {
              "name": "key",
              "type": "publicKey"
            }
          ]
        }
      }
    ],
    "errors": [
      {
        "code": 6000,
        "name": "InvalidGithubAccount",
        "msg": "Invalid Github account"
      }
    ]
  });