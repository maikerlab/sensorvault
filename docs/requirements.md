# Requirements

## Summary

- [ ] [SHA-001 Collecting and persisting sensor data](#sha-001-collecting-and-persisting-sensor-data)
- [ ] [SHA-002 Evaluate sensor data](#sha-002-evaluate-sensor-data)
- [ ] [SHA-003 Manage devices and sensors](#sha-003-manage-devices-and-sensors)

## SHA-001 Collecting and persisting sensor data

The system SHALL be able to collect measurements from various sources and save them.
The datasource where sensor data is stored SHALL be made accessible for 3rd party systems, so the data can be aggregated and evaluated.

As a minimum requirement, MQTT messages with raw numeric value in payload SHALL be supported.

The system SHALL be easily extensible with adapters for all kinds of sensor data sources, e.g.:

- MQTT messages sent by a Zibee2MQTT bridge
- Messages sent via OPC-UA protocol
- Sensor data read out via a serial interface of a device
- Measurements sent as a JSON payload via REST API
- Manual measurements, entered with shell commands

## SHA-002 Evaluate sensor data

The system SHALL be able to evaluate new sensor data and tell the user how to interpret the data.

### Example: Laser degradation of a LiDAR sensor

1. Signal strength at calibration step during production: `100mV` → saved by **collector**
2. Customer returned it to the manufacturer after 1 week of usage
3. Sensor is without errors, but needs to be recalibrated before sold again → Signal strength: `95mV` → saved by **collector**
4. "Predictive maintenance step" after calibration:
    - The SensorHealthAnalyzer takes the current signal strength (95mV), the current age and total operating hours of the device and evaluates it
    - For past data of sensors of this type, it was found that the drop of signal strength (5%) just during 1 week of usage is unusually high
    - This information is output to the user with the recommendation to replace the laser module and recalibrate the LiDAR sensor

The system SHALL be extensible, so new ML models and rules can be defined for various sensor types.

## SHA-003 Manage devices and sensors

A user of the system SHALL be able to manage sensors and devices.
It SHALL be possible to create, update and delete devices and link them to sensors (one-to-many relationship).
