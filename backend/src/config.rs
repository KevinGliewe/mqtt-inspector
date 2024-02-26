use std::fs;

use futures_channel::mpsc::UnboundedSender;

use crate::jsonrpc::JsonRpcNotification;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CommandMessage {
    name: String,
    topic: String,
    payload: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PipelineEntry {
    topic: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PipelineMessage {
    name: String,
    pipeline: Vec<PipelineEntry>,
}

pub fn add_to_commands(commands_path: &String, params: serde_json::Value) {
    let mut commands: Vec<CommandMessage> =
        if let Ok(file_content) = fs::read_to_string(commands_path) {
            serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
        } else {
            eprintln!("Failed to read file {}", commands_path);
            Vec::new()
        };

    if let Ok(new_command_message) = serde_json::from_value::<CommandMessage>(params) {
        commands.push(new_command_message);
        if let Ok(content) = serde_json::to_string(&commands) {
            if let Err(_) = fs::write(commands_path, content) {
                eprintln!("Failed to save new commands file to {}", commands_path);
            }
        } else {
            eprintln!("Failed to serialize updated saved commands.");
        }
    } else {
        eprintln!("Could not deserialize new command.");
    }
}

pub fn add_to_pipelines(pipelines_path: &String, params: serde_json::Value) {
    if let Ok(new_pipeline) = serde_json::from_value::<PipelineMessage>(params) {
        let new_pipeline_path = std::format!("{}/{}.json", pipelines_path, new_pipeline.name);
        if let Some(parent_dir) = std::path::Path::new(&new_pipeline_path).parent() {
            fs::create_dir_all(parent_dir).expect("Failed to create directory path");
        }
        if let Ok(content) = serde_json::to_string(&new_pipeline) {
            if let Err(_) = fs::write(&new_pipeline_path, content) {
                eprintln!("Failed to save new commands file to {}", new_pipeline_path);
            }
        } else {
            eprintln!("Failed to serialize updated saved commands.");
        }
    } else {
        println!("Could not deserialize new pipeline.");
    }
}

pub fn send_commands(
    sender: &UnboundedSender<warp::filters::ws::Message>,
    commands_path: &String,
) -> () {
    if let Ok(commands) = fs::read_to_string(commands_path) {
        let jsonrpc = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "commands".to_string(),
            params: serde_json::json!(&commands),
        };

        if let Ok(serialized) = serde_json::to_string(&jsonrpc) {
            match sender.unbounded_send(warp::filters::ws::Message::text(serialized)) {
                Ok(_) => { /* Implement Logging */ }
                Err(err) => println!("Error sending message: {:?}", err),
            }
        } else {
            eprintln!("Failed to serialize commands jsonjpc")
        }
    } else {
        eprintln!("Failed to read commands file from {}", commands_path);
    }
}

pub fn send_pipelines(
    sender: &UnboundedSender<warp::filters::ws::Message>,
    pipelines_path: &String,
) -> () {
    if let Ok(pipelines) = fs::read_dir(pipelines_path) {
        let pipelines: Vec<PipelineMessage> = pipelines
            .filter_map(|dir_entry| {
                if let Ok(file) = dir_entry {
                    if let Ok(file_content) = fs::read_to_string(file.path()) {
                        return serde_json::from_str(&file_content).ok();
                    }
                }
                None
            })
            .collect();

        let jsonrpc = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "pipelines".to_string(),
            params: serde_json::json!(&pipelines),
        };

        if let Ok(serialized) = serde_json::to_string(&jsonrpc) {
            match sender.unbounded_send(warp::filters::ws::Message::text(serialized)) {
                Ok(_) => { /* Implement Logging */ }
                Err(err) => println!("Error sending message: {:?}", err),
            }
        } else {
            eprintln!("Failed to serialize commands jsonjpc")
        }
    }
}

pub fn send_configs(
    sender: &UnboundedSender<warp::filters::ws::Message>,
    config_path: &String,
) -> () {
    send_commands(sender, &format!("{}/commands.json", config_path));
    send_pipelines(sender, &format!("{}/pipelines", config_path));
}