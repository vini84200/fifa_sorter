use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub enum Query {
    Player(String),
    User(u32),
    Top(i32, String),
    Tags(Vec<String>),
}

pub fn parse_query(query: &str) -> Result<Query> {
    let mut query = query.split_whitespace();
    match query.next() {
        Some("player") => {
            // Add whitespaces between words
            let name: String = query.collect::<Vec<&str>>().join(" ");
            Ok(Query::Player(name))
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
            let mut tags = Vec::new();
            for tag in query {
                let tag = tag.trim_matches('\'');
                tags.push(tag.to_string());
            }
            Ok(Query::Tags(tags))
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
}