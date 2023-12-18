use crate::release::get_release;
use inquire::{
    error::InquireResult,
    ui::{Color, RenderConfig, Styled},
    Editor,
};

/// Queries the changelog from the release.
async fn get_changelog(version: &str) -> String {
    let res = get_release(version).await;
    match res {
        Ok(release) => {
            let body = release.body;
            match body {
                Some(body) => {
                    println!("body: {}", body);
                    body
                }
                None => String::from(""),
            }
        }
        Err(_) => String::from(""),
    }
}

/// Lets the user adjust the features section of the proposal.
pub async fn adjust_features() -> InquireResult<String> {
    let changelog_body = get_changelog("v14.0.0").await;

    let _description = Editor::new("Description:")
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                String::from("<skipped>")
            } else if char_count <= 20 {
                submission.into()
            } else {
                let mut substr: String = submission.chars().take(17).collect();
                substr.push_str("...");
                substr
            }
        })
        .with_render_config(description_render_config())
        .prompt()?;

    Ok(changelog_body)
}

/// Describes the rendering configuration for the description prompt.
fn description_render_config() -> RenderConfig {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_changelog_pass() {
        let changelog = get_changelog("v14.0.0").await;
        assert_ne!(
            changelog.as_str(),
            "",
            "expected a changelog body to be returned"
        );
    }

    #[tokio::test]
    async fn test_get_changelog_fail() {
        let changelog = get_changelog("v14.0.8").await;
        assert_eq!(
            changelog.as_str(),
            "",
            "expected no changelog body to be returned"
        );
    }
}
