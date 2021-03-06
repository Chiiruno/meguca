use super::{Feed, FeedCommon};
use crate::{message::Message, registry::Registry, util::MessageCacher};
use actix::prelude::*;
use common::payloads::{post_body::Node, ThreadWithPosts};
use std::{collections::HashMap, sync::Arc};

/// Change to be applied to thread data
#[derive(Debug)]
pub enum Change {
	InsertThread(ThreadWithPosts),
	InsertPost(super::InsertPost),
	SetBody { id: u64, body: Arc<Node> },
}

/// Set of buffered changes for a particular thread
// TODO: handle this
#[derive(Debug)]
pub struct ChangeSet {
	/// Source feed of the change
	pub source_feed: u64,

	/// Concatenated messages to be sent to clients
	pub messages: Message,

	/// Unencoded contents of the messages
	pub changes: Vec<Change>,
}

/// Feed for the thread index
#[derive(Debug)]
pub struct IndexFeed {
	common: FeedCommon<Self>,
	threads: HashMap<u64, MessageCacher<ThreadWithPosts>>,
	changes: Vec<ChangeSet>,
}

crate::impl_feed_commons! {IndexFeed}

impl Feed for IndexFeed {
	fn get_feed_common(&mut self) -> &mut FeedCommon<Self> {
		&mut self.common
	}

	fn process_pulse(&mut self) {
		todo!()
	}
}

impl Handler<super::InsertThread> for IndexFeed {
	type Result = ();

	/// This method is called for every message received by this actor.
	fn handle(
		&mut self,
		msg: super::InsertThread,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use common::{
			payloads::{Post, ReplyCreationOpts, Thread},
			Encoder, MessageType,
		};

		self.common.schedule_pulse(ctx);

		let now = crate::util::now();
		let thread = ThreadWithPosts {
			thread_data: Thread::new(msg.id, now, msg.subject, msg.tags),
			posts: vec![Post::new(
				msg.id,
				msg.id,
				0,
				now,
				ReplyCreationOpts {
					sage: false,
					post_opts: msg.opts,
				},
			)],
		};
		let payload = match Encoder::encode(MessageType::InsertThread, &thread)
		{
			Ok(p) => p,
			Err(e) => {
				self.common.log_encode_error(0, e);
				return;
			}
		};
		self.changes.push(ChangeSet {
			source_feed: msg.id,
			messages: payload.into(),
			changes: vec![Change::InsertThread(thread)],
		});
	}
}

impl IndexFeed {
	pub fn new(
		threads: Vec<ThreadWithPosts>,
		registry: Addr<Registry>,
	) -> Self {
		Self {
			common: FeedCommon::new(registry),
			threads: threads
				.into_iter()
				.map(|t| (t.thread_data.id, t.into()))
				.collect(),
			changes: Default::default(),
		}
	}
}
