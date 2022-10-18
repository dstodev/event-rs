use tokio::sync::mpsc;

enum Event {
	Echo(String),
}

async fn event_loop(mut event_receiver: mpsc::Receiver<Event>) {
	while let Some(event) = event_receiver.recv().await {  // Repeats until all senders are dropped
		match event {
			Event::Echo(message) => println!("{}", message),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tokio;

	#[tokio::test]
	async fn event_loop_echo() {
		let (sender, receiver) = mpsc::channel::<Event>(10);

		// Spawn an event handler thread
		let event_handler = tokio::spawn(async move {
			event_loop(receiver).await;
		});

		// Send an event to the handler
		tokio::spawn(async move {
			let event = Event::Echo(String::from("My message"));
			assert!(sender.send(event).await.is_ok());
			// Sender is dropped
		});

		event_handler.await.unwrap();  // Wait for the receiver to exit
	}
}
