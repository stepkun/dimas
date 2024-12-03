// Copyright © 2024 Stephan Kunz

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use anyhow::Result;
use bitcode::encode;
use core::time::Duration;
use dimas_core::{
	enums::TaskSignal,
	message_types::{Message, ObservableControlResponse, ObservableResponse},
	traits::Context,
	utils::feedback_selector_from,
	OperationState, Operational, Transitions,
};
use futures::future::BoxFuture;
#[cfg(feature = "std")]
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::{error, event, info, instrument, warn, Level};
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;
use zenoh::Wait;
use zenoh::{
	qos::{CongestionControl, Priority},
	Session,
};
// endregion:	--- modules

// region:    	--- types
/// Type definition for an observables `control` callback
pub type ControlCallback<P> = Box<
	dyn FnMut(Context<P>, Message) -> BoxFuture<'static, Result<ObservableControlResponse>>
		+ Send
		+ Sync,
>;
/// Type definition for an observables atomic reference counted `control` callback
pub type ArcControlCallback<P> = Arc<Mutex<ControlCallback<P>>>;
/// Type definition for an observables `feedback` callback
pub type FeedbackCallback<P> =
	Box<dyn FnMut(Context<P>) -> BoxFuture<'static, Result<Message>> + Send + Sync>;
/// Type definition for an observables atomic reference counted `feedback` callback
pub type ArcFeedbackCallback<P> = Arc<Mutex<FeedbackCallback<P>>>;
/// Type definition for an observables atomic reference counted `execution` callback
pub type ExecutionCallback<P> =
	Box<dyn FnMut(Context<P>) -> BoxFuture<'static, Result<Message>> + Send + Sync>;
/// Type definition for an observables atomic reference counted `execution` callback
pub type ArcExecutionCallback<P> = Arc<Mutex<ExecutionCallback<P>>>;
// endregion: 	--- types

// region:		--- Observable
/// Observable
pub struct Observable<P>
where
	P: Send + Sync + 'static,
{
	/// The current state for [`Operational`]
	current_state: OperationState,
	/// the zenoh session this observable belongs to
	session: Arc<Session>,
	/// The observables key expression
	selector: String,
	/// Context for the Observable
	context: Context<P>,
	activation_state: OperationState,
	feedback_interval: Duration,
	/// callback for observation request and cancelation
	control_callback: ArcControlCallback<P>,
	/// callback for observation feedback
	feedback_callback: ArcFeedbackCallback<P>,
	feedback_publisher: Arc<Mutex<Option<zenoh::pubsub::Publisher<'static>>>>,
	/// function for observation execution
	execution_function: ArcExecutionCallback<P>,
	execution_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
	handle: parking_lot::Mutex<Option<JoinHandle<()>>>,
}

impl<P> core::fmt::Debug for Observable<P>
where
	P: Send + Sync + 'static,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Observable")
			.finish_non_exhaustive()
	}
}

impl<P> crate::traits::Responder for Observable<P>
where
	P: Send + Sync + 'static,
{
	/// Get `selector`
	fn selector(&self) -> &str {
		&self.selector
	}
}

impl<P> Transitions for Observable<P>
where
	P: Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "activate");
		let selector = self.selector.clone();
		let interval = self.feedback_interval;
		let ccb = self.control_callback.clone();
		let fcb = self.feedback_callback.clone();
		let fcbp = self.feedback_publisher.clone();
		let efc = self.execution_function.clone();
		let efch = self.execution_handle.clone();
		let ctx1 = self.context.clone();
		let ctx2 = self.context.clone();
		let session = self.session.clone();

		self.handle
			.lock()
			.replace(tokio::task::spawn(async move {
				let key = selector.clone();
				std::panic::set_hook(Box::new(move |reason| {
					error!("observable panic: {}", reason);
					if let Err(reason) = ctx1
						.sender()
						.blocking_send(TaskSignal::RestartObservable(key.clone()))
					{
						error!("could not restart observable: {}", reason);
					} else {
						info!("restarting observable!");
					};
				}));
				if let Err(error) =
					run_observable(session, selector, interval, ccb, fcb, fcbp, efc, efch, ctx2)
						.await
				{
					error!("observable failed with {error}");
				};
			}));

		Ok(())
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn deactivate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "deactivate");
		let handle = self.handle.lock().take();
		if let Some(handle) = handle {
			let feedback_publisher = self.feedback_publisher.clone();
			let feedback_callback = self.feedback_callback.clone();
			let execution_handle = self.execution_handle.clone();
			let ctx = self.context.clone();
			tokio::spawn(async move {
				// stop execution if running
				let handle = execution_handle.lock().await.take();
				if let Some(execution_handle) = handle {
					execution_handle.abort();
					// send back cancelation message
					let handle = feedback_publisher.lock().await.take();
					if let Some(publisher) = handle {
						let Ok(msg) = feedback_callback.lock().await(ctx).await else {
							// @TODO maybe implement retry!!
							error!("could not send feedback");
							return;
						};
						let response = ObservableResponse::Canceled(msg.value().clone());
						match publisher
							.put(Message::encode(&response).value().clone())
							.wait()
						{
							Ok(()) => {}
							Err(err) => error!("could not send cancel state due to {err}"),
						};
					};
				};
			});
			handle.abort();
		}
		Ok(())
	}
}

impl<P> Operational for Observable<P>
where
	P: Send + Sync + 'static,
{
	fn activation_state(&self) -> OperationState {
		self.activation_state
	}

	fn desired_state(&self, _state: OperationState) -> OperationState {
		todo!()
	}

	fn state(&self) -> OperationState {
		self.current_state
	}

	fn set_state(&mut self, state: OperationState) {
		self.current_state = state;
	}

	fn set_activation_state(&mut self, _state: OperationState) {
		todo!()
	}
}

impl<P> Observable<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for an [`Observable`]
	#[allow(clippy::too_many_arguments)]
	#[must_use]
	pub fn new(
		session: Arc<Session>,
		selector: impl Into<String>,
		context: Context<P>,
		activation_state: OperationState,
		feedback_interval: Duration,
		control_callback: ArcControlCallback<P>,
		feedback_callback: ArcFeedbackCallback<P>,
		execution_function: ArcExecutionCallback<P>,
	) -> Self {
		Self {
			current_state: OperationState::default(),
			session,
			selector: selector.into(),
			context,
			activation_state,
			feedback_interval,
			control_callback,
			feedback_callback,
			feedback_publisher: Arc::new(Mutex::new(None)),
			execution_function,
			execution_handle: Arc::new(Mutex::new(None)),
			handle: parking_lot::Mutex::new(None),
		}
	}
}
// endregion:	--- Observable

// region:		--- functions
#[allow(clippy::significant_drop_tightening)]
#[allow(clippy::too_many_arguments)]
#[instrument(name="observable", level = Level::ERROR, skip_all)]
async fn run_observable<P>(
	session: Arc<Session>,
	selector: String,
	feedback_interval: Duration,
	control_callback: ArcControlCallback<P>,
	feedback_callback: ArcFeedbackCallback<P>,
	feedback_publisher: Arc<Mutex<Option<zenoh::pubsub::Publisher<'static>>>>,
	execution_function: ArcExecutionCallback<P>,
	execution_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
	ctx: Context<P>,
) -> core::result::Result<(), Box<dyn core::error::Error + Send + Sync + 'static>>
where
	P: Send + Sync + 'static,
{
	// create the control queryable
	let builder = session
		.declare_queryable(&selector)
		.complete(true);

	#[cfg(feature = "unstable")]
	let builder = builder.allowed_origin(Locality::Any);

	let queryable = builder.await?;

	// initialize a pinned feedback timer
	// TODO: init here leads to on unnecessary timer-cycle without doing something
	let feedback_timer = tokio::time::sleep(feedback_interval);
	tokio::pin!(feedback_timer);

	// base communication key & selector for feedback publisher
	let key = selector.clone();
	let publisher_selector = feedback_selector_from(&key, &session.zid().to_string());

	// variables to manage control loop
	let mut is_running = false;
	let (tx, mut rx) = tokio::sync::mpsc::channel(8);

	// main control loop of the observable
	// started and terminated by state management
	// do not terminate loop in case of errors during execution
	loop {
		let ctx = ctx.clone();
		// different cases that may happen
		tokio::select! {
			// got query from an observer
			Ok(query) = queryable.recv_async() => {
				// TODO: make a proper "key: value" implementation
				let p = query.parameters().as_str();
				if p == "request" {
					// received request => if no execution is running: spawn execution with channel for result else: return already running message
					if is_running {
						// send occupied response
						let key = query.selector().key_expr().to_string();
						let encoded: Vec<u8> = encode(&ObservableControlResponse::Occupied);
						match query.reply(&key, encoded).wait() {
							Ok(()) => {},
							Err(err) => error!("failed to reply with {err}"),
						};
					} else {
						// start a computation
						// create Message from payload
						let content = query.payload().map_or_else(
							|| {
								let content: Vec<u8> = Vec::new();
								content
							},
							|value| {
								let content: Vec<u8> = value.to_bytes().into_owned();
								content
							},
						);
						let msg = Message::new(content);
						let ctx_clone = ctx.clone();
						let res = control_callback.lock().await(ctx_clone, msg).await;
						match res {
							Ok(response) => {
								if matches!(response, ObservableControlResponse::Accepted ) {
									// create feedback publisher
									let mut fp = feedback_publisher.lock().await;
									session
										.declare_publisher(publisher_selector.clone())
										.congestion_control(CongestionControl::Block)
										.priority(Priority::RealTime)
										.wait()
										.map_or_else(
											|err| error!("could not create feedback publisher due to {err}"),
											|publ| { fp.replace(publ); }
										);


									// spawn execution
									let tx_clone = tx.clone();
									let execution_function_clone = execution_function.clone();
									let ctx_clone = ctx.clone();
									execution_handle.lock().await.replace(tokio::spawn( async move {
										let res = execution_function_clone.lock().await(ctx_clone).await.unwrap_or_else(|_| {
											error!("execution function failed");
											todo!()
										});
										if !matches!(tx_clone.send(res).await, Ok(())) { error!("failed to send back execution result") };
									}));

									// start feedback timer
									feedback_timer.set(tokio::time::sleep(feedback_interval));
									is_running = true;
								}
								// send  response back to requestor
								let encoded: Vec<u8> = encode(&response);
								match query.reply(&key, encoded).wait() {
									Ok(()) => {},
									Err(err) => error!("failed to reply with {err}"),
								};
							}
							Err(error) => error!("control callback failed with {error}"),
						}
					}
				} else if p == "cancel" {
					// received cancel => abort a running execution
					if is_running {
						is_running = false;
						let publisher = feedback_publisher.lock().await.take();
						let handle = execution_handle.lock().await.take();
						if let Some(h) = handle {
							h.abort();
							// wait for abortion
							let _ = h.await;
							let Ok(msg) = feedback_callback.lock().await(ctx).await else {
								error!("feedback callback failed during cancel");
								todo!()
							};
							let response =
								ObservableResponse::Canceled(msg.value().clone());
							if let Some(p) = publisher {
								match p.put(Message::encode(&response).value().clone()).wait() {
									Ok(()) => {},
									Err(err) => error!("could not send cancel state due to {err}"),
								};
							} else {
								error!("missing publisher");
							};
						} else {
							error!("unexpected absence of execution handle");
						};
					}
					// acknowledge cancel request
					let encoded: Vec<u8> = encode(&ObservableControlResponse::Canceled);
					match query.reply(&key, encoded).wait() {
						Ok(()) => {},
						Err(err) => error!("failed to reply with {err}"),
					};
				} else {
					error!("observable got unknown parameter: {p}");
				}
			}

			// request finished => send back result of request (which may be a failure)
			Some(result) = rx.recv() => {
				if is_running {
					is_running = false;
					execution_handle.lock().await.take();
					let response = ObservableResponse::Finished(result.value().clone());
					feedback_publisher.lock().await.take().map_or_else(
						|| error!("could not publish result"),
						|p| {
							match p.put(Message::encode(&response).value()).wait() {
								Ok(()) => {},
								Err(err) => error!("publishing result failed due to {err}"),
							};
						}
					);
				}
			}

			// feedback timer expired and observable still is executing
			() = &mut feedback_timer, if is_running => {
				let Ok(msg) = feedback_callback.lock().await(ctx).await else {
					error!("calling feedback callback failed");
					todo!()
				};
				let response =
					ObservableResponse::Feedback(msg.value().clone());

				let lock = feedback_publisher.lock().await;
				let publisher = lock.as_ref().map_or_else(
					|| {
						error!("calling feedback publisher failed");
						todo!()
					},
					|p| p
				);
				match publisher.put(Message::encode(&response).value().clone()).wait() {
					Ok(()) => {},
					Err(err) => error!("publishing feedback failed due to {err}"),
				};

				// restart timer
				feedback_timer.set(tokio::time::sleep(feedback_interval));
			}
		}
	}
}
// endregion:	--- functions
