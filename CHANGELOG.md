# Changelog

All notable changes to this project will be documented in this file.
## [0.1.1] - 2026-06-16

### Fixed
- Ci release

### Build
- Bump ghcr.io/devcontainers/features/node
## [0.1.0] - 2026-06-12

### Added
- Initial commit
- Implement serving web page
- Add deployment
- Add tini as a cheat because I can't ctrl+c
- Implement ctrl+c handling
- Use only one port for GET and websocket
- Cleanup main.rs by moving stuff to server.rs
- Implement saving commands
- Implement pipelines
- Select brokers instantly incl. broadcast
- Add deployment instructions
- Implement broker saving
- Implement editing pipelines
- Add GitHub Logo
- Use delta_t for pipeline timing
- Implement delta_t for history in messages, too
- Sanitize payload before sending to backend
- Implement button to reconnect to websocket
- MInor cleanup 🧼
- Update tree text
- Add license notive to files
- Implement status message if MQTT Broker is connected.
- Replace client by MqttBroker struct
- Send brokerstatus to new peers
- Replace SocketAddr with String
- Cap number of messages per topic at 100
- Load up to 100 messages per topic from backend
- Use CodeSnippet for selected topic for copy button
- Implementing removing brokers
- Add trim for convenience when adding new brokers
- Save commands in directory instead of a single file
- Implement removing commands and pipelines
- Add clean all rows button to pipelines
- Add guard before clearing all pipeline rows
- Implement saveguards when overriding pipelines or commands
- Move dialogs into dialogs directory 🧼
- Fix potential block, rename mqtt::Map to mqtt::BrokerMap
- Move submodules into server module
- Move logic from server to websocket module
- Move logic from server to broker_peer_bridge
- Move logic from server to broker_peer_bridge
- Refactor to use borrowing where I can make it work
- Implement tests for config.rs
- Add test job for rust in CI
- Add some tests for most svelte function
- Add svelte testing to CI
- Reverse test and build CI order
- Add code quality checks to CI. Also cleanup CI
- Format frontend
- Format backend
- Add prettyPrint for json
- Add searchbar to topic_tree
- Add search to pipeline nextStepText
- Add vscode build tasks
- Use monaco editor for messages
- Add tagged deploy for releases
- Update publish message component alignment
- Update component scrolling behavior
- Add setup steps to system test
- Add rate history chart component for visualizing throughput and storage data
- Enhance WebSocket message handling with binary frame parsing and batching
- Randomize every message
- Add MQTT frame building and handling tests
- Enhance stress test with topic management and client roles
- Improve performance on many messages per topic
- Add total messages tracking and improve message handling across components
- Cleanup
- Refactor broker connection handling to support TLS and credentials
- Add broker authentication support and UI
- Add retain flag to MQTT messages and update related components
- Implement broker authentication and retain message feature
- Cleanup
- Enhance MQTT connection handling and improve websocket message efficiency
- Implement topic selector component and refactor topic handling in various components
- Formatting
- Implement downsampling for rate history and optimize rate tracking logic
- Enhance websocket message handling and optimize rate history processing
- Enhance publish message component and improve rate history display
- Update README
- Close #33
- Set dynamic page title for mqtt-inspector
- Keep at least one message per broker
- Handle large messages properly
- Add broker to title bar
- Make top bar responsive
- Handle blocking event by big message
- Implement reconnect delay in loop_forever and adjust incoming packet size constraints
- Implement getTopicSwitchResetState function and update message selection logic
- Copy functionality for topic and message

### Feat
- Update dev-container
- Move publish to tab
- Update message selection
- Fix layout
- Implement comparing messages
- Style diffs green and red
- Update layout to be more responsive
- Improve layout
- Store selected broker and ui in url
- Add prev, next and first buttons for messages
- Lock compare image
- Refactor WebSocket message handling and state management
- Add system tests. Try fix locks and memory issues. Improve UI
- Lazyload messages
- Remove dead code
- Enhance MQTT message handling and improve command/pipeline management

### Fix
- Add new messages to all brokers
- #24
- Formatting
- Editable text
- Update to artifact@v4
- Formatting
- Formatting
- Remove warnings about missing child elements
- Formatting
- Batch messages to keep stability in frontend

### Fixed
- Handle big payload sizes
- Update CI pipeline
- Add linter exception for monaco
- Update monaco.svelte Copyright
- Use proper workflows for pull requests
- Push master workflow
- Stop publishing reconnects
- Update Monaco on selectedCommand changed
- #28
- Formatting
- Update message when changing topic
- Formatting
- Refactor string formatting for improved readability and consistency
- Formatting
- Formatting
- Locking when we hit max_byte_size
- Formatting
- Show the line since when the messages are still available in throughput
- History calculation
- Failing test
- Formatting
- Update loop_forever to ignore payload size limit errors and continue processing
- Formatting
- Formatting
- Formatting
- Formatting

