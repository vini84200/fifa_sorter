use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum Query {
    Player(String),
    User(u32),
    Top(i32, String),
    Tags(Vec<String>),
}

fn parse_query(query: &str) -> Result<Query> {
    let mut query = query.split_whitespace();
    match query.next() {
        Some("player") => {
            // Add whitespaces between words
            let name: String = query.collect::<Vec<&str>>().join(" ");
            if name.trim().is_empty() {
                Err(anyhow!("Invalid query"))
            } else {
                Ok(Query::Player(name))
            }
        }
        Some("user") => {
            match query.next() {
                Some(user) => {
                    match user.parse::<u32>() {
                        Ok(user) => Ok(Query::User(user)),
                        Err(_) => anyhow::bail!("Invalid query")
                    }
                }
                None => anyhow::bail!("Invalid query")
            }
        }
        Some("tags") => {
            // Format: tags 'tag1' 'tag2' 'tag3 that is long'
            let tags_collected: String = query.collect::<Vec<&str>>().join(" ");
            if tags_collected.trim().is_empty() {
                Err(anyhow!("Invalid query"))
            } else {
                let query = tags_collected.split('\'').collect::<Vec<&str>>();
                let mut tags = Vec::new();
                for (i, tag) in query.iter().enumerate() {
                    if i % 2 == 1 {
                        tags.push(tag.to_string());
                    } else if !tag.trim().is_empty() {
                        anyhow::bail!("Invalid query");
                    }
                }
                Ok(Query::Tags(tags))
            }

        }
        Some(prompt) => {
            if prompt.starts_with("top") {
                // top10 'ST'
                let prompt = prompt.strip_prefix("top");
                if let Some(prompt) = prompt {
                    let i = prompt.parse::<i32>()?;
                    let pos = query.next().ok_or_else(|| anyhow::anyhow!("Invalid query"))?;
                    //remove the '
                    let pos = pos.strip_prefix("'").ok_or_else(|| anyhow::anyhow!("Invalid query"))?;
                    let pos = pos.strip_suffix("'").ok_or_else(|| anyhow::anyhow!("Invalid query"))?;

                    Ok(Query::Top(i, pos.to_string()))
                } else {
                    Err(anyhow::anyhow!("Invalid query"))
                }
            } else {
                Err(anyhow::anyhow!("Invalid query"))
            }
        }
        None => Err(anyhow::anyhow!("Invalid query"))
    }
}

impl TryFrom<String> for Query {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        parse_query(&value)
    }
}

impl TryFrom<&str> for Query {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_query(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query() {
        let query = "player Cristiano Ronaldo";
        let query = parse_query(query).unwrap();
        assert_eq!(query, Query::Player("Cristiano Ronaldo".to_string()));

        let query = "user 123";
        let query = parse_query(query).unwrap();
        assert_eq!(query, Query::User(123));

        let query = "tags 'ST' 'CF'";
        let query = parse_query(query).unwrap();
        assert_eq!(query, Query::Tags(vec!["ST".to_string(), "CF".to_string()]));

        let query = "top10 'ST'";
        let query = parse_query(query).unwrap();
        assert_eq!(query, Query::Top(10, "ST".to_string()));
    }

    #[test]
    fn test_try_from() {
        let query = "player Cristiano Ronaldo";
        let query = Query::try_from(query).unwrap();
        assert_eq!(query, Query::Player("Cristiano Ronaldo".to_string()));

        let query = "user 123";
        let query = Query::try_from(query).unwrap();
        assert_eq!(query, Query::User(123));

        let query = "tags 'ST' 'CF'";
        let query = Query::try_from(query).unwrap();
        assert_eq!(query, Query::Tags(vec!["ST".to_string(), "CF".to_string()]));

        let query = "top10 'ST'";
        let query = Query::try_from(query).unwrap();
        assert_eq!(query, Query::Top(10, "ST".to_string()));
    }

    #[test]
    fn test_wrong_query() {
        let query = "player";
        let query = parse_query(query);
        assert!(query.is_err());

        let query = "user";
        let query = parse_query(query);
        assert!(query.is_err());

        let query = "tags";
        let query = parse_query(query);
        assert!(query.is_err());

        let query = "tags ";
        let query = parse_query(query);
        assert!(query.is_err());

        let query = "top";
        let query = parse_query(query);
        assert!(query.is_err());

        let query = "top0";
        let query = parse_query(query);
        assert!(query.is_err());

        let query = "top9";
        let query = parse_query(query);
        assert!(query.is_err());

        let query = "top 'ST'";
        let query = parse_query(query);
        assert!(query.is_err());
    }
}