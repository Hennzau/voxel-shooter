use eyre::Result;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

// Message types for communication
#[derive(Debug)]
enum InputEvent {
    KeyPress(char),
}

#[derive(Debug)]
enum UpdateMessage {
    InputProcessed(char),
}

#[derive(Debug)]
enum RenderMessage {
    UpdatedState(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Channels: input_handling -> update and update -> renderer
    let (input_tx, input_rx) = mpsc::channel(32);
    let (update_tx, update_rx) = mpsc::channel(32);

    // Spawn the "input_handling" loop
    let input_task = tokio::spawn(input_handling_loop(input_tx.clone()));

    // Spawn the "update" loop
    let update_task = tokio::spawn(update_loop(input_rx, update_tx.clone()));

    // Spawn the "renderer" loop
    let render_task = tokio::spawn(renderer_loop(update_rx));

    // Wait for all tasks to finish (this example won't terminate by itself)
    tokio::try_join!(input_task, update_task, render_task)?;

    Ok(())
}

// "input_handling" loop: generates input events and sends them to "update"
async fn input_handling_loop(input_tx: mpsc::Sender<InputEvent>) {
    /*loop {
        // Simulate input handling (e.g., key press)
        let key_pressed = 'a'; // In a real scenario, this would come from user input
        println!("Input Handling: Key pressed '{}'", key_pressed);

        // Send the input event to the update loop
        if input_tx
            .send(InputEvent::KeyPress(key_pressed))
            .await
            .is_err()
        {
            println!("Input Handling: Update loop is closed");
            break;
        }

        // Simulate time between inputs
        sleep(Duration::from_secs(1)).await;
    }*/
}

// "update" loop: processes input events and sends updates to the "renderer"
async fn update_loop(
    mut input_rx: mpsc::Receiver<InputEvent>,
    update_tx: mpsc::Sender<RenderMessage>,
) {
    loop {
        // Non-blocking receive using try_recv
        match input_rx.try_recv() {
            Ok(input_event) => {
                match input_event {
                    InputEvent::KeyPress(key) => {
                        println!("Update: Processing key '{}'", key);
                        let updated_state = format!("State updated with '{}'", key);

                        // Send the updated state to the renderer
                        if update_tx
                            .send(RenderMessage::UpdatedState(updated_state))
                            .await
                            .is_err()
                        {
                            println!("Update: Renderer loop is closed");
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                // No message received, continue loop without blocking
                println!("Update: No new input event");
            }
        }

        // Simulate other work being done in the update loop
        sleep(Duration::from_millis(500)).await;
    }
}

// "renderer" loop: receives updates and renders the state
async fn renderer_loop(mut update_rx: mpsc::Receiver<RenderMessage>) {
    loop {
        // Non-blocking receive using try_recv
        match update_rx.try_recv() {
            Ok(render_message) => match render_message {
                RenderMessage::UpdatedState(state) => {
                    println!("Renderer: Rendering state '{}'", state);
                }
            },
            Err(_) => {
                // No message received, continue loop without blocking
                println!("Renderer: No new update to render");
            }
        }

        // Simulate other work being done in the renderer loop
        sleep(Duration::from_millis(500)).await;
    }
}
