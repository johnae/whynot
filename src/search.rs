use serde::{Deserialize, Serialize};

/// Container for notmuch search results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult(pub Vec<SearchItem>);

/// Individual search result from notmuch search command
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchItem {
    pub thread: String,
    pub timestamp: i64,
    pub date_relative: String,
    pub matched: u32,
    pub total: u32,
    pub authors: String,
    pub subject: String,
    pub query: Vec<Option<String>>,
    pub tags: Vec<String>,
}

impl SearchItem {
    /// Get the thread ID for this search result
    pub fn thread_id(&self) -> &str {
        &self.thread
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_search_result() {
        let json_data = r#"[
  {
    "thread": "00000000000276db",
    "timestamp": 1748767608,
    "date_relative": "46 mins. ago",
    "matched": 2,
    "total": 30,
    "authors": "itsTurnip, Kenny Levinsen| Hugo, Alexander Orzechowski, Simon Ser, Okami, Andre Esteve, minus, Vuk Mirovic, Kirill Primak, Conner Bondurant, Olivier Nicole, Daven Du, Isaac Freund, Dan Klishch, Stanislau T., marienz",
    "subject": "[swaywm/sway] sway crashes when interacting with chromium (Issue #8194)",
    "query": [
      "id:swaywm/sway/issues/8194/2926827404@github.com id:swaywm/sway/issues/8194/2926834461@github.com",
      "id:swaywm/sway/issues/8194@github.com id:swaywm/sway/issues/8194/2143507637@github.com id:swaywm/sway/issues/8194/2143860640@github.com id:swaywm/sway/issues/8194/2145753343@github.com id:swaywm/sway/issues/8194/2185787904@github.com id:swaywm/sway/issues/8194/2245531481@github.com id:swaywm/sway/issues/8194/2474374205@github.com id:swaywm/sway/issues/8194/2479758512@github.com id:swaywm/sway/issues/8194/2575424050@github.com id:swaywm/sway/issues/8194/2600921675@github.com id:swaywm/sway/issues/8194/2600925559@github.com id:swaywm/sway/issues/8194/2600930921@github.com id:swaywm/sway/issues/8194/2600932430@github.com id:swaywm/sway/issues/8194/2600936550@github.com id:swaywm/sway/issues/8194/2600943530@github.com id:swaywm/sway/issues/8194/2605306751@github.com id:swaywm/sway/issues/8194/2610422238@github.com id:swaywm/sway/issues/8194/2610956778@github.com id:swaywm/sway/issues/8194/2615471585@github.com id:swaywm/sway/issues/8194/2659926629@github.com id:swaywm/sway/issues/8194/2665935361@github.com id:swaywm/sway/issues/8194/2667333795@github.com id:swaywm/sway/issues/8194/2668100846@github.com id:swaywm/sway/issues/8194/2681420414@github.com id:swaywm/sway/issues/8194/2695458972@github.com id:swaywm/sway/issues/8194/2708863364@github.com id:swaywm/sway/issues/8194/2708868609@github.com id:swaywm/sway/issues/8194/2709132692@github.com"
    ],
    "tags": [
      "Mailinglist",
      "inbox",
      "unread"
    ]
  }
]"#;

        let result: SearchResult = serde_json::from_str(json_data).unwrap();

        assert_eq!(result.0.len(), 1);

        let item = &result.0[0];
        assert_eq!(item.thread, "00000000000276db");
        assert_eq!(item.timestamp, 1748767608);
        assert_eq!(item.date_relative, "46 mins. ago");
        assert_eq!(item.matched, 2);
        assert_eq!(item.total, 30);
        assert_eq!(
            item.authors,
            "itsTurnip, Kenny Levinsen| Hugo, Alexander Orzechowski, Simon Ser, Okami, Andre Esteve, minus, Vuk Mirovic, Kirill Primak, Conner Bondurant, Olivier Nicole, Daven Du, Isaac Freund, Dan Klishch, Stanislau T., marienz"
        );
        assert_eq!(
            item.subject,
            "[swaywm/sway] sway crashes when interacting with chromium (Issue #8194)"
        );
        assert_eq!(item.query.len(), 2);
        assert!(item.query[0].is_some());
        assert!(item.query[1].is_some());
        assert_eq!(item.tags.len(), 3);
        assert_eq!(item.tags[0], "Mailinglist");
        assert_eq!(item.tags[1], "inbox");
        assert_eq!(item.tags[2], "unread");
    }

    #[test]
    fn test_deserialize_search_result_with_null_query() {
        let json_data = r#"[
  {
    "thread": "00000000000306d5",
    "timestamp": 1748731534,
    "date_relative": "Today 00:45",
    "matched": 2,
    "total": 2,
    "authors": "Personal AI, alice@techcorp.example",
    "subject": "Din faktura från Personal AI #MSTRL-API-662120-004",
    "query": [
      "id:0102019728759228-ce2f31cb-b971-417c-a2ed-3d14bbc9ba8f-000000@eu-west-1.amazonses.com id:4116988b3d7411787763bd0659379036@techcorp.example",
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
]"#;

        let result: SearchResult = serde_json::from_str(json_data).unwrap();

        assert_eq!(result.0.len(), 1);

        let item = &result.0[0];
        assert_eq!(item.query.len(), 2);
        assert!(item.query[0].is_some());
        assert!(item.query[1].is_none());
        assert_eq!(item.tags.len(), 5);
        assert!(item.tags.contains(&"attachment".to_string()));
    }

    #[test]
    fn test_thread_id_helper() {
        let item = SearchItem {
            thread: "00000000000276db".to_string(),
            timestamp: 1748767608,
            date_relative: "46 mins. ago".to_string(),
            matched: 2,
            total: 30,
            authors: "Test Author".to_string(),
            subject: "Test Subject".to_string(),
            query: vec![Some("query1".to_string())],
            tags: vec!["inbox".to_string()],
        };

        assert_eq!(item.thread_id(), "00000000000276db");
    }
}
