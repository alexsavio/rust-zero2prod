use crate::helpers::{spawn_app, ConfirmationLinks, TestApp};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        // we assert that no request is made to the email server
        .expect(0)
        .mount(&app.email_server)
        .await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "newsletter title",
        "content": {
            "html": "<p>Newsletter body as HTML.</p>",
            "text": "Newsletter body as plain text.",
        },
    });
    let response = app.post_newsletters(newsletter_request_body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
    // Mock verifies on Drop that we haven't sent the newsletter email
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as HTML</p>",
        }
    });
    let response = app.post_newsletters(newsletter_request_body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
    // Mock verifies on Drop that we have sent the newsletter email
}

#[tokio::test]
async fn publish_newsletter_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "content": {
                    "text": "Newsletter body as plain text",
                    "html": "<p>Newsletter body as HTML</p>",
                }
            }),
            "missing field `title`",
        ),
        (
            serde_json::json!({
                "title": "Newsletter title",
            }),
            "missing field `content`",
        ),
        (serde_json::json!({}), "missing field `title`"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_newsletters(invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

/// Use the public API of the application under test to create
/// an unconfirmed subscriber.
async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;
    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();
    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await.html;
    let client = reqwest::Client::new();
    client
        .post(confirmation_link)
        .send()
        .await
        .expect("Failed to send POST request to confirmation link.")
        .error_for_status()
        .expect("Confirmation request failed.");
}
