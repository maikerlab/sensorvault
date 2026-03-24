INSERT INTO devices(id, material_no, serial_no, custom_id, name, device_type, location)
VALUES ('83e4eff3-25c1-4226-a940-959b46b9a8d0', '123456', '12345678', NULL, 'Washing Machine', 'washing_machine',
        'Bathroom'),
       ('4bc2397a-c3d9-4ec4-a7d9-7e9da6e777a1', NULL, NULL, 'my-stove', 'Stove+Oven', 'stove', 'Kitchen'),
       ('91bcbf1f-811c-4f73-827c-9784dfaa73a9', NULL, NULL, 'my-napoleon', 'Grill', 'grill', 'Balcony');

-- Sensors of washing machine
INSERT INTO sensors (id, device_id, channel, unit, description)
VALUES ('sensors/temperature/1', '83e4eff3-25c1-4226-a940-959b46b9a8d0', 'temperature', '°C', 'current water temperature (TEST)'),
       ('sensors/humid/1', '83e4eff3-25c1-4226-a940-959b46b9a8d0', 'humidity', '%RF', 'humidity inside drum (TEST)'),
       ('sensors/speed/1', '83e4eff3-25c1-4226-a940-959b46b9a8d0', 'speed', '1/min', 'current rotating speed (TEST)');

-- Sensors of stove+oven combo in the kitchen
INSERT INTO sensors (id, device_id, channel, unit, description)
VALUES ('sensors/temperature/2', '4bc2397a-c3d9-4ec4-a7d9-7e9da6e777a1', 'temperature', '°C', 'stove front (TEST)'),
       ('sensors/temperature/3', '4bc2397a-c3d9-4ec4-a7d9-7e9da6e777a1', 'temperature', '°C', 'stove rear (TEST)'),
       ('sensors/humid/2', '4bc2397a-c3d9-4ec4-a7d9-7e9da6e777a1', 'humidity', '%RF', 'above stove (TEST)');

-- Sensors without associated device
INSERT INTO sensors (id, device_id, channel, unit, description)
VALUES ('sensors/temperature/4', NULL, 'temperature', '°C', 'Aqara (TEST)');