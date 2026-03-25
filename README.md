# Sensor Health Analyzer

A system to collect data from all your sensors into a single "source of truth", enabling valuable insights and
predictive maintenance.

## System Architecture

![](docs/arch-step2.drawio.svg)

### Sensor Layer

- In a real-world scenario we have multiple sensors, each located in a different area, e.g. inside a machine/device or "
  stand-alone"
- Each sensor could have a different network interface (e.g. WiFi, Ethernet, LoRaWAN), speak a different communication
  protocol, or send data via different data formats
- Challenge: We want to save the measurements of all sensors into a single datasource, so we can visualize, analyze and
  also make predictions based on that data
- The goal is to support the most common networking protocols and data structures, so we can integrate as much sensors
  as possible
- As a first step, only MQTT is supported with plain numeric values sent in the payload
- The system is enabled to be easily extended in the future to support more protocols - just implement a new adapter
- To test the functionality of the system, without the need to integrate a lot of physical sensors, a simulator is part
  of this project, which continuously sends MQTT messages to the collector (see [mqtt_sim](/mqtt_sim) crate)

### SensorHealthAnalyzer

- The **collector** service receives measurements from sensors and saves them into a database
- All data received by sensors is saved in a [TimescaleDB](https://github.com/timescale/timescaledb)
- Grafana can be used to aggregate and visualize the sensor data on a Dashboard

## Project Structure

```shell
.
├── docs                # Requirements and Documentation 
├── core                # Core data structures
├── proto               # Protobuf definitions for communication between microservices
├── infra               # Traits + implementation of all interaction with external services, such as databases
├── collector           # Receives measurements from MQTT broker and saves them to database
├── evaluator           # Handling evaluation requests via gRPC and returns predictions made by a ML model (in ONNX format)
├── mqtt_sim            # CLI application for integration testing - e.g. publish random measurements to the collector
├── mosquitto           # Config for the Mosquitto MQTT broker running in Docker
├── Cargo.toml          # Cargo manifest for this workspace
└── docker-compose.yml  # All services to run the whole system locally in Docker containers
```

## Get started

### Run services

Run all services in background, needed for the binaries:

```shell
docker-compose up -d
```

This will run:

- `mqtt_broker`: Mosquitto MQTT Broker, running at port 1883
- `collector` subscribes to messages at topic `sensors/+/+` (wildcards mean "sensor_type/id")
- `mqtt_sim` simulates an MQTT sensor and publishes messages to the broker
- `db`: TimescaleDB database, running at port 5432 - Required for collector to save the sensor measurements
- `grafana`: For data aggregation, visualization and observability - UI accessible
  at [localhost:3000](http://localhost:3000/)

### Binaries

Run `collector` to receive sensor measurements via MQTT and save them in the database:

```shell
cargo run -p collector
```

### Testing

For simplicity, you can just run the `mqtt_sim` binary with the `loop` command, which continuously sends random values
to the collector:

```shell
cargo run -p mqtt_sim loop temperature
```

...or send a custom, single message to a topic:

```shell
cargo run -p mqtt_sim send sensors/temperature/1 23.5
```

Of course, you can also use your favorite MQTT client (like Eclipse Mosquitto):

```shell
mosquitto_pub -h localhost -p 1883 -t /sensors/temperature/1 -m "23.5"
```
