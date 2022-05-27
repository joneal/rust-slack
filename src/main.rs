#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod slack;

static HOOK_URL: &str = "https://slack.com/api/chat.postMessage";
static OAUTH_TOKEN: &str = "****-**********-*************-************************";
static CHANNEL_ID: &str = "CXXXXXXXX";
static USERNAME: &str = "Bleh";
static ICON_EMOJI: &str = ":fire:";


fn main() {

    let mut ctx = slack::SlackContext::new(HOOK_URL.to_owned(), OAUTH_TOKEN.to_owned(), CHANNEL_ID.to_owned(), USERNAME.to_owned(), ICON_EMOJI.to_owned());

    ctx.message = "This is a test".to_owned();

    let _res = slack::post_message(ctx);
}

