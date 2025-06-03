### Example thread output

By running the below command (via ssh in this case):

```bash
ssh icarus notmuch show --format=json --include-html --entire-thread 'thread:00000000000305e4' | jq .
```

You'll see this output:

```json
[
  [
    [
      {
        "id": "user@example.com",
        "match": true,
        "excluded": false,
        "filename": [
          "/home/user/Mail/archive/All Mail/cur/1748424311.690968_1.icarus,U=182875:2,RS"
        ],
        "timestamp": 1748424035,
        "date_relative": "Wed. 11:20",
        "tags": [
          "Important",
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
                "content": "<html xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\" xmlns:w=\"urn:schemas-microsoft-com:office:word\" xmlns:m=\"http://schemas.microsoft.com/office/2004/12/omml\" xmlns=\"http://www.w3.org/TR/REC-html40\">\n<head>\n<meta http-equiv=\"Content-Type\" content=\"text/html; charset=iso-8859-1\">\n<meta name=\"Generator\" content=\"Microsoft Word 15 (filtered medium)\">\n<!--[if !mso]><style>v\\:* {behavior:url(#default#VML);}\no\\:* {behavior:url(#default#VML);}\nw\\:* {behavior:url(#default#VML);}\n.shape {behavior:url(#default#VML);}\n</style><![endif]--><style><!--\n/* Font Definitions */\n@font-face\n\t{font-family:\"Cambria Math\";\n\tpanose-1:2 4 5 3 5 4 6 3 2 4;}\n@font-face\n\t{font-family:Aptos;}\n/* Style Definitions */\np.MsoNormal, li.MsoNormal, div.MsoNormal\n\t{margin:0cm;\n\tfont-size:12.0pt;\n\tfont-family:\"Aptos\",sans-serif;}\na:link, span.MsoHyperlink\n\t{mso-style-priority:99;\n\tcolor:blue;\n\ttext-decoration:underline;}\nspan.EmailStyle19\n\t{mso-style-type:personal-compose;}\n.MsoChpDefault\n\t{mso-style-type:export-only;\n\tfont-size:10.0pt;\n\tmso-ligatures:none;}\n@page WordSection1\n\t{size:612.0pt 792.0pt;\n\tmargin:70.85pt 70.85pt 70.85pt 70.85pt;}\ndiv.WordSection1\n\t{page:WordSection1;}\n--></style><!--[if gte mso 9]><xml>\n<o:shapedefaults v:ext=\"edit\" spidmax=\"1026\" />\n</xml><![endif]--><!--[if gte mso 9]><xml>\n<o:shapelayout v:ext=\"edit\">\n<o:idmap v:ext=\"edit\" data=\"1\" />\n</o:shapelayout></xml><![endif]-->\n</head>\n<body lang=\"SV\" link=\"blue\" vlink=\"purple\" style=\"word-wrap:break-word\">\n<div class=\"WordSection1\">\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\">Hej Alice,<o:p></o:p></p>\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\">This is a confirmation of our digital quarterly review meeting. We will meet via video conference\n<b>Wednesday June 4th</b> kl. <b>10.00</b>. Please accept the meeting invitation. The meeting link is below:\n<o:p></o:p></p>\n<p><a href=\"https://financecompany.zoom.us/j/55512345678\">https://financecompany.zoom.us/j/55512345678</a>\n<o:p></o:p></p>\n<p>Meeting ID: 555 123 4567 <o:p></o:p></p>\n<p>If you would like us to review your portfolio holdings as well, please reply with the current allocation details.\n<o:p></o:p></p>\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\">Med vänliga hälsningar,<o:p></o:p></p>\n<p class=\"MsoNormal\"><b><span style=\"mso-ligatures:standardcontextual\">Bob Wilson<o:p></o:p></span></b></p>\n<p class=\"MsoNormal\"><span style=\"mso-ligatures:standardcontextual\">Corporate Account Manager at Finance Company AB<o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"mso-ligatures:standardcontextual\">Tel: 555-0123<o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"mso-ligatures:standardcontextual\">Example Street 123, Box 456, 111 22 STOCKHOLM<o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"mso-ligatures:standardcontextual\"><a href=\"http://financecompany.se/\"><span style=\"color:#1B9830;mso-fareast-language:EN-GB\">financecompany.se</span></a></span><span style=\"mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"mso-ligatures:standardcontextual\"><a href=\"https://play.google.com/store/apps/details?id=se.finance companybank.androidapplikation\"><span style=\"color:#16A53F;mso-fareast-language:EN-GB\">Android-app</span></a></span><span style=\"mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"mso-ligatures:standardcontextual\"><a href=\"https://itunes.apple.com/se/app/finance company/id381311572?mt=8\"><span lang=\"EN-US\" style=\"color:#16A53F;mso-fareast-language:EN-GB\">iOS-app</span></a></span><span lang=\"EN-US\" style=\"mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"font-size:13.5pt;color:black;mso-fareast-language:EN-GB\"><img border=\"0\" width=\"200\" height=\"142\" style=\"width:2.0833in;height:1.4791in\" id=\"Picture_x0020_1\" src=\"cid:image001.png@01DBCFC2.879537F0\" alt=\"signature_260642366\"></span><span style=\"font-size:11.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:gray;mso-ligatures:standardcontextual\">Detta mejl kan innehålla konfidentiell information. Om du har fått det av misstag ber vi dig därför att inte</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:gray;mso-ligatures:standardcontextual\">kopiera eller vidarebefordra informationen annat än för att meddela avsändaren.\n</span><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">Radera sedan meddelandet.\n</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">Tänk också på att mejl kan omdirigeras, förvrängas och/eller innehålla till exempel virus; Försäkringsbolaget Finance Company AB tar\n</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">inget ansvar för någon sådan ev. åverkan. Försäkringsbolaget Finance Company AB. Styrelsens säte: Stockholm. Registration no. 123456-7890.</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">: Stockholm. Registration no. 123456-7890.</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><o:p>&nbsp;</o:p></p>\n<p class=\"MsoNormal\"><o:p>&nbsp;</o:p></p>\n</div>\n</body>\n</html>\n"
              },
              {
                "id": 3,
                "content-type": "text/calendar",
                "content-disposition": "attachment",
                "filename": "calendar.ics",
                "content": "BEGIN:VCALENDAR\nMETHOD:REQUEST\nPRODID:Microsoft Exchange Server 2010\nVERSION:2.0\nBEGIN:VTIMEZONE\nTZID:W. Europe Standard Time\nBEGIN:STANDARD\nDTSTART:16010101T030000\nTZOFFSETFROM:+0200\nTZOFFSETTO:+0100\nRRULE:FREQ=YEARLY;INTERVAL=1;BYDAY=-1SU;BYMONTH=10\nEND:STANDARD\nBEGIN:DAYLIGHT\nDTSTART:16010101T020000\nTZOFFSETFROM:+0100\nTZOFFSETTO:+0200\nRRULE:FREQ=YEARLY;INTERVAL=1;BYDAY=-1SU;BYMONTH=3\nEND:DAYLIGHT\nEND:VTIMEZONE\nBEGIN:VEVENT\nORGANIZER;CN=Bob Wilson:MAILTO:user@example.com\nATTENDEE;ROLE=REQ-PARTICIPANT;PARTSTAT=NEEDS-ACTION;RSVP=TRUE;CN=alice@techcorp.\n example:MAILTO:user@example.com\nATTACH:CID:image001.png@01DBCFC2.879537F0\nDESCRIPTION;LANGUAGE=en-US:Hej Alice\\,\\nHär kommer en bekräftelse på vår\n t digitala uppföljningsmöte. We will meet via video conference Wednesday June 4th kl. 1\n 0.00. Please accept the meeting invitation. The meeting link is below:\\\n n\\nhttps://financecompany.zoom.us/j/55512345678\\n\\nMeeting ID: 555 123 4567\\n\\nOm\n  du vill att vi kika på investment certificates också så kan du i vändande mail skic\n ka innehaven och fördelningen av dessa.\\nMed vänliga hälsningar\\,\\nBob\n  Wilson\\nCorporate Account Manager at Finance Company AB\\nTel: 555-0123\\nExempelgatan \n 103\\, Box 1399\\, 111 39 STOCKHOLM\\nfinancecompany.se<http://financecompany.se/>\\nAndroid-a\n pp<https://play.google.com/store/apps/details?id=se.finance companybank.androidappl\n ikation>\\niOS-app<https://itunes.apple.com/se/app/finance company/id381311572?mt=8>\n \\n[signature_260642366]\\nDetta mejl kan innehålla konfidentiell informati\n on. Om du har fått det av misstag ber vi dig därför att inte\\nkopiera e\n ller vidarebefordra informationen annat än för att meddela avsändaren. \n Radera sedan meddelandet.\\nTänk också på att mejl kan omdirigeras\\, fö\n rvrängas och/eller innehålla till exempel virus\\; Försäkringsbolaget A\n Finance Company AB tar\\ninget ansvar för någon sådan ev. åverkan. Försäkr\n ingsbolaget Finance Company AB. Styrelsens säte: Stockholm. Registration no. 516401-67\n 75.\\n: Stockholm. Registration no. 123456-7890.\\n\\n\\n\nUID:040000008200E00074C5B7101A82E008000000002094B747C2CFDB01000000000000000\n 01000000004444F071B796343A5B6C8A75EA8AA99\nSUMMARY;LANGUAGE=en-US:Uppföljningsmöte mellan Alice och Bob\\, Finance Company AB\nDTSTART;TZID=W. Europe Standard Time:20250604T100000\nDTEND;TZID=W. Europe Standard Time:20250604T104500\nCLASS:PUBLIC\nPRIORITY:5\nDTSTAMP:20250528T092034Z\nTRANSP:OPAQUE\nSTATUS:CONFIRMED\nSEQUENCE:0\nLOCATION;LANGUAGE=en-US:Zoom Möte\nX-MICROSOFT-CDO-APPT-SEQUENCE:0\nX-MICROSOFT-CDO-OWNERAPPTID:-1122215959\nX-MICROSOFT-CDO-BUSYSTATUS:TENTATIVE\nX-MICROSOFT-CDO-INTENDEDSTATUS:BUSY\nX-MICROSOFT-CDO-ALLDAYEVENT:FALSE\nX-MICROSOFT-CDO-IMPORTANCE:1\nX-MICROSOFT-CDO-INSTTYPE:0\nX-MICROSOFT-DONOTFORWARDMEETING:FALSE\nX-MICROSOFT-DISALLOW-COUNTER:FALSE\nBEGIN:VALARM\nDESCRIPTION:REMINDER\nTRIGGER;RELATED=START:-PT15M\nACTION:DISPLAY\nEND:VALARM\nEND:VEVENT\nEND:VCALENDAR\n"
              },
              {
                "id": 4,
                "content-type": "image/png",
                "content-disposition": "attachment",
                "content-id": "image001.png@01DBCFC2.879537F0",
                "filename": "image001.png",
                "content-transfer-encoding": "base64",
                "content-length": 61851
              }
            ]
          }
        ],
        "crypto": {},
        "headers": {
          "Subject": "Uppföljningsmöte mellan Alice och Bob, Finance Company AB",
          "From": "\"Bob Wilson\" <user@example.com>",
          "To": "\"user@example.com\" <user@example.com>",
          "Reply-To": "Bob Wilson <user@example.com>",
          "Date": "Wed, 28 May 2025 09:20:35 +0000"
        }
      },
      [
        [
          {
            "id": "1LRPSnmIIWVeX2D53dfikI4CXySItqoD5Y6RXJUIVbpNaA0-5gt6Rcw2xN6tgekvUXh8Ok_pMEN18f2OLdjbrCp1_5ZadEjonsbcbzjK-CM=@techcorp.example",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1748425511.691716_1.icarus,U=182878:2,S"
            ],
            "timestamp": 1748425489,
            "date_relative": "Wed. 11:44",
            "tags": [
              "Important",
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
                    "content": "<p dir=\"ltr\">Hej Bob,</p>\n<p dir=\"ltr\">Tack f&#246;r ett bra m&#246;te. Bifogar lite underlag fr&#229;n pensioner hos Insurance Provider A and B.</p>\n<p dir=\"ltr\">Mvh,<br>\nAlice</p>\n<div class=\"examplemail_quote\"><br><br>-------- Ursprungligt meddelande --------<br>Den 2025-05-28 11:20, Bob Wilson <user@example.com> skrev:<br><blockquote class=\"examplemail_quote\"><html xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\" xmlns:w=\"urn:schemas-microsoft-com:office:word\" xmlns:m=\"http://schemas.microsoft.com/office/2004/12/omml\" xmlns=\"http://www.w3.org/TR/REC-html40\">\n<head>\n<meta http-equiv=\"Content-Type\" content=\"text/html; charset=iso-8859-1\">\n<meta name=\"Generator\" content=\"Microsoft Word 15 (filtered medium)\">\n<!--[if !mso]><style>v\\:* {behavior:url(#default#VML);}\no\\:* {behavior:url(#default#VML);}\nw\\:* {behavior:url(#default#VML);}\n.shape {behavior:url(#default#VML);}\n</style><![endif]--><style><!--\n/* Font Definitions */\n@font-face\n\t{font-family:\"Cambria Math\";\n\tpanose-1:2 4 5 3 5 4 6 3 2 4;}\n@font-face\n\t{font-family:Aptos;}\n/* Style Definitions */\np.MsoNormal, li.MsoNormal, div.MsoNormal\n\t{margin:0cm;\n\tfont-size:12.0pt;\n\tfont-family:\"Aptos\",sans-serif;}\na:link, span.MsoHyperlink\n\t{mso-style-priority:99;\n\tcolor:blue;\n\ttext-decoration:underline;}\nspan.EmailStyle19\n\t{mso-style-type:personal-compose;}\n.MsoChpDefault\n\t{mso-style-type:export-only;\n\tfont-size:10.0pt;\n\tmso-ligatures:none;}\n@page WordSection1\n\t{size:612.0pt 792.0pt;\n\tmargin:70.85pt 70.85pt 70.85pt 70.85pt;}\ndiv.WordSection1\n\t{page:WordSection1;}\n--></style><!--[if gte mso 9]><xml>\n<o:shapedefaults v:ext=\"edit\" spidmax=\"1026\" />\n</xml><![endif]--><!--[if gte mso 9]><xml>\n<o:shapelayout v:ext=\"edit\">\n<o:idmap v:ext=\"edit\" data=\"1\" />\n</o:shapelayout></xml><![endif]-->\n</head>\n<body lang=\"SV\" link=\"blue\" vlink=\"purple\" style=\"word-wrap:break-word\">\n<div class=\"WordSection1\">\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\">Hej Alice,<o:p></o:p></p>\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\">This is a confirmation of our digital quarterly review meeting. We will meet via video conference\n<b>Wednesday June 4th</b> kl. <b>10.00</b>. Please accept the meeting invitation. The meeting link is below:\n<o:p></o:p></p>\n<p><a href=\"https://financecompany.zoom.us/j/55512345678\">https://financecompany.zoom.us/j/55512345678</a>\n<o:p></o:p></p>\n<p>Meeting ID: 555 123 4567 <o:p></o:p></p>\n<p>If you would like us to review your portfolio holdings as well, please reply with the current allocation details.\n<o:p></o:p></p>\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\">Med vänliga hälsningar,<o:p></o:p></p>\n<p class=\"MsoNormal\"><b><span style=\"mso-ligatures:standardcontextual\">Bob Wilson<o:p></o:p></span></b></p>\n<p class=\"MsoNormal\"><span style=\"mso-ligatures:standardcontextual\">Corporate Account Manager at Finance Company AB<o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"mso-ligatures:standardcontextual\">Tel: 555-0123<o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"mso-ligatures:standardcontextual\">Example Street 123, Box 456, 111 22 STOCKHOLM<o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"mso-ligatures:standardcontextual\"><a href=\"http://financecompany.se/\"><span style=\"color:#1B9830;mso-fareast-language:EN-GB\">financecompany.se</span></a></span><span style=\"mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"mso-ligatures:standardcontextual\"><a href=\"https://play.google.com/store/apps/details?id=se.finance companybank.androidapplikation\"><span style=\"color:#16A53F;mso-fareast-language:EN-GB\">Android-app</span></a></span><span style=\"mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"mso-ligatures:standardcontextual\"><a href=\"https://itunes.apple.com/se/app/finance company/id381311572?mt=8\"><span lang=\"EN-US\" style=\"color:#16A53F;mso-fareast-language:EN-GB\">iOS-app</span></a></span><span lang=\"EN-US\" style=\"mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"font-size:13.5pt;color:black;mso-fareast-language:EN-GB\"><img border=\"0\" width=\"200\" height=\"142\" style=\"width:2.0833in;height:1.4791in\" id=\"Picture_x0020_1\" src=\"cid:image001.png@01DBCFC2.879537F0\" alt=\"signature_260642366\"></span><span style=\"font-size:11.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:gray;mso-ligatures:standardcontextual\">Detta mejl kan innehålla konfidentiell information. Om du har fått det av misstag ber vi dig därför att inte</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:gray;mso-ligatures:standardcontextual\">kopiera eller vidarebefordra informationen annat än för att meddela avsändaren.\n</span><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">Radera sedan meddelandet.\n</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">Tänk också på att mejl kan omdirigeras, förvrängas och/eller innehålla till exempel virus; Försäkringsbolaget Finance Company AB tar\n</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">inget ansvar för någon sådan ev. åverkan. Försäkringsbolaget Finance Company AB. Styrelsens säte: Stockholm. Registration no. 123456-7890.</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">: Stockholm. Registration no. 123456-7890.</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><o:p>&nbsp;</o:p></p>\n<p class=\"MsoNormal\"><o:p>&nbsp;</o:p></p>\n</div>\n</body>\n</html>\n</blockquote></div>"
                  },
                  {
                    "id": 3,
                    "content-type": "image/jpeg",
                    "content-disposition": "attachment",
                    "filename": "Screenshot_20250528_113839_Chrome.jpg",
                    "content-transfer-encoding": "base64",
                    "content-length": 656441
                  },
                  {
                    "id": 4,
                    "content-type": "image/jpeg",
                    "content-disposition": "attachment",
                    "filename": "Screenshot_20250528_113909_Chrome.jpg",
                    "content-transfer-encoding": "base64",
                    "content-length": 669591
                  },
                  {
                    "id": 5,
                    "content-type": "image/jpeg",
                    "content-disposition": "attachment",
                    "filename": "Screenshot_20250528_114107_Chrome.jpg",
                    "content-transfer-encoding": "base64",
                    "content-length": 809577
                  },
                  {
                    "id": 6,
                    "content-type": "image/jpeg",
                    "content-disposition": "attachment",
                    "filename": "Screenshot_20250528_113653_Chrome.jpg",
                    "content-transfer-encoding": "base64",
                    "content-length": 504929
                  },
                  {
                    "id": 7,
                    "content-type": "image/jpeg",
                    "content-disposition": "attachment",
                    "filename": "Screenshot_20250528_113704_Chrome.jpg",
                    "content-transfer-encoding": "base64",
                    "content-length": 589690
                  },
                  {
                    "id": 8,
                    "content-type": "image/jpeg",
                    "content-disposition": "attachment",
                    "filename": "Screenshot_20250528_113734_Chrome.jpg",
                    "content-transfer-encoding": "base64",
                    "content-length": 441712
                  },
                  {
                    "id": 9,
                    "content-type": "image/jpeg",
                    "content-disposition": "attachment",
                    "filename": "Screenshot_20250528_114124_Chrome.jpg",
                    "content-transfer-encoding": "base64",
                    "content-length": 825362
                  }
                ]
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: Uppföljningsmöte mellan Alice och Bob, Finance Company AB",
              "From": "\"Alice Thompson\" <user@example.com>",
              "To": "\"user@example.com\" <user@example.com>",
              "Reply-To": "Alice Thompson <user@example.com>",
              "Date": "Wed, 28 May 2025 09:44:49 +0000"
            }
          },
          [
            [
              {
                "id": "user@example.com",
                "match": true,
                "excluded": false,
                "filename": [
                  "/home/user/Mail/archive/All Mail/new/1748426411.692537_2.icarus,U=182880:2,"
                ],
                "timestamp": 1748426343,
                "date_relative": "Wed. 11:59",
                "tags": [
                  "Important",
                  "inbox",
                  "unread"
                ],
                "duplicate": 1,
                "body": [
                  {
                    "id": 1,
                    "content-type": "text/html",
                    "content": "<html xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\" xmlns:w=\"urn:schemas-microsoft-com:office:word\" xmlns:m=\"http://schemas.microsoft.com/office/2004/12/omml\" xmlns=\"http://www.w3.org/TR/REC-html40\">\n<head>\n<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">\n<meta name=\"Generator\" content=\"Microsoft Word 15 (filtered medium)\">\n<!--[if !mso]><style>v\\:* {behavior:url(#default#VML);}\no\\:* {behavior:url(#default#VML);}\nw\\:* {behavior:url(#default#VML);}\n.shape {behavior:url(#default#VML);}\n</style><![endif]--><style><!--\n/* Font Definitions */\n@font-face\n\t{font-family:\"Cambria Math\";\n\tpanose-1:2 4 5 3 5 4 6 3 2 4;}\n@font-face\n\t{font-family:Calibri;\n\tpanose-1:2 15 5 2 2 2 4 3 2 4;}\n@font-face\n\t{font-family:Aptos;}\n/* Style Definitions */\np.MsoNormal, li.MsoNormal, div.MsoNormal\n\t{margin:0cm;\n\tfont-size:10.0pt;\n\tfont-family:\"Aptos\",sans-serif;}\na:link, span.MsoHyperlink\n\t{mso-style-priority:99;\n\tcolor:blue;\n\ttext-decoration:underline;}\nspan.EmailStyle21\n\t{mso-style-type:personal-reply;\n\tfont-family:\"Aptos\",sans-serif;\n\tcolor:windowtext;}\n.MsoChpDefault\n\t{mso-style-type:export-only;\n\tfont-size:10.0pt;\n\tmso-ligatures:none;}\n@page WordSection1\n\t{size:612.0pt 792.0pt;\n\tmargin:70.85pt 70.85pt 70.85pt 70.85pt;}\ndiv.WordSection1\n\t{page:WordSection1;}\n--></style><!--[if gte mso 9]><xml>\n<o:shapedefaults v:ext=\"edit\" spidmax=\"1026\" />\n</xml><![endif]--><!--[if gte mso 9]><xml>\n<o:shapelayout v:ext=\"edit\">\n<o:idmap v:ext=\"edit\" data=\"1\" />\n</o:shapelayout></xml><![endif]-->\n</head>\n<body lang=\"SV\" link=\"blue\" vlink=\"purple\" style=\"word-wrap:break-word\">\n<div class=\"WordSection1\">\n<p class=\"MsoNormal\"><span style=\"font-size:12.0pt;mso-fareast-language:EN-US\">Strålande, tack! Enjoy your upcoming time off and see you on the video call next week\n</span><span style=\"font-size:12.0pt;font-family:&quot;Segoe UI Emoji&quot;,sans-serif;mso-fareast-language:EN-US\">&#128522;</span><span style=\"font-size:12.0pt;mso-fareast-language:EN-US\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:12.0pt;mso-fareast-language:EN-US\"><o:p>&nbsp;</o:p></span></p>\n<div style=\"border:none;border-top:solid #E1E1E1 1.0pt;padding:3.0pt 0cm 0cm 0cm\">\n<p class=\"MsoNormal\"><b><span lang=\"EN-US\" style=\"font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif\">From:</span></b><span lang=\"EN-US\" style=\"font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif\"> Alice Thompson &lt;user@example.com&gt;\n<br>\n<b>Sent:</b> den 28 maj 2025 11:45<br>\n<b>To:</b> Bob Wilson &lt;user@example.com&gt;<br>\n<b>Subject:</b> Re: Uppföljningsmöte mellan Alice och Bob, Finance Company AB<o:p></o:p></span></p>\n</div>\n<p class=\"MsoNormal\"><o:p>&nbsp;</o:p></p>\n<p>Hej Bob,<o:p></o:p></p>\n<p>Thank you for the productive meeting. I am attaching some documentation from pension providers at Insurance Provider A and B.<o:p></o:p></p>\n<p>Mvh,<br>\nAlice<o:p></o:p></p>\n<div>\n<p class=\"MsoNormal\"><br>\n<br>\n-------- Ursprungligt meddelande --------<br>\nDen 2025-05-28 11:20, Bob Wilson skrev:<o:p></o:p></p>\n<blockquote style=\"margin-top:5.0pt;margin-bottom:5.0pt\">\n<div>\n<table class=\"MsoNormalTable\" border=\"0\" cellspacing=\"3\" cellpadding=\"0\" style=\"background:lightyellow\">\n<tbody>\n<tr>\n<td style=\"padding:.75pt .75pt .75pt .75pt\">\n<p class=\"MsoNormal\"><strong><span style=\"font-size:12.0pt;font-family:&quot;Aptos&quot;,sans-serif;color:red\">EXTERNAL EMAIL:</span></strong><span style=\"font-size:12.0pt;color:black\"> Do not click any links or open any attachments unless you trust the sender and know\n the content is safe.</span><span style=\"font-size:12.0pt\"><o:p></o:p></span></p>\n</td>\n</tr>\n</tbody>\n</table>\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\"><span style=\"font-size:12.0pt\">Hej Alice,<o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\"><span style=\"font-size:12.0pt\">This is a confirmation of our digital quarterly review meeting. We will meet via video conference\n<b>Wednesday June 4th</b> kl. <b>10.00</b>. Please accept the meeting invitation. The meeting link is below:\n<o:p></o:p></span></p>\n<p><a href=\"https://financecompany.zoom.us/j/55512345678\">https://financecompany.zoom.us/j/55512345678</a>\n<o:p></o:p></p>\n<p>Meeting ID: 555 123 4567 <o:p></o:p></p>\n<p>If you would like us to review your portfolio holdings as well, please reply with the current allocation details.\n<o:p></o:p></p>\n<p class=\"MsoNormal\" style=\"mso-margin-top-alt:auto;mso-margin-bottom-alt:auto\"><span style=\"font-size:12.0pt\">Med vänliga hälsningar,<o:p></o:p></span></p>\n<p class=\"MsoNormal\"><b><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual\">Bob Wilson<o:p></o:p></span></b></p>\n<p class=\"MsoNormal\"><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual\">Corporate Account Manager at Finance Company AB<o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual\">Tel: 555-0123<o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual\">Example Street 123, Box 456, 111 22 STOCKHOLM<o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual\"><a href=\"http://financecompany.se/\"><span style=\"color:#1B9830;mso-fareast-language:EN-GB\">financecompany.se</span></a></span><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual\"><a href=\"https://play.google.com/store/apps/details?id=se.finance companybank.androidapplikation\"><span style=\"color:#16A53F;mso-fareast-language:EN-GB\">Android-app</span></a></span><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"font-size:12.0pt;mso-ligatures:standardcontextual\"><a href=\"https://itunes.apple.com/se/app/finance company/id381311572?mt=8\"><span lang=\"EN-US\" style=\"color:#16A53F;mso-fareast-language:EN-GB\">iOS-app</span></a></span><span lang=\"EN-US\" style=\"font-size:12.0pt;mso-ligatures:standardcontextual;mso-fareast-language:EN-GB\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\" style=\"text-autospace:none\"><span style=\"font-size:13.5pt;color:black;mso-fareast-language:EN-GB\"><img border=\"0\" width=\"200\" height=\"142\" style=\"width:2.0833in;height:1.4791in\" id=\"_x0000_i1025\" src=\"cid:image001.png@01DBCFC2.879537F0\" alt=\"signature_260642366\"></span><span style=\"font-size:11.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:gray;mso-ligatures:standardcontextual\">Detta mejl kan innehålla konfidentiell information. Om du har fått det av misstag ber vi dig därför att inte</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:gray;mso-ligatures:standardcontextual\">kopiera eller vidarebefordra informationen annat än för att meddela avsändaren.\n</span><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">Radera sedan meddelandet.\n</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">Tänk också på att mejl kan omdirigeras, förvrängas och/eller innehålla till exempel virus; Försäkringsbolaget Finance Company AB tar\n</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">inget ansvar för någon sådan ev. åverkan. Försäkringsbolaget Finance Company AB. Styrelsens säte: Stockholm. Registration no. 123456-7890.</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:9.0pt;color:#7F7F7F;mso-ligatures:standardcontextual\">: Stockholm. Registration no. 123456-7890.</span><span style=\"font-size:9.0pt;mso-ligatures:standardcontextual\"><o:p></o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:12.0pt\"><o:p>&nbsp;</o:p></span></p>\n<p class=\"MsoNormal\"><span style=\"font-size:12.0pt\"><o:p>&nbsp;</o:p></span></p>\n</div>\n</blockquote>\n</div>\n</div>\n</body>\n</html>\n"
                  }
                ],
                "crypto": {},
                "headers": {
                  "Subject": "RE: Uppföljningsmöte mellan Alice och Bob, Finance Company AB",
                  "From": "\"Bob Wilson\" <user@example.com>",
                  "To": "Alice Thompson <user@example.com>",
                  "Reply-To": "Bob Wilson <user@example.com>",
                  "Date": "Wed, 28 May 2025 09:59:03 +0000"
                }
              },
              []
            ]
          ]
        ]
      ]
    ]
  ]
]
```
