### Example search output with attachments

By running the below command (via ssh in this case), here we look for pdfs specifically (but could be any type):

```bash
ssh icarus notmuch search --limit=1 --format=json 'attachment:pdf' | jq .
```

You'll see this output:

```json
[
  {
    "thread": "00000000000306d5",
    "timestamp": 1748731534,
    "date_relative": "Today 00:45",
    "matched": 2,
    "total": 2,
    "authors": "Your Personal AI, user@example.com",
    "subject": "Din faktura från Personal AI",
    "query": [
      "id:user@example.com id:user@example.com",
      null
    ],
    "tags": [
      "Forwarded",
      "Invoice",
      "attachment",
      "inbox",
      "unread"
    ]
  }
]
```

Note that the exact output may differ as new email comes in.

### Viewing the thread
To actually view that thread, you would run this command (again, in this case via ssh):

```bash
[
  [
    [
      {
        "id": "user@example.com",
        "match": true,
        "excluded": false,
        "filename": [
          "/home/user/Mail/archive/All Mail/cur/1748730635.395033_1.icarus,U=183117:2,R"
        ],
        "timestamp": 1748730483,
        "date_relative": "Today 00:28",
        "tags": [
          "Forwarded",
          "Invoice",
          "attachment",
          "inbox",
          "unread"
        ],
        "duplicate": 1,
        "body": [
          {
            "id": 1,
            "content-type": "multipart/mixed",
            "content": [
              {
                "id": 2,
                "content-type": "text/html",
                "content": "<html><head><meta content=\"text/html; charset=utf-8\" http-equiv=\"Content-Type\" /><meta charset=\"utf-8\" /><meta content=\"width=device-width,initial-scale=1\" name=\"viewport\" /><meta name=\"x-apple-disable-message-reformatting\" /><style type=\"text/css\">table td {\n  mso-line-height-rule: exactly;\n}\ntable,\ntd {\n  font-family: Helvetica Neue, Helvetica, Arial;\n}\n.main-table {\n  margin: auto;\n  max-width: 600px;\n  min-width: 600px;\n}\n\n@media screen and (max-width: 776px) {\n  .main-table {\n    max-width: 360px;\n    min-width: 360px;\n  }\n}\na img {\n  border: none;\n}\na[x-apple-data-detectors] {\n  color: inherit !important;\n  text-decoration: none !important;\n}\na,\na:visited,\na:hover,\na:active {\n  color: inherit;\n}</style></head><body style=\"margin: 0; padding: 0; word-spacing: normal; background-color: #f3f4f6; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%;\"><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; border: 0; margin: 0; padding: 0\"><tr><td><table cellpadding=\"0\" cellspacing=\"0\" class=\"main-table\" style=\"padding: 64px 0\"><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; background-color: #fff; border: 1px solid #d9dee7; border-radius: 12px; padding: 32px;\"><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"margin: auto; padding-bottom: 24px\"><tr><td style=\"color: #66758f; font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: center;\">Faktura från Personal AI</td></tr><tr><td style=\"color: #19212e; font-size: 32px; font-weight: 700; line-height: 40px; letter-spacing: 0em; text-align: center;\">14,99 $</td></tr><tr><td style=\"color: #66758f; font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: center;\">utfärdad den 01 juni 2025</td></tr></table><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; padding: 24px 0; border-top: 1px solid #d9dee7; border-bottom: 1px solid #d9dee7;\"><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%\"><tr></tr><tr><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: left; padding-right: 16px; color: #66758f; white-space: nowrap; padding-bottom: 4px;\">Fakturanummer</td><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: right; color: #19212e; white-space: nowrap; padding-bottom: 4px;\">MSTRL-API-662120-004</td></tr><tr><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: left; padding-right: 16px; color: #66758f; white-space: nowrap;\">Fakturadatum</td><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: right; color: #19212e; white-space: nowrap;\">01 juni 2025</td></tr></table></td></tr></table><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; padding: 24px 0; border-bottom: 1px solid #d9dee7;\"><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"margin: auto\"><tr><td style=\"padding-right: 8px;\"><svg fill=\"#006CFA\" height=\"16px\" style=\"padding-top: 4px\" viewBox=\"0 0 16 16\" width=\"16px\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M12.705 10.27C12.5176 10.0838 12.2642 9.97921 12 9.97921C11.7358 9.97921 11.4824 10.0838 11.295 10.27L8.99999 12.75V1C8.99999 0.734784 8.89463 0.48043 8.7071 0.292893C8.51956 0.105357 8.26521 0 7.99999 0C7.73477 0 7.48042 0.105357 7.29288 0.292893C7.10535 0.48043 6.99999 0.734784 6.99999 1V12.755L4.70499 10.255C4.51763 10.0688 4.26417 9.96421 3.99999 9.96421C3.7358 9.96421 3.48235 10.0688 3.29499 10.255C3.19898 10.3482 3.12265 10.4597 3.07053 10.583C3.0184 10.7062 2.99155 10.8387 2.99155 10.9725C2.99155 11.1063 3.0184 11.2388 3.07053 11.362C3.12265 11.4853 3.19898 11.5968 3.29499 11.69L6.93999 15.54C7.22124 15.8209 7.60249 15.9787 7.99999 15.9787C8.39749 15.9787 8.77874 15.8209 9.05999 15.54L12.705 11.69C12.7987 11.597 12.8731 11.4864 12.9239 11.3646C12.9746 11.2427 13.0008 11.112 13.0008 10.98C13.0008 10.848 12.9746 10.7173 12.9239 10.5954C12.8731 10.4736 12.7987 10.363 12.705 10.27Z\"></path></svg></td><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; color: #19212e;\"><a href=\"https://dm7jjg24.r.eu-west-1.awstrack.me/L0/https:%2F%2Fapi.eu.getlago.com%2Frails%2Factive_storage%2Fblobs%2Fredirect%2FeyJfcmFpbHMiOnsiZGF0YSI6Ijc0NmM0OTdhLTc3MWUtNDE2Ni05NzNkLWNjNTRlYTA2MTk1MiIsInB1ciI6ImJsb2JfaWQifX0=--a7fc2f14422fa3301165a23aa0a423ec19dc3e5b%2FMSTRL-API-662120-004.pdf/1/0102019728759228-ce2f31cb-b971-417c-a2ed-3d14bbc9ba8f-000000/1oPaLDTd_oWdPqRJ41MfToT2fp8=428\" style=\"text-decoration: none\">Ladda ner fakturan för ytterligare information</a></td></tr></table></td></tr></table></td></tr></table></td></tr></table></td></tr></table><img alt=\"\" src=\"https://dm7jjg24.r.eu-west-1.awstrack.me/I0/0102019728759228-ce2f31cb-b971-417c-a2ed-3d14bbc9ba8f-000000/ivaKBgf0S6KTWmP9QoC8ZqMfb8I=428\" style=\"display: none; width: 1px; height: 1px;\">\n</body></html>"
              },
              {
                "id": 3,
                "content-type": "application/pdf",
                "content-disposition": "attachment",
                "filename": "invoice-PRSNL-API-1234.pdf",
                "content-transfer-encoding": "base64",
                "content-length": 91820
              }
            ]
          }
        ],
        "crypto": {},
        "headers": {
          "Subject": "Din faktura från Personal AI",
          "From": "\"Personal AI\" <user@example.com>",
          "To": "user@example.com",
          "Reply-To": "Personal AI <user@example.com>",
          "Date": "Sat, 31 May 2025 22:28:03 +0000"
        }
      },
      [
        [
          {
            "id": "user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1748731835.396699_1.icarus,U=183119:2,S"
            ],
            "timestamp": 1748731534,
            "date_relative": "Today 00:45",
            "tags": [
              "Invoice",
              "attachment",
              "inbox",
              "unread"
            ],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "multipart/mixed",
                "content": [
                  {
                    "id": 2,
                    "content-type": "text/html",
                    "content": "<html><head><meta content=\"text/html; charset=UTF-8\" http-equiv=\"Content-Type\"/><meta charset=\"UTF-8\"/><meta content=\"width=device-width,initial-scale=1\" name=\"viewport\"/><meta name=\"x-apple-disable-message-reformatting\"/><style type=\"text/css\">table td {\n  mso-line-height-rule: exactly;\n}\ntable,\ntd {\n  font-family: Helvetica Neue, Helvetica, Arial;\n}\n.main-table {\n  margin: auto;\n  max-width: 600px;\n  min-width: 600px;\n}\n\n@media screen and (max-width: 776px) {\n  .main-table {\n    max-width: 360px;\n    min-width: 360px;\n  }\n}\na img {\n  border: none;\n}\na[x-apple-data-detectors] {\n  color: inherit !important;\n  text-decoration: none !important;\n}\na,\na:visited,\na:hover,\na:active {\n  color: inherit;\n}</style></head><body style=\"margin: 0; padding: 0; word-spacing: normal; background-color: #f3f4f6; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%;\"><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; border: 0; margin: 0; padding: 0\"><tbody><tr><td><table cellpadding=\"0\" cellspacing=\"0\" class=\"main-table\" style=\"padding: 64px 0\"><tbody><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; background-color: #fff; border: 1px solid #d9dee7; border-radius: 12px; padding: 32px;\"><tbody><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"margin: auto; padding-bottom: 24px\"><tbody><tr><td style=\"color: #66758f; font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: center;\">Faktura från Personal AI</td></tr><tr><td style=\"color: #19212e; font-size: 32px; font-weight: 700; line-height: 40px; letter-spacing: 0em; text-align: center;\">14,99 $</td></tr><tr><td style=\"color: #66758f; font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: center;\">utfärdad den 01 juni 2025</td></tr></tbody></table><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; padding: 24px 0; border-top: 1px solid #d9dee7; border-bottom: 1px solid #d9dee7;\"><tbody><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%\"><tbody><tr></tr><tr><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: left; padding-right: 16px; color: #66758f; white-space: nowrap; padding-bottom: 4px;\">Fakturanummer</td><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: right; color: #19212e; white-space: nowrap; padding-bottom: 4px;\">MSTRL-API-662120-004</td></tr><tr><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: left; padding-right: 16px; color: #66758f; white-space: nowrap;\">Fakturadatum</td><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: right; color: #19212e; white-space: nowrap;\">01 juni 2025</td></tr></tbody></table></td></tr></tbody></table><table cellpadding=\"0\" cellspacing=\"0\" style=\"width: 100%; padding: 24px 0; border-bottom: 1px solid #d9dee7;\"><tbody><tr><td><table cellpadding=\"0\" cellspacing=\"0\" style=\"margin: auto\"><tbody><tr><td style=\"padding-right: 8px;\"><svg fill=\"#006CFA\" height=\"16px\" style=\"padding-top: 4px\" viewBox=\"0 0 16 16\" width=\"16px\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M12.705 10.27C12.5176 10.0838 12.2642 9.97921 12 9.97921C11.7358 9.97921 11.4824 10.0838 11.295 10.27L8.99999 12.75V1C8.99999 0.734784 8.89463 0.48043 8.7071 0.292893C8.51956 0.105357 8.26521 0 7.99999 0C7.73477 0 7.48042 0.105357 7.29288 0.292893C7.10535 0.48043 6.99999 0.734784 6.99999 1V12.755L4.70499 10.255C4.51763 10.0688 4.26417 9.96421 3.99999 9.96421C3.7358 9.96421 3.48235 10.0688 3.29499 10.255C3.19898 10.3482 3.12265 10.4597 3.07053 10.583C3.0184 10.7062 2.99155 10.8387 2.99155 10.9725C2.99155 11.1063 3.0184 11.2388 3.07053 11.362C3.12265 11.4853 3.19898 11.5968 3.29499 11.69L6.93999 15.54C7.22124 15.8209 7.60249 15.9787 7.99999 15.9787C8.39749 15.9787 8.77874 15.8209 9.05999 15.54L12.705 11.69C12.7987 11.597 12.8731 11.4864 12.9239 11.3646C12.9746 11.2427 13.0008 11.112 13.0008 10.98C13.0008 10.848 12.9746 10.7173 12.9239 10.5954C12.8731 10.4736 12.7987 10.363 12.705 10.27Z\"></path></svg></td><td style=\"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; color: #19212e;\"><a href=\"https://dm7jjg24.r.eu-west-1.awstrack.me/L0/https:%2F%2Fapi.eu.getlago.com%2Frails%2Factive_storage%2Fblobs%2Fredirect%2FeyJfcmFpbHMiOnsiZGF0YSI6Ijc0NmM0OTdhLTc3MWUtNDE2Ni05NzNkLWNjNTRlYTA2MTk1MiIsInB1ciI6ImJsb2JfaWQifX0=--a7fc2f14422fa3301165a23aa0a423ec19dc3e5b%2FMSTRL-API-662120-004.pdf/1/0102019728759228-ce2f31cb-b971-417c-a2ed-3d14bbc9ba8f-000000/1oPaLDTd_oWdPqRJ41MfToT2fp8=428\" style=\"text-decoration: none\">Ladda ner fakturan för ytterligare information</a></td></tr></tbody></table></td></tr></tbody></table></td></tr></tbody></table></td></tr></tbody></table></td></tr></tbody></table><img alt=\"\" src=\"https://dm7jjg24.r.eu-west-1.awstrack.me/I0/0102019728759228-ce2f31cb-b971-417c-a2ed-3d14bbc9ba8f-000000/ivaKBgf0S6KTWmP9QoC8ZqMfb8I=428\" style=\"display: none; width: 1px; height: 1px;\"/>\n</body></html>"
                  },
                  {
                    "id": 3,
                    "content-type": "application/pdf",
                    "content-disposition": "attachment",
                    "filename": "invoice-PRSNL-API-1234.pdf",
                    "content-transfer-encoding": "base64",
                    "content-length": 91820
                  }
                ]
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Din faktura från Personal AI",
              "From": "<user@example.com>",
              "To": "user@example.com",
              "Reply-To": "user@example.com",
              "Date": "Sat, 31 May 2025 22:45:34 +0000"
            }
          },
          []
        ]
      ]
    ]
  ]
]
```

In the above example, the invoice was also forwarded which is why there's an additional message in the thread. To actually get the attachment itself, you can use something like:

```bash
ssh icarus notmuch show --format=raw 'id:user@example.com'
```

Which would return this:

```
Content-Type: multipart/mixed;
 boundary=e285fb777919c2b26e5d1f717a6b8cadb49cf09a4b7a3ce97e9c366d9be66f4b
References: <1aLY3ZOIc8fOgCcz1DUFqdNvNDV5W655HjTIf0hASCOTAIK7p-l1u_1OsTl05qjwL8QkKPWlzJtvVgxvDXYnAA==@something>
X-Pm-Date: Sat, 31 May 2025 22:28:03 +0000
X-Pm-External-Id: <user@example.com>
X-Pm-Internal-Id: 1aLY3ZOIc8fOgCcz1DUFqdNvNDV5W655HjTIf0hASCOTAIK7p-l1u_1OsTl05qjwL8QkKPWlzJtvVgxvDXYnAA==
To: <user@example.com>
Reply-To: "Personal AI" <user@example.com>
From: "Personal AI" <user@example.com>
Subject: =?utf-8?q?Din_faktura_fr=C3=A5n_Personal_AI
Return-Path: <user@example.com>
X-Original-To: user@example.com
Delivered-To: user@example.com
Authentication-Results: mail.examplemail.com; dkim=pass (Good 2048 bit    rsa-sha256 signature) header.d=Personal.ai header.a=rsa-sha256;    dkim=pass (Good 1024 bit rsa-sha256 signature) header.d=amazonses.com    header.a=rsa-sha256
Authentication-Results: mail.examplemail.com; dmarc=pass (p=quarantine dis=none) header.from=Personal.ai
Authentication-Results: mail.examplemail.com; spf=pass smtp.mailfrom=eu-west-1.amazonses.com
Authentication-Results: mail.examplemail.com; arc=none smtp.remote-ip=54.240.7.30
Authentication-Results: mail.examplemail.com; dkim=pass (2048-bit key) header.d=Personal.ai header.i=@Personal.ai header.b="K0uPbdSO"; dkim=pass (1024-bit key) header.d=amazonses.com header.i=@amazonses.com header.b="NIzRKx4U"
Date: Sat, 31 May 2025 22:28:03 +0000
Message-Id: <user@example.com>
Mime-Version: 1.0
X-Pm-Origin: external
X-Pm-Transfer-Encryption: TLSv1.3 with cipher TLS_AES_256_GCM_SHA384 (256/256 bits)
X-Pm-Content-Encryption: on-delivery
X-Pm-Spamscore: 0
X-Pm-Spam-Action: inbox
X-Pm-Forwarded-To: user@example.com
X-TUID: u5WfUM+qL9ni
X-Aim-Invoice-Amount: 14.99
X-Aim-Invoice-Currency: USD
X-Aim-Invoice-Number: MSTRL-API-662120-004
X-Aim-Invoice-Date: 2025-06-01
X-Aim-Invoice-Due: 2025-06-01

--e285fb777919c2b26e5d1f717a6b8cadb49cf09a4b7a3ce97e9c366d9be66f4b
Content-Transfer-Encoding: quoted-printable
Content-Type: text/html; charset=utf-8

<html><head><meta content=3D"text/html; charset=3Dutf-8" http-equiv=3D"Cont=
ent-Type" /><meta charset=3D"utf-8" /><meta content=3D"width=3Ddevice-width=
,initial-scale=3D1" name=3D"viewport" /><meta name=3D"x-apple-disable-messa=
ge-reformatting" /><style type=3D"text/css">table td {
  mso-line-height-rule: exactly;
}
table,
td {
  font-family: Helvetica Neue, Helvetica, Arial;
}
.main-table {
  margin: auto;
  max-width: 600px;
  min-width: 600px;
}

@media screen and (max-width: 776px) {
  .main-table {
    max-width: 360px;
    min-width: 360px;
  }
}
a img {
  border: none;
}
a[x-apple-data-detectors] {
  color: inherit !important;
  text-decoration: none !important;
}
a,
a:visited,
a:hover,
a:active {
  color: inherit;
}</style></head><body style=3D"margin: 0; padding: 0; word-spacing: normal;=
 background-color: #f3f4f6; -webkit-text-size-adjust: 100%; -ms-text-size-a=
djust: 100%;"><table cellpadding=3D"0" cellspacing=3D"0" style=3D"width: 10=
0%; border: 0; margin: 0; padding: 0"><tr><td><table cellpadding=3D"0" cell=
spacing=3D"0" class=3D"main-table" style=3D"padding: 64px 0"><tr><td><table=
 cellpadding=3D"0" cellspacing=3D"0" style=3D"width: 100%; background-color=
: #fff; border: 1px solid #d9dee7; border-radius: 12px; padding: 32px;"><tr=
><td><table cellpadding=3D"0" cellspacing=3D"0" style=3D"margin: auto; padd=
ing-bottom: 24px"><tr><td style=3D"color: #66758f; font-size: 14px; font-we=
ight: 400; line-height: 20px; letter-spacing: 0em; text-align: center;">Fak=
tura fr=C3=A5n Personal AI</td></tr><tr><td style=3D"color: #19212e; font-si=
ze: 32px; font-weight: 700; line-height: 40px; letter-spacing: 0em; text-al=
ign: center;">14,99 $</td></tr><tr><td style=3D"color: #66758f; font-size: =
14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align:=
 center;">utf=C3=A4rdad den 01 juni 2025</td></tr></table><table cellpaddin=
g=3D"0" cellspacing=3D"0" style=3D"width: 100%; padding: 24px 0; border-top=
: 1px solid #d9dee7; border-bottom: 1px solid #d9dee7;"><tr><td><table cell=
padding=3D"0" cellspacing=3D"0" style=3D"width: 100%"><tr></tr><tr><td styl=
e=3D"font-size: 14px; font-weight: 400; line-height: 20px; letter-spacing: =
0em; text-align: left; padding-right: 16px; color: #66758f; white-space: no=
wrap; padding-bottom: 4px;">Fakturanummer</td><td style=3D"font-size: 14px;=
 font-weight: 400; line-height: 20px; letter-spacing: 0em; text-align: righ=
t; color: #19212e; white-space: nowrap; padding-bottom: 4px;">MSTRL-API-662=
120-004</td></tr><tr><td style=3D"font-size: 14px; font-weight: 400; line-h=
eight: 20px; letter-spacing: 0em; text-align: left; padding-right: 16px; co=
lor: #66758f; white-space: nowrap;">Fakturadatum</td><td style=3D"font-size=
: 14px; font-weight: 400; line-height: 20px; letter-spacing: 0em; text-alig=
n: right; color: #19212e; white-space: nowrap;">01 juni 2025</td></tr></tab=
le></td></tr></table><table cellpadding=3D"0" cellspacing=3D"0" style=3D"wi=
dth: 100%; padding: 24px 0; border-bottom: 1px solid #d9dee7;"><tr><td><tab=
le cellpadding=3D"0" cellspacing=3D"0" style=3D"margin: auto"><tr><td style=
=3D"padding-right: 8px;"><svg fill=3D"#006CFA" height=3D"16px" style=3D"pad=
ding-top: 4px" viewBox=3D"0 0 16 16" width=3D"16px" xmlns=3D"http://www.w3.=
org/2000/svg"><path d=3D"M12.705 10.27C12.5176 10.0838 12.2642 9.97921 12 9=
.97921C11.7358 9.97921 11.4824 10.0838 11.295 10.27L8.99999 12.75V1C8.99999=
 0.734784 8.89463 0.48043 8.7071 0.292893C8.51956 0.105357 8.26521 0 7.9999=
9 0C7.73477 0 7.48042 0.105357 7.29288 0.292893C7.10535 0.48043 6.99999 0.7=
34784 6.99999 1V12.755L4.70499 10.255C4.51763 10.0688 4.26417 9.96421 3.999=
99 9.96421C3.7358 9.96421 3.48235 10.0688 3.29499 10.255C3.19898 10.3482 3.=
12265 10.4597 3.07053 10.583C3.0184 10.7062 2.99155 10.8387 2.99155 10.9725=
C2.99155 11.1063 3.0184 11.2388 3.07053 11.362C3.12265 11.4853 3.19898 11.5=
968 3.29499 11.69L6.93999 15.54C7.22124 15.8209 7.60249 15.9787 7.99999 15.=
9787C8.39749 15.9787 8.77874 15.8209 9.05999 15.54L12.705 11.69C12.7987 11.=
597 12.8731 11.4864 12.9239 11.3646C12.9746 11.2427 13.0008 11.112 13.0008 =
10.98C13.0008 10.848 12.9746 10.7173 12.9239 10.5954C12.8731 10.4736 12.798=
7 10.363 12.705 10.27Z"></path></svg></td><td style=3D"font-size: 14px; fon=
t-weight: 400; line-height: 20px; letter-spacing: 0em; color: #19212e;"><a =
href=3D"https://dm7jjg24.r.eu-west-1.awstrack.me/L0/https:%2F%2Fapi.eu.getl=
ago.com%2Frails%2Factive_storage%2Fblobs%2Fredirect%2FeyJfcmFpbHMiOnsiZGF0Y=
SI6Ijc0NmM0OTdhLTc3MWUtNDE2Ni05NzNkLWNjNTRlYTA2MTk1MiIsInB1ciI6ImJsb2JfaWQi=
fX0=3D--a7fc2f14422fa3301165a23aa0a423ec19dc3e5b%2FMSTRL-API-662120-004.pdf=
/1/0102019728759228-ce2f31cb-b971-417c-a2ed-3d14bbc9ba8f-000000/1oPaLDTd_oW=
dPqRJ41MfToT2fp8=3D428" style=3D"text-decoration: none">Ladda ner fakturan =
f=C3=B6r ytterligare information</a></td></tr></table></td></tr></table></t=
d></tr></table></td></tr></table></td></tr></table><img alt=3D"" src=3D"htt=
ps://dm7jjg24.r.eu-west-1.awstrack.me/I0/0102019728759228-ce2f31cb-b971-417=
c-a2ed-3d14bbc9ba8f-000000/ivaKBgf0S6KTWmP9QoC8ZqMfb8I=3D428" style=3D"disp=
lay: none; width: 1px; height: 1px;">
</body></html>
--e285fb777919c2b26e5d1f717a6b8cadb49cf09a4b7a3ce97e9c366d9be66f4b
Content-Transfer-Encoding: base64
Content-Disposition: attachment; filename=invoice-PRSNL-API-12345.pdf
Content-Type: application/pdf; filename=invoice-PRSNL-API-12345.pdf;
 name=invoice-PRSNL-API-1234.pdf
x-pm-content-encryption: on-delivery

SW4gdGhlIGRpbS1saXQgYWxsZXksIHNoYWRvd3MgZGFuY2UgYW
5kIHN3YXksCkEgbG9uZSBkZXRlY3RpdmUgd2l0aCBzZWNyZXRz
IG9uIGhlciBtaW5kLApTaGUgY3JvdWNoZXMgYnkgdGhlIG1haW
xib3ggYXQgdGhlIGJyZWFrIG9mIGRheSwKQ29udmluY2VkIHRo
ZSB3b3JsZOKAmXMgaGlkZGVuIHRydXRocyB0aGVyZSBzaGUgd2
lsbCBmaW5kLgoKSGVyIHRyZW5jaCBjb2F0IGZsdXR0ZXJzLCBj
b2xsYXIgdHVybmVkIHRvIHRoZSBza3ksCkEgZmVkb3JhIHRpbH
RlZCBsb3cgdXBvbiBoZXIgYnJvdywKU2hlIHdoaXNwZXJzIHRv
IHRoZSBuaWdodCwgYSBzaWxlbnQgYmF0dGxlIGNyeToK4oCcU2
9vbiBJ4oCZbGwgZXhwb3NlIHdoYXQgZXZlcnlvbmXigJlzIHdo
aXNwZXJpbmcgYWJvdXQgbm93IeKAnQoKU2hlIHNsaXBzIGludG
8gYSBjYWbDqSwgc2lwcGluZyBiaXR0ZXIgcm9hc3QgYnJldywK
RWF2ZXNkcm9wcGluZyBvbiBzdHJhbmdlcnMgd2hvIHNoYXJlIG
h1c2hlZCBjb25zcGlyYWNpZXMsCkhlciBleWVzIGZsaWNrZXIg
ZmlybWx5IG9uIHRoYXQgb25lIGNyeXB0aWMgY2x1ZToKQSBuYX
BraW4gc2NyYXdsZWQgd2l0aCBudW1iZXJz4oCUbXlzdGljIG5v
ZXNlcy4KCkEgY29kZT8gQSBjaXBoZXI/IFNoZSBmZWVscyBoZX
IgcHVsc2UgcXVpY2tlbiBmYXN0LApQZW4gaW4gaGFuZCwgc2hl
IGRlY2lwaGVycyBlYWNoIHB1enpsaW5nIGxpbmUsCkVhY2ggc3
ltYm9sIHRlYXNlcyBwcm9taXNlcyBvZiBzZWNyZXRzIHZhc3Qs
ClNoZeKAmXMgY2VydGFpbiBzY2FuZGFsIGFuZCBzY2FuZGFsb3
VzIGdvc3NpcCBjb21iaW5lLgoKU2hlIHRhaWxzIGEgc3VpdGVk
IGZlbGxvdyB3aXRoIGEgY3VyaW91cyBnYWl0OwpIZSBzbGlwcy
BhIFVTQiB1bmRlciBhIG5lb24tbGl0IGJlbmNoLApIZXIgaGVh
cnQgcmFjZXMgZmFzdGVy4oCUaGVyIHByZXkgd29u4oCZdCBlc2
NhcGUgZmF0ZSwKU2hlIGluY2hlcyBjbG9zZXIgdG8gdGhlIHRy
dXRoLCBjaXR54oCZcyB1bmRlcmNvdmVyIHRyZW5jaC4KCkF0IG
1pZG5pZ2h04oCZcyBodXNoLCBzaGUgcHJpZXMgdGhhdCBVU0Ig
bG9vc2UsCkEgY2xpY2vigJRhIGZsYXNo4oCUYSBmb2xkZXIgdG
l0bGVkIOKAnFZlcnkgUHJpdmF0ZSBTdHVmZizigJ0KSGVyIGds
b3ZlZCBmaW5nZXJzIHRyZW1ibGU7IHNoZSBmZWVscyBvbiB0aG
UgbG9vc2UsClNoZeKAmXMgY2VydGFpbiB0aGVzZSBmaWxlcyBj
b250YWluIHNlY3JldHMgZW5vdWdoLgoKV2l0aCBsYXNlciBmb2
N1cywgc2hlIG9wZW5zIGVhY2ggZm9sZGVyIGluIHR1cm4sCkV4
cGVjdGluZyBmaW5hbmNpYWwgZnJhdWQsIGhpZGRlbiBsb3ZlIG
xldHRlcnMsIGFuZCBtb3Jl4oCUCkluc3RlYWQsIHNoZSBmaW5k
cyBwaG90b3Mgb2Yga2l0dGVucyBhc2xlZXAsIGhhaXIgdGhhdC
B3b27igJl0IGJ1cm4sClJlY2lwZXMgZm9yIGdyYW5kbWHigJlz
IHNvdXAgYW5kIGEgY2F0IHdlYXJpbmcgYSBib3cgdGllLCBnYW
xvcmUuCgpIZXIgYnJvdyBmdXJyb3dzLiBUaGVzZSDigJxzZWNy
ZXRz4oCdIGJyaW5nIG5vIHNjYW5kYWxvdXMgc3BvaWwsCkluc3
RlYWQsIHNpbGx5IHNuYXBzaG90cywgYW5kIGhhcm1sZXNzIHRy
aW5rZXRzIGdhbG9yZSwKV2FzIGFsbCBoZXIgbWlkbmlnaHQgc2
xldXRoaW5nIGp1c3Qgaml0dGVyeSB0b2lsPwpObyBkYXJpbmcg
cmV2ZWxhdGlvbnMsIG5vIHNob2NraW5nIHVwcm9hci4KClNoZS
BzZW5kcyBhIHNtb2tlIHNpZ25hbOKAlGhlciBwYXJ0bmVyIGFw
cGVhcnMgaW4gYSBmbGFzaCwK4oCcRGlkIHlvdSBmaW5kIHdoYX
QgeW91IHNvdWdodD/igJ0gaGUgYXNrcywgY2lnYXLigJlzIGVt
YmVycyBicmlnaHQuClNoZSBzaHJ1Z3Mgd2l0aCBhIGdyaW4gYW
5kIHRyaWVzIG5vdCB0byBsZXQgZHJlYW0gZGFzaDoK4oCcTm90
aGluZyBidXQgZmx1ZmbigJRsaWtlIGtpdHRlbnMgd2hvIGxvaX
RlciBhdCBuaWdodCHigJ0KClRoZXkgbG9jayBhcm1zIGluIGxh
dWdodGVyLCBkZXRlY3RpdmUgYW5kIGZyaWVuZCwKVW5kZXIgYS
BsYW1wcG9zdOKAmXMgZ2xvdywgbWlzbGVkIGJ5IGEgY2xhbmRl
c3RpbmUgbHVyZSwKVGhvdWdoIHNlY3JldHMgdW5lYXJ0aGVkIH
dlcmVu4oCZdCBxdWl0ZSBpbnRlbCB0byBzZW5kLApBdCBsZWFz
dCB0aGV5IGZvdW5kIGpveSBpbiBhIGNhcGVyIGFic3VyZCwgZm
9yIHN1cmUuCgpTbyBpZiBldmVyIHlvdSBjcmF2ZSBjbGFuZGVz
dGluZSB3aGlzcGVycyB1bnRvbGQsCkJld2FyZSBvZiByZWQgaG
VycmluZ3MgbHVya2luZyBpbiBzaGFkb3d5IHN0cmVldHMsCkZv
ciBzb21ldGltZXMgeW914oCZbGwgZmluZCwgaW5zdGVhZCBvZi
BzZWNyZXRzIGFuZCBnb2xkLApKdXN0IGtpdHRlbnMgYW5kIGNv
b2tpZXPigJRsaWZl4oCZcyBzd2VldGVzdCBkZWZlYXRzLg==
--e285fb777919c2b26e5d1f717a6b8cadb49cf09a4b7a3ce97e9c366d9be66f4b--

```

To actually extract the attachment you would need to use either a rust library or something on the cli like ripmime I suppose. Please make sure to keep the filename if making the attachment downloadable in a web ui or other application.
