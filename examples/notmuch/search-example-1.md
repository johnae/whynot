### Example search output

By running the below command (via ssh in this case):

```bash
ssh icarus notmuch search --limit=1 --format=json 'tag:Mailinglist' | jq .
```

You'll see this output:

```json
[
  {
    "thread": "00000000000276db",
    "timestamp": 1748767608,
    "date_relative": "46 mins. ago",
    "matched": 2,
    "total": 30,
    "authors": "itsTurnip, Kenny Levinsen| Hugo, Alexander Orzechowski, Simon Ser, Okami, Andre Esteve, minus, Vuk Mirovic, Kirill Primak, Conner Bondurant, Olivier Nicole, Daven Du, Isaac Freund, Dan Klishch, Stanislau T., marienz",
    "subject": "[swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
    "query": [
      "id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com",
      "id:swaywm/sway/issues/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com id:swaywm/sway/issues/8194/user@example.com"
    ],
    "tags": [
      "Mailinglist",
      "inbox",
      "unread"
    ]
  }
]
```

Note that the exact output may differ as new email comes in since we're searching for email with a certain tag.

### Viewing the thread
To actually view that thread, you would run this command (again, in this case via ssh):

```bash
[
  [
    [
      {
        "id": "swaywm/sway/issues/user@example.com",
        "match": true,
        "excluded": false,
        "filename": [
          "/home/user/Mail/archive/All Mail/cur/1742814138.1726233_152733.icarus,U=152733:2,T"
        ],
        "timestamp": 1717255984,
        "date_relative": "2024-06-01",
        "tags": [],
        "duplicate": 1,
        "body": [
          {
            "id": 1,
            "content-type": "text/html",
            "content": "<p></p>\n<ul dir=\"auto\">\n<li><strong>Sway Version:</strong></li>\n</ul>\n<p dir=\"auto\">sway version 1.10-dev-2686afb9 (May  7 2024, branch 'master')</p>\n<ul dir=\"auto\">\n<li><strong>Debug Log:</strong></li>\n</ul>\n<p dir=\"auto\"><a href=\"https://paste.sr.ht/~whynothugo/95a3445a0e678c2a642994d7662d5f0357ae1d14\" rel=\"nofollow\">https://paste.sr.ht/~whynothugo/95a3445a0e678c2a642994d7662d5f0357ae1d14</a></p>\n<ul dir=\"auto\">\n<li><strong>Stack Trace:</strong></li>\n</ul>\n<p dir=\"auto\">I didn't run sway with <code class=\"notranslate\">gdb</code>; will try again and follow-up.</p>\n<ul dir=\"auto\">\n<li><strong>Description:</strong></li>\n</ul>\n<p dir=\"auto\">Interacting with chromium makes sway crash. Sometimes it's when right clicking, but often times this happens when toggling fullscreen.</p>\n<p dir=\"auto\">I can't find an exact reproducer. Opening a terminal and chromium and moving the windows around / toggling full screen seems to toggle it. Sometimes it happens at the first right-click.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFCEYA2OFP73SEX2ADZFHSTBAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43ASLTON2WKOZSGMZDSMRQGIYDSNY\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGLKSDYZ2NAGV4C363ZFHSTBA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNFJFZXG5LFVJRW63LNMVXHIX3JMTHIVVGNWE.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
          }
        ],
        "crypto": {},
        "headers": {
          "Subject": "[swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
          "From": "\"Hugo\" <user@example.com>",
          "To": "swaywm/sway <user@example.com>",
          "Cc": "Subscribed <user@example.com>",
          "Reply-To": "swaywm/sway <user@example.com>",
          "Date": "Sat, 01 Jun 2024 08:33:04 -0700"
        }
      },
      [
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814138.1726233_152734.icarus,U=152734:2,T"
            ],
            "timestamp": 1717259721,
            "date_relative": "2024-06-01",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">bt:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">#1  0x00007ffff7fa9845 in raise (sig=sig@entry=6) at src/signal/raise.c:11\n        set = {__bits = {0, 206158430224, 140737488343872, 140737488343680, 1027, 1, 93824993928152, 93824993928080, 93824993928384, 23, 12020, 140737353616819, 140737488344048, 140737488344064, 140737488344048, 0}}\n        ret = 0\n#2  0x00007ffff7f78c21 in abort () at src/exit/abort.c:11\nNo locals.\n#3  0x00007ffff7f78ccf in __assert_fail (expr=&lt;optimized out&gt;, file=&lt;optimized out&gt;, line=&lt;optimized out&gt;, func=&lt;optimized out&gt;) at src/exit/assert.c:7\nNo locals.\n#4  0x000055555568aedd in wlr_render_pass_add_texture (render_pass=0x7fffe9856860, options=0x7fffffffd420) at ../subprojects/wlroots/render/pass.c:23\n        box = 0x7fffffffd428\n        __func__ = \"wlr_render_pass_add_texture\"\n#5  0x000055555562f57c in scene_entry_render (entry=0x7fffe8a8bff8, data=0x7fffffffd650) at ../subprojects/wlroots/types/scene/wlr_scene.c:1232\n        texture = 0x7fffe9855bb0\n        scene_rect = 0x55555572d8c0 &lt;surface_addon_impl&gt;\n        scene_buffer = 0x7fffe8d84560\n        transform = WL_OUTPUT_TRANSFORM_NORMAL\n        sample_event = {output = 0x7fffe95c12e0, direct_scanout = 56}\n        node = 0x7fffe8d84560\n        render_region = {extents = {x1 = 0, y1 = 45, x2 = 1897, y2 = 1027}, data = 0x0}\n        dst_box = {x = 0, y = 47, width = 1896, height = 979}\n        opaque = {extents = {x1 = 0, y1 = 45, x2 = 1897, y2 = 1027}, data = 0x7fffe98246f0}\n        __func__ = \"scene_entry_render\"\n#6  0x000055555563135c in wlr_scene_output_build_state (scene_output=0x7fffe95c12e0, state=0x7fffffffd6d0, options=0x7fffffffd590) at ../subprojects/wlroots/types/scene/wlr_scene.c:1923\n        entry = 0x7fffe8a8bff8\n        i = 1\n        default_options = {timer = 0x0, color_transform = 0x0, swapchain = 0x0}\n        timer = 0x0\n        start_time = {tv_sec = 2, tv_nsec = 140737115910664}\n        output = 0x7fffe994b6b0\n        debug_damage = WLR_SCENE_DEBUG_DAMAGE_NONE\n        render_data = {transform = WL_OUTPUT_TRANSFORM_NORMAL, scale = 1.5, logical = {x = 0, y = 0, width = 1280, height = 720}, trans_width = 1920, trans_height = 1080, output = 0x7fffe95c12e0, render_pass = 0x7fffe9856860,\n          damage = {extents = {x1 = 0, y1 = 45, x2 = 1897, y2 = 1027}, data = 0x0}}\n        resolution_width = 1920\n        resolution_height = 1080\n        list_con = {box = {x = 0, y = 0, width = 1280, height = 720}, render_list = 0x7fffe95c1418, calculate_visibility = true}\n        list_data = 0x7fffe8a8bfe0\n        list_len = 21\n        now = {tv_sec = 12507516256, tv_nsec = 140737115910648}\n        scanout = false\n        swapchain = 0x7fffea0ae870\n        buffer = 0x7fffe9432890\n        __func__ = \"wlr_scene_output_build_state\"\n        render_pass = 0x7fffe9856860\n        background = {extents = {x1 = 0, y1 = 46, x2 = 1897, y2 = 1027}, data = 0x7fffe98240b0}\n#7  0x000055555563074a in wlr_scene_output_commit (scene_output=0x7fffe95c12e0, options=0x0) at ../subprojects/wlroots/types/scene/wlr_scene.c:1683\n        ok = false\n        state = {committed = 2, allow_reconfiguration = false, damage = {extents = {x1 = 0, y1 = 45, x2 = 1897, y2 = 1027}, data = 0x0}, enabled = false, scale = 0, transform = WL_OUTPUT_TRANSFORM_NORMAL,\n          adaptive_sync_enabled = false, render_format = 0, subpixel = WL_OUTPUT_SUBPIXEL_UNKNOWN, buffer = 0x0, tearing_page_flip = false, mode_type = WLR_OUTPUT_STATE_MODE_FIXED, mode = 0x0, custom_mode = {width = 0,\n            height = 0, refresh = 0}, gamma_lut = 0x0, gamma_lut_size = 0, layers = 0x0, layers_len = 0}\n#8  0x00005555555945b1 in output_repaint_timer_handler (data=0x7fffe947f310) at ../sway/desktop/output.c:272\n        output = 0x7fffe947f310\n#9  0x0000555555594753 in handle_frame (listener=0x7fffe947f460, user_data=0x7fffe994b6b0) at ../sway/desktop/output.c:326\n        output = 0x7fffe947f310\n        msec_until_refresh = 0\n        delay = 0\n        data = {when = {tv_sec = 0, tv_nsec = 140737488345248}, msec_until_refresh = 2, output = 0x7fffffffd8c8}\n#10 0x00007ffff7a9c61d in wl_signal_emit_mutable () from /usr/lib/libwayland-server.so.0\nNo symbol table info available.\n#11 0x00005555556282e3 in wlr_output_send_frame (output=0x7fffe994b6b0) at ../subprojects/wlroots/types/output/output.c:753\nNo locals.\n#12 0x0000555555628327 in schedule_frame_handle_idle_timer (data=0x7fffe994b6b0) at ../subprojects/wlroots/types/output/output.c:761\n        output = 0x7fffe994b6b0\n#13 0x00007ffff7a9d935 in wl_event_loop_dispatch_idle () from /usr/lib/libwayland-server.so.0\nNo symbol table info available.\n#14 0x00007ffff7a9dae8 in wl_event_loop_dispatch () from /usr/lib/libwayland-server.so.0\nNo symbol table info available.\n#15 0x00007ffff7a9df17 in wl_display_run () from /usr/lib/libwayland-server.so.0\nNo symbol table info available.\n#16 0x0000555555590bae in server_run (server=0x555555737120 &lt;server&gt;) at ../sway/server.c:493\nNo locals.\n#17 0x000055555558f304 in main (argc=1, argv=0x7fffffffdc58) at ../sway/main.c:373\n        verbose = false\n        debug = false\n        validate = false\n        config_path = 0x0\n        c = -1\n</code></pre>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2143507637\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LA2SET73NBLGMU2TALZFHZ4TAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDCNBTGUYDONRTG4\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LHOTEGGPBWEP7HQY43ZFHZ4TA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTT7YNKLK.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2143507637</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2143507637\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2143507637\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sat, 01 Jun 2024 09:35:21 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814138.1726233_152785.icarus,U=152785:2,T"
            ],
            "timestamp": 1717336062,
            "date_relative": "2024-06-02",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">TBH, I don't really understand if this is the client sending bad data, or an issue in wlroots. This issue is a bit beyond my understanding of the codebase.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2143860640\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LGPFVNZZ7RDR3C5AKTZFMO75AVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDCNBTHA3DANRUGA\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFP27W2RNE6HZD3YR3ZFMO75A5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTT7ZC32A.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2143860640</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2143860640\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2143860640\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 02 Jun 2024 06:47:42 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814138.1726233_152880.icarus,U=152880:2,T"
            ],
            "timestamp": 1717435587,
            "date_relative": "2024-06-03",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">It's a wlroots issue. If a client sends bad state, it should never crash the entire compositor.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2145753343\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LBZGENMLHZYOZZFG23ZFSRMHAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDCNBVG42TGMZUGM\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFTXVORJOXZMIXOGKDZFSRMHA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTT74WMP6.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2145753343</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2145753343\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2145753343\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Alexander Orzechowski\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Mon, 03 Jun 2024 10:26:27 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814140.1726233_154158.icarus,U=154158:2,T"
            ],
            "timestamp": 1719213611,
            "date_relative": "2024-06-24",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">Can you provide a <code class=\"notranslate\">WAYLAND_DEBUG=1</code> log of your client?</p>\n<div class=\"highlight highlight-source-shell\" dir=\"auto\"><pre class=\"notranslate\">WAYLAND_DEBUG=1 your-client <span class=\"pl-k\">&gt;</span>client.log <span class=\"pl-k\">2&gt;&amp;1</span>\n<span class=\"pl-c\"><span class=\"pl-c\">#</span> Then reproduce the bug</span></pre></div>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2185787904\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LDHMWCKL2VZEVHJ673ZI7CCXAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDCOBVG44DOOJQGQ\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LD4E3TOS4HN2XXELTLZI7CCXA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTUCJB5AA.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2185787904</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2185787904\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2185787904\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Simon Ser\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Mon, 24 Jun 2024 00:20:11 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814144.1726233_156109.icarus,U=156109:2,ST"
            ],
            "timestamp": 1721747718,
            "date_relative": "2024-07-23",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">Here goes. It's a bit long; I don't have clear reproduction examples, I just resize the window, fullscreen it, move it around a few times, and eventually sway crashes.</p>\n<p dir=\"auto\"><a href=\"https://github.com/user-attachments/files/16350896/client.log\">client.log</a></p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2245531481\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFV65DGE5575LMZTADZNZXQNAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDENBVGUZTCNBYGE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LDI3UAK2Z5WSKUYX6DZNZXQNA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTUF3ALVS.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2245531481</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2245531481\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2245531481\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Tue, 23 Jul 2024 08:15:18 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814159.1726233_164608.icarus,U=164608:2,T"
            ],
            "timestamp": 1731521288,
            "date_relative": "2024-11-13",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">Firefox may trigger this too:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">#0  __pthread_kill_implementation (threadid=&lt;optimized out&gt;, signo=signo@entry=6, no_tid=no_tid@entry=0) at pthread_kill.c:44\n#1  ? in __pthread_kill_internal (threadid=&lt;optimized out&gt;, signo=6) at pthread_kill.c:78\n#2  ? in __GI_raise (sig=sig@entry=6) at ../sysdeps/posix/raise.c:26\n#3  ? in __GI_abort () at abort.c:79\n#4  ? in __assert_fail_base\n    (fmt=? \"%s%s%s:%u: %s%sAssertion `%s' failed.\\n%n\", assertion=assertion@entry=? \"box-&gt;x &gt;= 0 &amp;&amp; box-&gt;y &gt;= 0 &amp;&amp; box-&gt;x + box-&gt;width &lt;= options-&gt;texture-&gt;width &amp;&amp; box-&gt;y + box-&gt;height &lt;= options-&gt;texture-&gt;height\", file=file@entry=? \"render/pass.c\", line=line@entry=23, function=function@entry=? &lt;__PRETTY_FUNCTION__.1&gt; \"wlr_render_pass_add_texture\") at assert.c:94\n#5  ? in __assert_fail\n    (assertion=? \"box-&gt;x &gt;= 0 &amp;&amp; box-&gt;y &gt;= 0 &amp;&amp; box-&gt;x + box-&gt;width &lt;= options-&gt;texture-&gt;width &amp;&amp; box-&gt;y + box-&gt;height &lt;= options-&gt;texture-&gt;height\", file=? \"render/pass.c\", line=23, function=? &lt;__PRETTY_FUNCTION__.1&gt; \"wlr_render_pass_add_texture\") at assert.c:103\n#6  ? in wlr_render_pass_add_texture (render_pass=?, options=?) at ../subprojects/wlroots/render/pass.c:23\n#7  ? in scene_entry_render (entry=?, data=?) at ../subprojects/wlroots/types/scene/wlr_scene.c:1388\n#8  ? in wlr_scene_output_build_state (scene_output=?, state=?, options=?) at ../subprojects/wlroots/types/scene/wlr_scene.c:2192\n#9  ? in output_repaint_timer_handler (data=?) at ../sway/desktop/output.c:283\n#10 ? in handle_frame (listener=?, user_data=?) at ../sway/desktop/output.c:355\n#11 ? in wl_signal_emit_mutable (signal=&lt;optimized out&gt;, data=?) at ../wayland-1.23.1/src/wayland-server.c:2314\n#12 ? in wlr_output_send_frame (output=?) at ../subprojects/wlroots/types/output/output.c:787\n#13 ? in handle_page_flip (fd=10, seq=0, tv_sec=29181, tv_usec=482033, crtc_id=57, data=?) at ../subprojects/wlroots/backend/drm/drm.c:2001\n#14 ? in drmHandleEvent (fd=10, evctx=?) at ../libdrm-2.4.123/xf86drmMode.c:1070\n#15 ? in handle_drm_event (fd=10, mask=1, data=?) at ../subprojects/wlroots/backend/drm/drm.c:2013\n#16 ? in wl_event_loop_dispatch (loop=?, timeout=&lt;optimized out&gt;, timeout@entry=-1) at ../wayland-1.23.1/src/event-loop.c:1105\n#17 ? in wl_display_run (display=?) at ../wayland-1.23.1/src/wayland-server.c:1530\n#18 ? in server_run (server=? &lt;server&gt;) at ../sway/server.c:508\n#19 ? in main (argc=2, argv=?) at ../sway/main.c:373\n</code></pre>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2474374205\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFVPFQLP5CGSDDVBFL2AOIQRAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDINZUGM3TIMRQGU\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LHHEEEYGPXJM4A7PJ32AOIQRA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTUTPP2D2.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2474374205</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2474374205\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2474374205\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Okami\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Wed, 13 Nov 2024 10:08:08 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814159.1726233_164781.icarus,U=164781:2,T"
            ],
            "timestamp": 1731698300,
            "date_relative": "2024-11-15",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">I think I am getting the same on 1.10 release. Here's my traces in case it helps:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">libpng warning: sRGB: out of place\nlibpng warning: sRGB: out of place\nlibpng warning: sRGB: out of place\nwarn: quirks.c:80: applying wl_surface_damage_buffer() workaround for Sway\n00:04:24.474 [ERROR] [wlr] [xwayland/xwm.c:1192] Failed to get window property\n00:04:54.715 [ERROR] [wlr] [xwayland/xwm.c:1192] Failed to get window property\nlibpng warning: sRGB: out of place\nlibpng warning: sRGB: out of place\n00:06:21.960 [ERROR] [sway/sway_text_node.c:110] cairo_image_surface_create failed: invalid value (typically too big) for the size of the input (surface, pattern, etc.)\n00:06:21.960 [ERROR] [sway/sway_text_node.c:110] cairo_image_surface_create failed: invalid value (typically too big) for the size of the input (surface, pattern, etc.)\nsway: render/pass.c:23: wlr_render_pass_add_texture: Assertion `box-&gt;x &gt;= 0 &amp;&amp; box-&gt;y &gt;= 0 &amp;&amp; box-&gt;x + box-&gt;width &lt;= options-&gt;texture-&gt;width &amp;&amp; box-&gt;y + box-&gt;height &lt;= options-&gt;texture-&gt;height' failed.\n(EE)[2024-11-15 10:59:33.889] [error] Scratchpad: Unable to receive IPC header\n failed to read Wayland events: Connection reset by peer\n</code></pre>\n<p dir=\"auto\">And the core dump trace:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">Stack trace of thread 723:\n#0  0x000077dc2cb723f4 n/a (libc.so.6 + 0x963f4)\n#1  0x000077dc2cb19120 raise (libc.so.6 + 0x3d120)\n#2  0x000077dc2cb004c3 abort (libc.so.6 + 0x244c3)\n#3  0x000077dc2cb003df n/a (libc.so.6 + 0x243df)\n#4  0x000077dc2cb11177 __assert_fail (libc.so.6 + 0x35177)\n#5  0x000077dc2cd8440e wlr_render_pass_add_texture (libwlroots-0.18.so + 0x2b40e)\n#6  0x000077dc2cdc5f91 wlr_scene_output_build_state (libwlroots-0.18.so + 0x6cf91)\n#7  0x000058e56d1f6e0e n/a (sway + 0x1ee0e)\n#8  0x000058e56d1f7057 n/a (sway + 0x1f057)\n#9  0x000077dc2ce7747e wl_signal_emit_mutable (libwayland-server.so.0 + 0x847e)\n#10 0x000077dc2ce78efc wl_event_loop_dispatch_idle (libwayland-server.so.0 + 0x9efc)\n#11 0x000077dc2ce79177 wl_event_loop_dispatch (libwayland-server.so.0 + 0xa177)\n#12 0x000077dc2ce7b1f7 wl_display_run (libwayland-server.so.0 + 0xc1f7)\n#13 0x000058e56d1e7dd2 n/a (sway + 0xfdd2)\n#14 0x000077dc2cb01e08 n/a (libc.so.6 + 0x25e08)\n#15 0x000077dc2cb01ecc __libc_start_main (libc.so.6 + 0x25ecc)\n#16 0x000058e56d1e8275 n/a (sway + 0x10275)\n</code></pre>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2479758512\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LBJQ3UTSFC5HGSJTBD2AZCHZAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDINZZG42TQNJRGI\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LCMOEJX2GHEL7R2YMD2AZCHZA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTUTZYOLA.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2479758512</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2479758512\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2479758512\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Andre Esteve\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Fri, 15 Nov 2024 11:18:20 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814166.1726233_168688.icarus,U=168688:2,T"
            ],
            "timestamp": 1736260098,
            "date_relative": "January 07",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">A better reproduction example is sorely needed here. My current approach is to just run chromium, resize it and move it around a lot, and eventually the issue triggers. But this can take between 1 second and several minutes, and produces massive logs.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2575424050\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LHIWYLRK4CA6LHZEYD2JPQAFAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDKNZVGQZDIMBVGA\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LE74T23U3XWIDWWYRL2JPQAFA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTUZQHNDE.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2575424050</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2575424050\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2575424050\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Tue, 07 Jan 2025 06:28:18 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814168.1726233_169509.icarus,U=169509:2,T"
            ],
            "timestamp": 1737302308,
            "date_relative": "January 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">I accidentally found a reliable reproducer: A massive window title triggers it. Here's two PoC HTML files: <a href=\"https://github.com/user-attachments/files/18469391/black-bar.txt\">black-bar.txt</a> causes the title bar to only show a black bar, and doubling the title size (<a href=\"https://github.com/user-attachments/files/18469390/crash.txt\">crash.txt</a>) causes a crash.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600921675\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFLQLSW3G6HIYVQME32LPDSJAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMBQHEZDCNRXGU\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGKUSEA3ZCV7EJ7R7L2LPDSJA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3A3VEW.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2600921675</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600921675\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600921675\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"minus\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 19 Jan 2025 07:58:28 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814168.1726233_169510.icarus,U=169510:2,T"
            ],
            "timestamp": 1737302957,
            "date_relative": "January 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">In a tabbed layout, opening <code class=\"notranslate\">black-bar.html</code> in Firefox immediately crashed sway for me. It didn't crash sway in a vertical split.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600925559\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LF2GK55ANIDENBDBCT2LPE23AVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMBQHEZDKNJVHE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LBHF252SIAW44ZI3PD2LPE23A5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3A34XO.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2600925559</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600925559\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600925559\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 19 Jan 2025 08:09:17 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814168.1726233_169513.icarus,U=169513:2,T"
            ],
            "timestamp": 1737303947,
            "date_relative": "January 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\"><a class=\"user-mention notranslate\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/minus7/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"https://github.com/minus7\">@minus7</a> both files in both Firefox and Brave (Chromium based) show only black bar and no crash here.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600930921\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LD64MWVBAOYQO72QK32LPGYXAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMBQHEZTAOJSGE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LBZZBQJMT4S5VHNXHD2LPGYXA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3A4HGS.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2600930921</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600930921\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600930921\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Vuk Mirovic\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 19 Jan 2025 08:25:47 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814168.1726233_169514.icarus,U=169514:2,T"
            ],
            "timestamp": 1737304197,
            "date_relative": "January 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\"><a class=\"user-mention notranslate\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/wooque/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"https://github.com/wooque\">@wooque</a> which version of sway? Can you try moving that Firefox window into a top-level tabbed container?</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600932430\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LCIZ73PPM2G4CSPBXL2LPHILAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMBQHEZTENBTGA\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LDMR76525ZB5FAOFKL2LPHILA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3A4KE4.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2600932430</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600932430\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600932430\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 19 Jan 2025 08:29:57 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814168.1726233_169515.icarus,U=169515:2,T"
            ],
            "timestamp": 1737304915,
            "date_relative": "January 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">Can someone reproduce this with the following wlroots patch and check if my guess at what happens here is correct?</p>\n<div class=\"highlight highlight-source-diff\" dir=\"auto\"><pre class=\"notranslate\"><span class=\"pl-c1\">diff --git a/types/scene/surface.c b/types/scene/surface.c</span>\nindex 2aff5af3..a2acf272 100644\n<span class=\"pl-md\">--- a/types/scene/surface.c</span>\n<span class=\"pl-mi1\">+++ b/types/scene/surface.c</span>\n<span class=\"pl-mdr\">@@ -1,18 +1,19 @@</span>\n #include &lt;assert.h&gt;\n #include &lt;stdlib.h&gt;\n #include &lt;wlr/types/wlr_alpha_modifier_v1.h&gt;\n #include &lt;wlr/types/wlr_compositor.h&gt;\n #include &lt;wlr/types/wlr_scene.h&gt;\n #include &lt;wlr/types/wlr_fractional_scale_v1.h&gt;\n #include &lt;wlr/types/wlr_linux_drm_syncobj_v1.h&gt;\n #include &lt;wlr/types/wlr_presentation_time.h&gt;\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span>#include &lt;wlr/util/log.h&gt;</span>\n #include &lt;wlr/util/transform.h&gt;\n #include \"types/wlr_scene.h\"\n\n static void handle_scene_buffer_outputs_update(\n \t\tstruct wl_listener *listener, void *data) {\n \tstruct wlr_scene_surface *surface =\n \t\twl_container_of(listener, surface, outputs_update);\n\n \tif (surface-&gt;buffer-&gt;primary_output == NULL) {\n \t\treturn;\n<span class=\"pl-mdr\">@@ -113,34 +114,42 @@</span> static void surface_reconfigure(struct wlr_scene_surface *scene_surface) {\n \tpixman_region32_t opaque;\n \tpixman_region32_init(&amp;opaque);\n \tpixman_region32_copy(&amp;opaque, &amp;surface-&gt;opaque_region);\n\n \tint width = state-&gt;width;\n \tint height = state-&gt;height;\n\n \tif (!wlr_box_empty(&amp;scene_surface-&gt;clip)) {\n \t\tstruct wlr_box *clip = &amp;scene_surface-&gt;clip;\n\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span>\t\tstruct wlr_fbox orig = src_box;</span>\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span></span>\n \t\tint buffer_width = state-&gt;buffer_width;\n \t\tint buffer_height = state-&gt;buffer_height;\n \t\twidth = min(clip-&gt;width, width - clip-&gt;x);\n \t\theight = min(clip-&gt;height, height - clip-&gt;y);\n\n \t\twlr_fbox_transform(&amp;src_box, &amp;src_box, state-&gt;transform,\n \t\t\tbuffer_width, buffer_height);\n \t\twlr_output_transform_coords(state-&gt;transform, &amp;buffer_width, &amp;buffer_height);\n\n \t\tsrc_box.x += (double)(clip-&gt;x * buffer_width) / state-&gt;width;\n \t\tsrc_box.y += (double)(clip-&gt;y * buffer_height) / state-&gt;height;\n \t\tsrc_box.width *= (double)width / state-&gt;width;\n \t\tsrc_box.height *= (double)height / state-&gt;height;\n\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span>\t\tif (src_box.x + src_box.width &gt; orig.x + orig.width || src_box.y + src_box.height &gt; orig.y + orig.height) {</span>\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span>\t\t\twlr_log(WLR_ERROR, \"!!! src_box has been expanded during clipping !!!\");</span>\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span>\t\t\twlr_log(WLR_ERROR, \" from %f,%f %fx%f | right = %f bottom = %f\", orig.x, orig.y, orig.width, orig.height, orig.x + orig.width, orig.y + orig.height);</span>\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span>\t\t\twlr_log(WLR_ERROR, \"   to %f,%f %fx%f | right = %f bottom = %f\", src_box.x, src_box.y, src_box.width, src_box.height, src_box.x + src_box.width, src_box.y + src_box.height);</span>\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span>\t\t}</span>\n<span class=\"pl-mi1\"><span class=\"pl-mi1\">+</span></span>\n \t\twlr_fbox_transform(&amp;src_box, &amp;src_box, wlr_output_transform_invert(state-&gt;transform),\n \t\t\tbuffer_width, buffer_height);\n\n \t\tpixman_region32_translate(&amp;opaque, -clip-&gt;x, -clip-&gt;y);\n \t\tpixman_region32_intersect_rect(&amp;opaque, &amp;opaque, 0, 0, width, height);\n \t}\n\n \tif (width &lt;= 0 || height &lt;= 0) {\n \t\twlr_scene_buffer_set_buffer(scene_buffer, NULL);\n \t\tpixman_region32_fini(&amp;opaque);</pre></div>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600936550\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LD7KVVARFMZ2JTTZXD2LPIVHAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMBQHEZTMNJVGA\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LDR7PPHNXO3LO3RSZL2LPIVHA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3A4SGM.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2600936550</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600936550\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600936550\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Kirill Primak\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 19 Jan 2025 08:41:55 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814168.1726233_169516.icarus,U=169516:2,T"
            ],
            "timestamp": 1737306123,
            "date_relative": "January 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\"><a class=\"user-mention notranslate\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/WhyNotHugo/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"https://github.com/WhyNotHugo\">@WhyNotHugo</a></p>\n<p dir=\"auto\">1.10<br>\nhere is the screenshot, only black bar shown, no crashing.</p>\n<p dir=\"auto\"><a href=\"https://github.com/user-attachments/assets/639d7a25-3f58-4e78-a161-6143a5719378\">20250119_18h00m22s_grim.png (view on web)</a></p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600943530\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LGZQX7XFNLUX6SNRL32LPLAXAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMBQHE2DGNJTGA\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LEXVXKSCEH3IG5P5ID2LPLAXA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3A472U.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2600943530</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600943530\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2600943530\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Vuk Mirovic\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 19 Jan 2025 09:02:03 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814168.1726233_169663.icarus,U=169663:2,T"
            ],
            "timestamp": 1737479627,
            "date_relative": "January 21",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">Sway renders a black bar or crashes depending on the length of the titlebar. I'd guess that the screen resolution/scale is also relevant here.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2605306751\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LCDSPJHJRHLBSUQMYD2LZ54XAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMBVGMYDMNZVGE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGS2ULL7REC2HOHGHT2LZ54XA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3JHJX6.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2605306751</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2605306751\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2605306751\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Tue, 21 Jan 2025 09:13:47 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814169.1726233_169829.icarus,U=169829:2,T"
            ],
            "timestamp": 1737651539,
            "date_relative": "January 23",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">I believe I might be also having this issue, as I crash in the same circumstance.<br>\nMy best way to reproduce the behavior is with the Reaper DAW.<br>\nI have a 100% replication rate across 5-6 separate attempts so far when I open <code class=\"notranslate\">ReaSamplOmatic5000</code> in a plugin window, float the window, and then attempt to increase its width in any amount.</p>\n<p dir=\"auto\">I am currently working on generating a proper and useful coredump.</p>\n<p dir=\"auto\">Will build with the posted patch and add more information once I have more results to share.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2610422238\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LGGL6V42DC24NGKNQT2MENVHAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMJQGQZDEMRTHA\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LDT5URBPOKREI35V4T2MENVHA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3S7Q54.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2610422238</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2610422238\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2610422238\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Conner Bondurant\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Thu, 23 Jan 2025 08:58:59 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814169.1726233_169844.icarus,U=169844:2,T"
            ],
            "timestamp": 1737664295,
            "date_relative": "January 23",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">Additional detail I have identified: I have used <code class=\"notranslate\">WLR_RENDERER=vulkan</code> on my system for a long time now, because at the time I enabled it, I had been dealing with horrible flickering in certain applications.</p>\n<p dir=\"auto\">As part of testing I had to exclude that environment configuration, and I cannot replicate the crash without the vulkan renderer. It is possible that it is the cause.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2610956778\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LD3NYVUWETGH5YGUCD2MFGSPAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMJQHE2TMNZXHA\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LCLYR5JU32IIRTEJED2MFGSPA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU3UAE6U.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2610956778</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2610956778\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2610956778\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Conner Bondurant\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Thu, 23 Jan 2025 12:31:35 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814169.1726233_170071.icarus,U=170071:2,T"
            ],
            "timestamp": 1737976444,
            "date_relative": "January 27",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p></p>\n<p dir=\"auto\">I had the same crash on 1.10 that I can reproduce by visiting a web page leading to a very long window title as suggested by <a class=\"user-mention notranslate\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/minus7/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"https://github.com/minus7\">@minus7</a>, with both Firefox and Chromium.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2615471585\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LDDLTU5JYLXTBJJHE32MYIHZAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMMJVGQ3TCNJYGU\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LESXEDOISO3IPDCNB32MYIHZA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU34TW6C.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2615471585</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2615471585\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2615471585\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Olivier Nicole\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Mon, 27 Jan 2025 03:14:04 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814171.1726233_171322.icarus,U=171322:2,T"
            ],
            "timestamp": 1739555178,
            "date_relative": "February 14",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">I suspect this is related with fractional scaling. On sway <code class=\"notranslate\">1.10.1</code>, my screen is configured like this:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\"># 3 displays\noutput HDMI-A-1 scale 1.5\noutput DP-1     scale 1.5\noutput DP-2     scale 1.5 transform 270\n\noutput DP-1     pos 1440 1440\noutput DP-2     pos 0     0\noutput HDMI-A-1 pos 1440  0\n</code></pre>\n<p dir=\"auto\">And have those monitors (some information removed to shorten message):</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">&gt; swaymsg -t get_outputs\nOutput HDMI-A-1\n  Current mode: 3840x2160 @ 60.000 Hz\n  Position: 1440,0\n  Scale factor: 1.500000\n  Scale filter: linear\n  Subpixel hinting: unknown\n  Transform: normal\n  Adaptive sync: disabled\n  Allow tearing: no\n  Available modes: ...\n\nOutput DP-2\n  Current mode: 3840x2160 @ 59.997 Hz\n  Position: 0,0\n  Scale factor: 1.500000\n  Scale filter: linear\n  Subpixel hinting: unknown\n  Transform: 270\n  Adaptive sync: disabled\n  Allow tearing: no\n  Available modes: ...\n\nOutput DP-1\n  Current mode: 3840x2160 @ 59.997 Hz\n  Position: 1440,1440\n  Scale factor: 1.500000\n  Scale filter: linear\n  Subpixel hinting: unknown\n  Transform: normal\n  Adaptive sync: disabled\n  Allow tearing: no\n  Available modes: ...\n</code></pre>\n<p dir=\"auto\">Operations on <code class=\"notranslate\">DP-1</code> is have a higher risk of causing crash with following stack:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">...\n#5  0x00007fbbf54ddc30 in __assert_fail (assertion=&lt;optimized out&gt;, file=&lt;optimized out&gt;, line=&lt;optimized out&gt;, function=&lt;optimized out&gt;) at assert.c:127\n#6  0x00007fbbf576040e in wlr_render_pass_add_texture (render_pass=0x56a2a7a478d0, options=0x7fff490e3470) at ../wlroots-0.18.2/render/pass.c:23\n#7  0x00007fbbf57a2311 in scene_entry_render (entry=0x56a2a7aed3b8, data=0x7fff490e3420) at ../wlroots-0.18.2/types/scene/wlr_scene.c:1270\n#8  wlr_scene_output_build_state (scene_output=0x56a2a6b85cc0, state=state@entry=0x7fff490e3560, options=&lt;optimized out&gt;, options@entry=0x7fff490e3540) at ../wlroots-0.18.2/types/scene/wlr_scene.c:1959\n#9  0x000056a29e601060 in output_repaint_timer_handler (data=data@entry=0x56a2a6bc3430) at ../sway/sway/desktop/output.c:285\n...\n</code></pre>\n<p dir=\"auto\">Source code and related variables:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">(gdb) list\n18\t\t\tconst struct wlr_render_texture_options *options) {\n19\t\t// make sure the texture source box does not try and sample outside of the\n20\t\t// texture\n21\t\tif (!wlr_fbox_empty(&amp;options-&gt;src_box)) {\n22\t\t\tconst struct wlr_fbox *box = &amp;options-&gt;src_box;\n23\t\t\tassert(box-&gt;x &gt;= 0 &amp;&amp; box-&gt;y &gt;= 0 &amp;&amp;\n24\t\t\t\tbox-&gt;x + box-&gt;width &lt;= options-&gt;texture-&gt;width &amp;&amp;\n25\t\t\t\tbox-&gt;y + box-&gt;height &lt;= options-&gt;texture-&gt;height);\n26\t\t}\n27\n(gdb) print *box\n$2 = {\n  x = 24,\n  y = 15.008103727714749,\n  width = 2136,\n  height = 910.99189627228532\n}\n(gdb) print *options-&gt;texture\n$3 = {\n  impl = 0x7fbbf583f540 &lt;texture_impl.lto_priv&gt;,\n  width = 0x870,\n  height = 0x39e,\n  renderer = 0x56a2a67487d0\n}\n(gdb) print 15.008103727714749+910.99189627228532\n$4 = 926.00000000000011\n(gdb) print 926\n$5 = 0x39e\n</code></pre>\n<p dir=\"auto\">I'm not familiar with Wayland stack, but I suspect this is some float point rounding margins during calculating texture decoration, and causing this assert failed.</p>\n<p dir=\"auto\">Still, I'm unable to distinguish if this is sway's problem, or wlroot's. Checked some previous issues including <code class=\"notranslate\">wlroots/wlroots #3766, #3790</code> but I don't think they are related with this bug.</p>\n<p dir=\"auto\"><a class=\"user-mention notranslate\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/emersion/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"https://github.com/emersion\">@emersion</a> Do you want a copy of coredump, or anything else I can do to help?</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2659926629\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFLWK4HEA5AKQ7IPYT2PYTWVAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNJZHEZDMNRSHE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFC4XFLTPHRZFAHUID2PYTWVA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU6RNBGK.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2659926629</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"davendu\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/58910463?s=20&amp;v=4\" /><strong>davendu</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2659926629\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">I suspect this is related with fractional scaling. On sway <code class=\"notranslate\">1.10.1</code>, my screen is configured like this:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\"># 3 displays\noutput HDMI-A-1 scale 1.5\noutput DP-1     scale 1.5\noutput DP-2     scale 1.5 transform 270\n\noutput DP-1     pos 1440 1440\noutput DP-2     pos 0     0\noutput HDMI-A-1 pos 1440  0\n</code></pre>\n<p dir=\"auto\">And have those monitors (some information removed to shorten message):</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">&gt; swaymsg -t get_outputs\nOutput HDMI-A-1\n  Current mode: 3840x2160 @ 60.000 Hz\n  Position: 1440,0\n  Scale factor: 1.500000\n  Scale filter: linear\n  Subpixel hinting: unknown\n  Transform: normal\n  Adaptive sync: disabled\n  Allow tearing: no\n  Available modes: ...\n\nOutput DP-2\n  Current mode: 3840x2160 @ 59.997 Hz\n  Position: 0,0\n  Scale factor: 1.500000\n  Scale filter: linear\n  Subpixel hinting: unknown\n  Transform: 270\n  Adaptive sync: disabled\n  Allow tearing: no\n  Available modes: ...\n\nOutput DP-1\n  Current mode: 3840x2160 @ 59.997 Hz\n  Position: 1440,1440\n  Scale factor: 1.500000\n  Scale filter: linear\n  Subpixel hinting: unknown\n  Transform: normal\n  Adaptive sync: disabled\n  Allow tearing: no\n  Available modes: ...\n</code></pre>\n<p dir=\"auto\">Operations on <code class=\"notranslate\">DP-1</code> is have a higher risk of causing crash with following stack:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">...\n#5  0x00007fbbf54ddc30 in __assert_fail (assertion=&lt;optimized out&gt;, file=&lt;optimized out&gt;, line=&lt;optimized out&gt;, function=&lt;optimized out&gt;) at assert.c:127\n#6  0x00007fbbf576040e in wlr_render_pass_add_texture (render_pass=0x56a2a7a478d0, options=0x7fff490e3470) at ../wlroots-0.18.2/render/pass.c:23\n#7  0x00007fbbf57a2311 in scene_entry_render (entry=0x56a2a7aed3b8, data=0x7fff490e3420) at ../wlroots-0.18.2/types/scene/wlr_scene.c:1270\n#8  wlr_scene_output_build_state (scene_output=0x56a2a6b85cc0, state=state@entry=0x7fff490e3560, options=&lt;optimized out&gt;, options@entry=0x7fff490e3540) at ../wlroots-0.18.2/types/scene/wlr_scene.c:1959\n#9  0x000056a29e601060 in output_repaint_timer_handler (data=data@entry=0x56a2a6bc3430) at ../sway/sway/desktop/output.c:285\n...\n</code></pre>\n<p dir=\"auto\">Source code and related variables:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">(gdb) list\n18\t\t\tconst struct wlr_render_texture_options *options) {\n19\t\t// make sure the texture source box does not try and sample outside of the\n20\t\t// texture\n21\t\tif (!wlr_fbox_empty(&amp;options-&gt;src_box)) {\n22\t\t\tconst struct wlr_fbox *box = &amp;options-&gt;src_box;\n23\t\t\tassert(box-&gt;x &gt;= 0 &amp;&amp; box-&gt;y &gt;= 0 &amp;&amp;\n24\t\t\t\tbox-&gt;x + box-&gt;width &lt;= options-&gt;texture-&gt;width &amp;&amp;\n25\t\t\t\tbox-&gt;y + box-&gt;height &lt;= options-&gt;texture-&gt;height);\n26\t\t}\n27\n(gdb) print *box\n$2 = {\n  x = 24,\n  y = 15.008103727714749,\n  width = 2136,\n  height = 910.99189627228532\n}\n(gdb) print *options-&gt;texture\n$3 = {\n  impl = 0x7fbbf583f540 &lt;texture_impl.lto_priv&gt;,\n  width = 0x870,\n  height = 0x39e,\n  renderer = 0x56a2a67487d0\n}\n(gdb) print 15.008103727714749+910.99189627228532\n$4 = 926.00000000000011\n(gdb) print 926\n$5 = 0x39e\n</code></pre>\n<p dir=\"auto\">I'm not familiar with Wayland stack, but I suspect this is some float point rounding margins during calculating texture decoration, and causing this assert failed.</p>\n<p dir=\"auto\">Still, I'm unable to distinguish if this is sway's problem, or wlroot's. Checked some previous issues including <code class=\"notranslate\">wlroots/wlroots #3766, #3790</code> but I don't think they are related with this bug.</p>\n<p dir=\"auto\"><a class=\"user-mention notranslate\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/emersion/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"https://github.com/emersion\">@emersion</a> Do you want a copy of coredump, or anything else I can do to help?</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2659926629\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFLWK4HEA5AKQ7IPYT2PYTWVAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNJZHEZDMNRSHE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFC4XFLTPHRZFAHUID2PYTWVA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU6RNBGK.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2659926629</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2659926629\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2659926629\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Daven Du\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Fri, 14 Feb 2025 09:46:18 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814172.1726233_171577.icarus,U=171577:2,T"
            ],
            "timestamp": 1739890150,
            "date_relative": "February 18",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">It would be interesting to know if anyone can reproduce this when using a wlroots version with this patch merged: <a href=\"https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981\" rel=\"nofollow\">https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981</a></p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2665935361\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LDVW7MP764XSKDSAKL2QNB6NAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNRVHEZTKMZWGE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFBLZOLP3RXMVVOXXD2QNB6NA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU643ZAC.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2665935361</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"ifreund\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/12723818?s=20&amp;v=4\" /><strong>ifreund</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2665935361\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">It would be interesting to know if anyone can reproduce this when using a wlroots version with this patch merged: <a href=\"https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981\" rel=\"nofollow\">https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981</a></p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2665935361\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LDVW7MP764XSKDSAKL2QNB6NAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNRVHEZTKMZWGE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFBLZOLP3RXMVVOXXD2QNB6NA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU643ZAC.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2665935361</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2665935361\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2665935361\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Isaac Freund\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Tue, 18 Feb 2025 06:49:10 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814172.1726233_171621.icarus,U=171621:2,T"
            ],
            "timestamp": 1739930561,
            "date_relative": "February 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">I've just compiled latest masters (sway's <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/swaywm/sway/commit/10e50e6bf9b63b205c141f97a5709fd4d405542f/hovercard\" href=\"https://github.com/swaywm/sway/commit/10e50e6bf9b63b205c141f97a5709fd4d405542f\"><tt>10e50e6</tt></a> and wlroots' dc7dba8b). The exact same assertion triggers. (Interestingly, for me, black-bar.html causes crash as well)</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2667333795\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LBSAWSNYYRHQI5BRCD2QPQ4DAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNRXGMZTGNZZGU\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGXBAABTOU67TDYCC32QPQ4DA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU67REKG.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2667333795</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"DanShaders\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/30951924?s=20&amp;v=4\" /><strong>DanShaders</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2667333795\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">I've just compiled latest masters (sway's <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/swaywm/sway/commit/10e50e6bf9b63b205c141f97a5709fd4d405542f/hovercard\" href=\"https://github.com/swaywm/sway/commit/10e50e6bf9b63b205c141f97a5709fd4d405542f\"><tt>10e50e6</tt></a> and wlroots' dc7dba8b). The exact same assertion triggers. (Interestingly, for me, black-bar.html causes crash as well)</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2667333795\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LBSAWSNYYRHQI5BRCD2QPQ4DAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNRXGMZTGNZZGU\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGXBAABTOU67TDYCC32QPQ4DA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU67REKG.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2667333795</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2667333795\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2667333795\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Dan Klishch\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Tue, 18 Feb 2025 18:02:41 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814172.1726233_171655.icarus,U=171655:2,T"
            ],
            "timestamp": 1739958753,
            "date_relative": "February 19",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">My impression so far is that very wide text renders a black titlebar, and even wider text causes it to crash.</p>\n<p dir=\"auto\">The exact definition of \"wide\" depends on the actual size of the titlebar, hence, for some users a given title causes a crash, but for users of much higher resolutions (with a much larger window) it produces only a black titlebar.</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2668100846\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LGJT6JAOERIVFBD4CT2QRH6DAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNRYGEYDAOBUGY\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LELM3F44OOM7EWYP7T2QRH6DA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU7A76O4.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2668100846</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"WhyNotHugo\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/730811?s=20&amp;v=4\" /><strong>WhyNotHugo</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2668100846\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">My impression so far is that very wide text renders a black titlebar, and even wider text causes it to crash.</p>\n<p dir=\"auto\">The exact definition of \"wide\" depends on the actual size of the titlebar, hence, for some users a given title causes a crash, but for users of much higher resolutions (with a much larger window) it produces only a black titlebar.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2668100846\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LGJT6JAOERIVFBD4CT2QRH6DAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMNRYGEYDAOBUGY\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LELM3F44OOM7EWYP7T2QRH6DA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU7A76O4.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2668100846</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2668100846\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2668100846\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Hugo\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Wed, 19 Feb 2025 01:52:33 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814173.1726233_172051.icarus,U=172051:2,T"
            ],
            "timestamp": 1740477916,
            "date_relative": "February 25",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">For the black box on long titles, see: <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"2877850445\" data-permission-text=\"Title is private\" data-url=\"https://github.com/swaywm/sway/issues/8586\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/swaywm/sway/pull/8586/hovercard\" href=\"https://github.com/swaywm/sway/pull/8586\">#8586</a></p>\n<p dir=\"auto\">Not sure if it also fixes the source box assert.</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2681420414\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LDNOVXFSJ5FX3HD4ED2RQ55ZAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMOBRGQZDANBRGQ\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LF5QD6CJTI2TGO7SPL2RQ55ZA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU72M5H4.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2681420414</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"kennylevinsen\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/176245?s=20&amp;v=4\" /><strong>kennylevinsen</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2681420414\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">For the black box on long titles, see: <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"2877850445\" data-permission-text=\"Title is private\" data-url=\"https://github.com/swaywm/sway/issues/8586\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/swaywm/sway/pull/8586/hovercard\" href=\"https://github.com/swaywm/sway/pull/8586\">#8586</a></p>\n<p dir=\"auto\">Not sure if it also fixes the source box assert.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2681420414\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LDNOVXFSJ5FX3HD4ED2RQ55ZAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMOBRGQZDANBRGQ\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LF5QD6CJTI2TGO7SPL2RQ55ZA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTU72M5H4.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2681420414</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2681420414\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2681420414\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Kenny Levinsen\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Tue, 25 Feb 2025 02:05:16 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742814125.1726233_145774.icarus,U=145774:2,ST"
            ],
            "timestamp": 1741033515,
            "date_relative": "March 03",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">JFYI if you urgently need to mitigate this without rebuilding sway: switching the browser window into floating mode (mod+shift+space by default) seems to work.</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2695458972\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LEAXEXUC2WADQMPNRD2SS3CXAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMOJVGQ2TQOJXGI\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGOY6QPPCE326DLLED2SS3CXA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVAVFYJY.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2695458972</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"xtsm\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/44941959?s=20&amp;v=4\" /><strong>xtsm</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2695458972\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">JFYI if you urgently need to mitigate this without rebuilding sway: switching the browser window into floating mode (mod+shift+space by default) seems to work.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2695458972\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LEAXEXUC2WADQMPNRD2SS3CXAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDMOJVGQ2TQOJXGI\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGOY6QPPCE326DLLED2SS3CXA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVAVFYJY.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2695458972</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2695458972\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2695458972\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Stanislau T.\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Mon, 03 Mar 2025 12:25:15 -0800"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742813852.1726233_1239.icarus,U=1239:2,T"
            ],
            "timestamp": 1741527763,
            "date_relative": "March 09",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">I'm triggering what I'm currently assuming is the same crash by opening a print preview window in Chrome (using its wayland Ozone backend), using Sway 1.10.1 and wlroots 0.18.2 with the patch from <a href=\"https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981\" rel=\"nofollow\">https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981</a> applied. Same \"cairo_image_surface_create failed\", then:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">(gdb) bt\n(assert stuff omitted)\n#5  0x00007f24feecd692 in __assert_fail\n    (assertion=0x7f24ff8c3328 \"box-&gt;x &gt;= 0 &amp;&amp; box-&gt;y &gt;= 0 &amp;&amp; box-&gt;x + box-&gt;width &lt;= options-&gt;texture-&gt;width &amp;&amp; box-&gt;y + box-&gt;height &lt;= options-&gt;texture-&gt;height\", file=0x7f24ff8c32e0 \"render/pass.c\", line=23, function=0x7f24ff8c3410 &lt;__PRETTY_FUNCTION__.1&gt; \"wlr_render_pass_add_texture\") at ./assert/assert.c:105\n#6  0x00007f24ff80765d in wlr_render_pass_add_texture (render_pass=0x55cff2179200, options=0x7ffe85840170) at ../subprojects/wlroots/render/pass.c:23\n#7  0x00007f24ff844e96 in scene_entry_render (entry=0x55cff2159cf8, data=0x7ffe85840440) at ../subprojects/wlroots/types/scene/wlr_scene.c:1270\n#8  0x00007f24ff846af9 in wlr_scene_output_build_state (scene_output=0x55cff1afb5d0, state=0x7ffe85840580, options=0x7ffe85840600) at ../subprojects/wlroots/types/scene/wlr_scene.c:1959\n#9  0x000055cfb3f91122 in output_repaint_timer_handler (data=0x55cff1aeabe0) at ../sway/desktop/output.c:285\n#10 0x000055cfb3f9146d in handle_frame (listener=0x55cff1aead30, user_data=0x55cff1b0d4b0) at ../sway/desktop/output.c:373\n#11 0x00007f24ff121b4c in wl_signal_emit_mutable () at /lib/x86_64-linux-gnu/libwayland-server.so.0\n#12 0x00007f24ff83e10d in wlr_output_send_frame (output=0x55cff1b0d4b0) at ../subprojects/wlroots/types/output/output.c:753\n#13 0x00007f24ff81e68b in handle_page_flip (fd=12, seq=488118, tv_sec=8164, tv_usec=155314, crtc_id=100, data=0x55cff21818d0) at ../subprojects/wlroots/backend/drm/drm.c:2100\n#14 0x00007f24ff65fc47 in drmHandleEvent () at /lib/x86_64-linux-gnu/libdrm.so.2\n#15 0x00007f24ff81e6de in handle_drm_event (fd=12, mask=1, data=0x55cff0a72000) at ../subprojects/wlroots/backend/drm/drm.c:2112\n#16 0x00007f24ff123cf2 in wl_event_loop_dispatch () at /lib/x86_64-linux-gnu/libwayland-server.so.0\n#17 0x00007f24ff121525 in wl_display_run () at /lib/x86_64-linux-gnu/libwayland-server.so.0\n#18 0x000055cfb3f8d4c7 in server_run (server=0x55cfb400dce0 &lt;server&gt;) at ../sway/server.c:501\n#19 0x000055cfb3f8bc33 in main (argc=1, argv=0x7ffe85841068) at ../sway/main.c:374\n\n(gdb) f 6\n...\n(gdb) p *box\n$1 = {x = 0, y = 0, width = 3820, height = 36}\n(gdb) p *options-&gt;texture\n$2 = {impl = 0x7f24ff908fe0 &lt;texture_impl&gt;, width = 374, height = 36, renderer = 0x55cff0c50f00}\n</code></pre>\n<p dir=\"auto\">(height matches, box width much larger than texture width).</p>\n<p dir=\"auto\">I have a 3840x2400 output with scale factor 2, but this doesn't obviously look like something scale-factor-related. My titlebars are (based on counting pixels in a screenshot) 52 pixels tall, but there's a bunch of padding around the titlebar text so if there's a texture not including that text a 36 pixel height for that seems plausible. (Also, <code class=\"notranslate\">options-&gt;dst_box</code> has x=10, y=8, and width and height same as <code class=\"notranslate\">src_box</code>: assuming that's relative to the output that'd fit with it being titlebar text).</p>\n<p dir=\"auto\">Applying the logging patch from <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"2329202097\" data-permission-text=\"Title is private\" data-url=\"https://github.com/swaywm/sway/issues/8194\" data-hovercard-type=\"issue\" data-hovercard-url=\"/swaywm/sway/issues/8194/hovercard?comment_id=2600936550&amp;comment_type=issue_comment\" href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600936550\">#8194 (comment)</a> (on top of the other wlroots patch, it looks like the added logging still makes sense), that logging does not trigger.</p>\n<p dir=\"auto\">Haven't wrapped my head around what all this code does yet...</p>\n<p dir=\"auto\">Looking at <code class=\"notranslate\">sway_text_node_set_max_width</code>, there's an <code class=\"notranslate\">update_source_box</code> call before the <code class=\"notranslate\">render_backing_buffer</code> call (<a href=\"https://github.com/swaywm/sway/blame/fde480b242aab59b36d427db81f4fd016cf0e0bd/sway/sway_text_node.c#L303\">here</a>). <code class=\"notranslate\">render_backing_buffer</code> contains its own <code class=\"notranslate\">update_source_box</code> call <a href=\"https://github.com/swaywm/sway/blame/fde480b242aab59b36d427db81f4fd016cf0e0bd/sway/sway_text_node.c#L150\">here</a>, which is paired with a <code class=\"notranslate\">wlr_scene_buffer_set_buffer</code> call. Judging by that previous logged error, we're exiting <code class=\"notranslate\">render_backing_buffer</code> early (<a href=\"https://github.com/swaywm/sway/blame/fde480b242aab59b36d427db81f4fd016cf0e0bd/sway/sway_text_node.c#L110\">here</a>).</p>\n<p dir=\"auto\">Again, don't know what this code does yet, but that seemed suspicious...<br>\nI tried commenting out the <code class=\"notranslate\">update_source_box</code> call before the <code class=\"notranslate\">render_backing_buffer</code> call (reasoning that as long as <code class=\"notranslate\">render_backing_buffer</code> doesn't fail it'll call <code class=\"notranslate\">update_source_box</code> again anyway and at first glance that'd plausibly be early enough) and now I'm no longer crashing but I get a very horizontally stretched titlebar: it's plausible the text is supposed to be 374 pixels (from the earlier coredump) wide, but it's stretched horizontally to fill the entire titlebar. I also saw a similarly stretched titlebar briefly when starting Chrome.</p>\n<p dir=\"auto\">So that's not the right fix but hopefully that narrows it down a bit further... (I'll look into this more if I find time)</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2708863364\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LCGJP7OY4NLALII66L2TRANHAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDOMBYHA3DGMZWGQ\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFS4MUOBFBMSDHH46L2TRANHA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVBOX4YI.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2708863364</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"marienz\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/516706?s=20&amp;v=4\" /><strong>marienz</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2708863364\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">I'm triggering what I'm currently assuming is the same crash by opening a print preview window in Chrome (using its wayland Ozone backend), using Sway 1.10.1 and wlroots 0.18.2 with the patch from <a href=\"https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981\" rel=\"nofollow\">https://gitlab.freedesktop.org/wlroots/wlroots/-/merge_requests/4981</a> applied. Same \"cairo_image_surface_create failed\", then:</p>\n<pre class=\"notranslate\"><code class=\"notranslate\">(gdb) bt\n(assert stuff omitted)\n#5  0x00007f24feecd692 in __assert_fail\n    (assertion=0x7f24ff8c3328 \"box-&gt;x &gt;= 0 &amp;&amp; box-&gt;y &gt;= 0 &amp;&amp; box-&gt;x + box-&gt;width &lt;= options-&gt;texture-&gt;width &amp;&amp; box-&gt;y + box-&gt;height &lt;= options-&gt;texture-&gt;height\", file=0x7f24ff8c32e0 \"render/pass.c\", line=23, function=0x7f24ff8c3410 &lt;__PRETTY_FUNCTION__.1&gt; \"wlr_render_pass_add_texture\") at ./assert/assert.c:105\n#6  0x00007f24ff80765d in wlr_render_pass_add_texture (render_pass=0x55cff2179200, options=0x7ffe85840170) at ../subprojects/wlroots/render/pass.c:23\n#7  0x00007f24ff844e96 in scene_entry_render (entry=0x55cff2159cf8, data=0x7ffe85840440) at ../subprojects/wlroots/types/scene/wlr_scene.c:1270\n#8  0x00007f24ff846af9 in wlr_scene_output_build_state (scene_output=0x55cff1afb5d0, state=0x7ffe85840580, options=0x7ffe85840600) at ../subprojects/wlroots/types/scene/wlr_scene.c:1959\n#9  0x000055cfb3f91122 in output_repaint_timer_handler (data=0x55cff1aeabe0) at ../sway/desktop/output.c:285\n#10 0x000055cfb3f9146d in handle_frame (listener=0x55cff1aead30, user_data=0x55cff1b0d4b0) at ../sway/desktop/output.c:373\n#11 0x00007f24ff121b4c in wl_signal_emit_mutable () at /lib/x86_64-linux-gnu/libwayland-server.so.0\n#12 0x00007f24ff83e10d in wlr_output_send_frame (output=0x55cff1b0d4b0) at ../subprojects/wlroots/types/output/output.c:753\n#13 0x00007f24ff81e68b in handle_page_flip (fd=12, seq=488118, tv_sec=8164, tv_usec=155314, crtc_id=100, data=0x55cff21818d0) at ../subprojects/wlroots/backend/drm/drm.c:2100\n#14 0x00007f24ff65fc47 in drmHandleEvent () at /lib/x86_64-linux-gnu/libdrm.so.2\n#15 0x00007f24ff81e6de in handle_drm_event (fd=12, mask=1, data=0x55cff0a72000) at ../subprojects/wlroots/backend/drm/drm.c:2112\n#16 0x00007f24ff123cf2 in wl_event_loop_dispatch () at /lib/x86_64-linux-gnu/libwayland-server.so.0\n#17 0x00007f24ff121525 in wl_display_run () at /lib/x86_64-linux-gnu/libwayland-server.so.0\n#18 0x000055cfb3f8d4c7 in server_run (server=0x55cfb400dce0 &lt;server&gt;) at ../sway/server.c:501\n#19 0x000055cfb3f8bc33 in main (argc=1, argv=0x7ffe85841068) at ../sway/main.c:374\n\n(gdb) f 6\n...\n(gdb) p *box\n$1 = {x = 0, y = 0, width = 3820, height = 36}\n(gdb) p *options-&gt;texture\n$2 = {impl = 0x7f24ff908fe0 &lt;texture_impl&gt;, width = 374, height = 36, renderer = 0x55cff0c50f00}\n</code></pre>\n<p dir=\"auto\">(height matches, box width much larger than texture width).</p>\n<p dir=\"auto\">I have a 3840x2400 output with scale factor 2, but this doesn't obviously look like something scale-factor-related. My titlebars are (based on counting pixels in a screenshot) 52 pixels tall, but there's a bunch of padding around the titlebar text so if there's a texture not including that text a 36 pixel height for that seems plausible. (Also, <code class=\"notranslate\">options-&gt;dst_box</code> has x=10, y=8, and width and height same as <code class=\"notranslate\">src_box</code>: assuming that's relative to the output that'd fit with it being titlebar text).</p>\n<p dir=\"auto\">Applying the logging patch from <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"2329202097\" data-permission-text=\"Title is private\" data-url=\"https://github.com/swaywm/sway/issues/8194\" data-hovercard-type=\"issue\" data-hovercard-url=\"/swaywm/sway/issues/8194/hovercard?comment_id=2600936550&amp;comment_type=issue_comment\" href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2600936550\">#8194 (comment)</a> (on top of the other wlroots patch, it looks like the added logging still makes sense), that logging does not trigger.</p>\n<p dir=\"auto\">Haven't wrapped my head around what all this code does yet...</p>\n<p dir=\"auto\">Looking at <code class=\"notranslate\">sway_text_node_set_max_width</code>, there's an <code class=\"notranslate\">update_source_box</code> call before the <code class=\"notranslate\">render_backing_buffer</code> call (<a href=\"https://github.com/swaywm/sway/blame/fde480b242aab59b36d427db81f4fd016cf0e0bd/sway/sway_text_node.c#L303\">here</a>). <code class=\"notranslate\">render_backing_buffer</code> contains its own <code class=\"notranslate\">update_source_box</code> call <a href=\"https://github.com/swaywm/sway/blame/fde480b242aab59b36d427db81f4fd016cf0e0bd/sway/sway_text_node.c#L150\">here</a>, which is paired with a <code class=\"notranslate\">wlr_scene_buffer_set_buffer</code> call. Judging by that previous logged error, we're exiting <code class=\"notranslate\">render_backing_buffer</code> early (<a href=\"https://github.com/swaywm/sway/blame/fde480b242aab59b36d427db81f4fd016cf0e0bd/sway/sway_text_node.c#L110\">here</a>).</p>\n<p dir=\"auto\">Again, don't know what this code does yet, but that seemed suspicious...<br>\nI tried commenting out the <code class=\"notranslate\">update_source_box</code> call before the <code class=\"notranslate\">render_backing_buffer</code> call (reasoning that as long as <code class=\"notranslate\">render_backing_buffer</code> doesn't fail it'll call <code class=\"notranslate\">update_source_box</code> again anyway and at first glance that'd plausibly be early enough) and now I'm no longer crashing but I get a very horizontally stretched titlebar: it's plausible the text is supposed to be 374 pixels (from the earlier coredump) wide, but it's stretched horizontally to fill the entire titlebar. I also saw a similarly stretched titlebar briefly when starting Chrome.</p>\n<p dir=\"auto\">So that's not the right fix but hopefully that narrows it down a bit further... (I'll look into this more if I find time)</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2708863364\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LCGJP7OY4NLALII66L2TRANHAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDOMBYHA3DGMZWGQ\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LFS4MUOBFBMSDHH46L2TRANHA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVBOX4YI.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2708863364</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2708863364\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2708863364\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"marienz\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 09 Mar 2025 06:42:43 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742813852.1726233_1238.icarus,U=1238:2,T"
            ],
            "timestamp": 1741528522,
            "date_relative": "March 09",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">The PR I linked above has been merged to master and removes the use of a source box from <code class=\"notranslate\">sway_text_node</code> entirely.</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2708868609\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFL67XVCWONVDPSZLD2TRB4VAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDOMBYHA3DQNRQHE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGXQIJSLUAULMXDKL32TRB4VA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVBOYHAC.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2708868609</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"kennylevinsen\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/176245?s=20&amp;v=4\" /><strong>kennylevinsen</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2708868609\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">The PR I linked above has been merged to master and removes the use of a source box from <code class=\"notranslate\">sway_text_node</code> entirely.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2708868609\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LFL67XVCWONVDPSZLD2TRB4VAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDOMBYHA3DQNRQHE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LGXQIJSLUAULMXDKL32TRB4VA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVBOYHAC.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2708868609</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2708868609\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2708868609\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Kenny Levinsen\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 09 Mar 2025 06:55:22 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1742813852.1726233_1187.icarus,U=1187:2,T"
            ],
            "timestamp": 1741563430,
            "date_relative": "March 10",
            "tags": [],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<span style=\"color: transparent; display: none; height: 0; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; visibility: hidden; width: 0;\">\n  <p dir=\"auto\">Ah, whoops, I'd missed the significance of that PR. Applying those two commits to 1.10 fixes the crash for me (without stretched titlebar text this time).</p><p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2709132692\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LG2DHE7UJW6WIDRUET2TTGCNAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDOMBZGEZTENRZGI\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LESECW7TZLWKYU5X6T2TTGCNA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVBPIKZI.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2709132692</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n</span>\n\n\n<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img alt=\"marienz\" height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/516706?s=20&amp;v=4\" /><strong>marienz</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2709132692\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">Ah, whoops, I'd missed the significance of that PR. Applying those two commits to 1.10 fixes the crash for me (without stretched titlebar text this time).</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2709132692\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LG2DHE7UJW6WIDRUET2TTGCNAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDOMBZGEZTENRZGI\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LESECW7TZLWKYU5X6T2TTGCNA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVBPIKZI.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2709132692</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2709132692\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2709132692\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"marienz\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 09 Mar 2025 16:37:10 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/new/1748767535.465903_1.icarus,U=183143:2,"
            ],
            "timestamp": 1748767225,
            "date_relative": "55 mins. ago",
            "tags": [
              "Mailinglist",
              "inbox",
              "unread"
            ],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/22935812?s=20&amp;v=4\" /><strong>itsTurnip</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2926827404\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">I've met the same crash error while using Telegram after their 5.14.* update. This happens after some time of regular using app, and occasionally in some day it starts to crash sway session whenever I try to open it (I couldn't find the cause of this event).</p>\n<p dir=\"auto\">I uploaded my sway debug log to <a href=\"https://gist.github.com/itsTurnip/cefc0d6455730adbe0ef9aca83bbbc52\">gist</a>, also there's <a href=\"https://gist.github.com/itsTurnip/2d4fbcd2f67f33d06675a86bcefb3a81#file-twayland-debug-log\">WAYLAND_DEBUG</a>.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2926827404\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LGD7TUR7JDYUOQRUO33BK37TAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDSMRWHAZDONBQGQ\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LG5DC4ZDC6MGWBXAA33BK37TA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVOOPLYY.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2926827404</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2926827404\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2926827404\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"itsTurnip\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 01 Jun 2025 01:40:25 -0700"
            }
          },
          []
        ],
        [
          {
            "id": "swaywm/sway/issues/8194/user@example.com",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/new/1748767835.466332_1.icarus,U=183144:2,"
            ],
            "timestamp": 1748767608,
            "date_relative": "49 mins. ago",
            "tags": [
              "Mailinglist",
              "inbox",
              "unread"
            ],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<div style=\"display: flex; flex-wrap: wrap; white-space: pre-wrap; align-items: center; \"><img height=\"20\" width=\"20\" style=\"border-radius:50%; margin-right: 4px;\" decoding=\"async\" src=\"https://avatars.githubusercontent.com/u/176245?s=20&amp;v=4\" /><strong>kennylevinsen</strong> left a comment <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2926834461\">(swaywm/sway#8194)</a></div>\n<p dir=\"auto\">Should be fixed by the sway release candidate.</p>\n\n<p style=\"font-size:small;-webkit-text-size-adjust:none;color:#666;\">&mdash;<br />Reply to this email directly, <a href=\"https://github.com/swaywm/sway/issues/8194#issuecomment-2926834461\">view it on GitHub</a>, or <a href=\"https://github.com/notifications/unsubscribe-auth/AAAG5LH3XXIPTRSPI2AWSED3BK4XRAVCNFSM6AAAAABIUHYHR2VHI2DSMVQWIX3LMV43OSLTON2WKQ3PNVWWK3TUHMZDSMRWHAZTINBWGE\">unsubscribe</a>.<br />You are receiving this because you are subscribed to this thread.<img src=\"https://github.com/notifications/beacon/AAAG5LEXAVMJXRH3SJEDBNT3BK4XRA5CNFSM6AAAAABIUHYHR2WGG33NNVSW45C7OR4XAZNMJFZXG5LFINXW23LFNZ2KUY3PNVWWK3TUL5UWJTVOOPZR2.gif\" height=\"1\" width=\"1\" alt=\"\" /><span style=\"color: transparent; font-size: 0; display: none; visibility: hidden; overflow: hidden; opacity: 0; width: 0; height: 0; max-width: 0; max-height: 0; mso-hide: all\">Message ID: <span>&lt;swaywm/sway/issues/8194/2926834461</span><span>@</span><span>github</span><span>.</span><span>com&gt;</span></span></p>\n\n<script type=\"application/ld+json\">[\n{\n\"@context\": \"http://schema.org\",\n\"@type\": \"EmailMessage\",\n\"potentialAction\": {\n\"@type\": \"ViewAction\",\n\"target\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2926834461\",\n\"url\": \"https://github.com/swaywm/sway/issues/8194#issuecomment-2926834461\",\n\"name\": \"View Issue\"\n},\n\"description\": \"View this Issue on GitHub\",\n\"publisher\": {\n\"@type\": \"Organization\",\n\"name\": \"GitHub\",\n\"url\": \"https://github.com\"\n}\n}\n]</script>\n"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: [swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
              "From": "\"Kenny Levinsen\" <user@example.com>",
              "To": "swaywm/sway <user@example.com>",
              "Cc": "Subscribed <user@example.com>",
              "Reply-To": "swaywm/sway <user@example.com>",
              "Date": "Sun, 01 Jun 2025 01:46:48 -0700"
            }
          },
          []
        ]
      ]
    ]
  ]
]

```
