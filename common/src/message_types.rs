use serde::{Deserialize, Serialize};

/// Types of messages passed through websockets
#[repr(u8)]
#[serde(untagged)]
#[derive(
	Serialize, Deserialize, FromPrimitive, Copy, Clone, Eq, PartialEq, Debug,
)]
pub enum MessageType {
	/// Initial handshake with server
	Handshake = 0,

	/// Request and response to synchronize with a thread or thread index
	Synchronize,

	/// Request to create a new thread or new thread creation event
	InsertThread,

	/// Acknowledgment of thread creation. Response to InsertThread from server.
	InsertThreadAck,

	/// Request to create a new post or post creation event
	InsertPost,

	/// Acknowledgment of thread creation. Response to InsertPost from server.
	InsertPostAck,

	/// Image inserted into an open post
	InsertImage,

	/// Submit captcha or pass result captcha authentication
	Captcha,

	/// Notify client a captcha is required for a previously attempted request
	NeedCaptcha,

	/// Apply a patch to an existing post body
	PatchPostBody,

	/// Append string to the end of the post body
	Append,

	/// Shorten post body by one Unicode character
	Backspace,

	/// Send server's current Unix timestamp
	CurrentTime,

	/// Request or send a page of a thread
	Page,
}
