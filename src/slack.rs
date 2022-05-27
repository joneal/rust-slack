
    pub fn post_message(ctx: SlackContext) -> Result<(), Box<dyn std::error::Error>> {

        let payload = SlackPayload::new(ctx.channel_id, ctx.username, ctx.icon_emoji, ctx.message);

        let auth_header = &("Bearer ".to_owned() + ctx.oauth_token.as_str());

        let _resp: String = ureq::post(ctx.hook_url.as_str())
            .set("Content-Type","application/json")
            .set("Authorization", auth_header)
            .send_json(payload)?
            .into_string()?;
            Ok(())
    }

    // The SlackContext is built by the calling client.  It takes things like the Slack channel and associated URL, as well 
    // as the message to send to the channel.  
    pub struct SlackContext {
        hook_url: String,
        oauth_token: String,
        channel_id: String,
        username: String,
        icon_emoji: String,
        pub message: String
    }

    impl SlackContext {
        pub fn new(hook_url: String, oauth_token: String, channel_id: String, username: String, icon_emoji: String) -> Self {
            Self {
                hook_url: hook_url,
                oauth_token: oauth_token,
                channel_id: channel_id,
                username: username,
                icon_emoji: icon_emoji,
                message: "".to_owned()
            }
        }
    }

    // The SlackPayload is the data that must be sent to the Slack URL as JSON data in the POST payload.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct SlackPayload {
        channel: String,
        username: String,
        icon_emoji: String,
        pub text: String
    }

    impl SlackPayload {
        pub fn new(channel_id: String, username: String, icon_emoji: String, message: String) -> Self {
            Self {
                channel: channel_id,
                username: username,
                icon_emoji: icon_emoji,
                text: message
            }
        }
    }
