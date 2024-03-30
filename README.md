# Dead Letter Queue (DLQ) Implementation with RabbitMQ in Rust

This repository contains an implementation of a Dead Letter Queue (DLQ) using Rust programming language and RabbitMQ messaging broker. DLQs are crucial components in messaging systems, providing a mechanism for handling messages that cannot be delivered successfully to their intended recipients.

## Introduction

A Dead Letter Queue (DLQ) is a specialized queue used in messaging systems to capture messages that, for various reasons, cannot be processed successfully and delivered to their intended destination. These undeliverable messages are routed to the DLQ for further analysis and potential action.

## Why Use a Dead Letter Queue?

DLQs serve several important purposes:

- **Error Handling**: DLQs provide a mechanism for handling message processing errors gracefully, ensuring that failed messages are not lost but are instead stored for later analysis and potential reprocessing.
- **Diagnostic Tool**: DLQs offer valuable insights into the health and behavior of a messaging system, allowing developers to identify patterns of failure and diagnose issues.
- **Fallback Mechanism**: DLQs act as a fallback mechanism for messages that encounter repeated processing failures, ensuring that these messages are not discarded outright but are instead redirected to the DLQ for further action.

## Prerequisites

Before running the application, make sure you have the following installed:

- Docker
- Rust (if you want to modify or compile the Rust code)

## Getting Started

1. Clone this repository to your local machine:

    ```bash
    git clone git@github.com:semicolon-10/dead-letter-queue.git
    ```

2. Navigate to the repository directory:

    ```bash
    cd dead-letter-queue
    ```

3. Start the Rabbit MQ Container

    ```bash
    docker run -d --name rabbitmq -p 5672:5672 rabbitmq:latest
    ```
    
4. Run App:

    ```bash
    cargo run
    ```

# Subscribe to my youtube Channel 🎥

[Semicolon](https://www.youtube.com/@Semicolon10)
